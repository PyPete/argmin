

/// Covariance Matrix Adaption Evolutionary Strategy (CMA-ES)
///
/// Many continuous domain evolutionary algorithms use a normal distribution
/// to sample new search points. CMA-ES is an algorithm with a multi-variate
/// normal search distribution
///
/// # References:

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use argmin_core::ArgminOp;
use std;
use std::default::Default;
use std::f64;

#[derive(Clone, Serialize, Deserialize)]
pub struct CMAES<O>
where
    O: ArgminOp<Output = f64>,
    <O as ArgminOp>::Param: Position,
{
    /// number of generations
    tol: u32,
    /// backward time horizon for the evolution path (default: -1)
    cc: f64,
    /// correction for small variance loss (default: -1)
    cs: f64,
    /// learning rate for the rank-one update of the covariance
    /// matrix (default: -1)
    c1: f64,
    /// learning rate for the rank-(TODO: mean symbol) update of the covariance
    /// matrix (default: -1)
    cmu: f64,
    /// initial step-size (default: 0.5)
    sigma0: f64,
    /// ftol - stopping criteria on the parameter values (default: 1E-6)
    ftol: f64,
    /// xtol - stopping criteria on the function (default: 1E-6)
    xtol: f64,
    /// when true the adapted parameters are not reset between successive
    /// calls to the evolve method (default:false)
    memory: bool,
    /// Box bounds enforced (default:false)
    force_bounds: bool,
    // seed for random number generator
    seed: u32
}