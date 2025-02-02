// Copyright 2018-2024 argmin developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[allow(unused_imports)]
use argmin::{
    core::{observers::ObserverMode, CostFunction, Error, Executor, Gradient, Hessian},
    solver::trustregion::{CauchyPoint, Dogleg, Steihaug, TrustRegion},
};
use argmin_observer_slog::SlogLogger;
use argmin_testfunctions::{rosenbrock, rosenbrock_derivative, rosenbrock_hessian};
use ndarray::{Array, Array1, Array2};

struct Rosenbrock {}

impl CostFunction for Rosenbrock {
    type Param = Array1<f64>;
    type Output = f64;

    fn cost(&self, p: &Self::Param) -> Result<Self::Output, Error> {
        Ok(rosenbrock(p.as_slice().unwrap()))
    }
}

impl Gradient for Rosenbrock {
    type Param = Array1<f64>;
    type Gradient = Array1<f64>;

    fn gradient(&self, p: &Self::Param) -> Result<Self::Gradient, Error> {
        Ok(Array1::from(rosenbrock_derivative(p.as_slice().unwrap())))
    }
}

impl Hessian for Rosenbrock {
    type Param = Array1<f64>;
    type Hessian = Array2<f64>;

    fn hessian(&self, p: &Self::Param) -> Result<Self::Hessian, Error> {
        let h = rosenbrock_hessian(p.as_slice().unwrap())
            .into_iter()
            .flatten()
            .collect();
        Ok(Array::from_shape_vec((p.len(), p.len()), h)?)
    }
}

fn run() -> Result<(), Error> {
    // Define cost function
    let cost = Rosenbrock {};

    // Define initial parameter vector
    // easy case
    // let init_param: Array1<f64> = Array1::from_vec(vec![1.2, 1.2]);
    // tough case
    let init_param: Array1<f64> = Array1::from(vec![-1.2, 1.0]);

    // Set up the subproblem
    let subproblem = Steihaug::new().with_max_iters(2);
    // let subproblem = CauchyPoint::new();
    // let subproblem = Dogleg::new();

    // Set up solver
    let solver = TrustRegion::new(subproblem);

    // Run solver
    let res = Executor::new(cost, solver)
        .configure(|state| state.param(init_param).max_iters(50))
        .add_observer(SlogLogger::term(), ObserverMode::Always)
        .run()?;

    // Print result
    println!("{res}");
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("{e}");
        std::process::exit(1);
    }
}
