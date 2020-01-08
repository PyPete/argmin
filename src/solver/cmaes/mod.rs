

/// Covariance Matrix Adaption Evolutionary Strategy (CMA-ES)
///
///
/// # References:
/// 
#[derive(Clone, Serialize, Deserialize)]
pub struct cmaes {
    /// required relative accuracy
    tol: f64,
    /// left or right boundary of current interval
    a: f64,
    /// currently proposed best guess
    b: f64,
    /// left or right boundary of current interval
    c: f64,
    /// helper variable
    d: f64,
    /// another helper variable
    e: f64,
    /// function value at `a`
    fa: f64,
    /// function value at `b`
    fb: f64,
    /// function value at `c`
    fc: f64,
}