/// Returns the hyperbolic sine of the given number.
/// ```
/// use mathrs::hyperbolic::sinh;
/// use std::f64::consts::PI;
/// assert_eq!(sinh(PI / 4.0), 0.8686709614860095);
/// ```
pub fn sinh(n: f64) -> f64 {
    return n.sinh();
}

/// Returns the hyperbolic cosine of the given number.
/// ```
/// use mathrs::hyperbolic::cosh;
/// use std::f64::consts::PI;
/// assert_eq!(cosh(PI / 6.0), 1.1402383210764286);
/// ```
pub fn cosh(n: f64) -> f64 {
    return n.cosh();
}

/// Returns the hyperbolic cosine of the given number.
/// ```
/// use mathrs::hyperbolic::tanh;
/// use std::f64::consts::PI;
/// assert_eq!(tanh(PI / 8.0), 0.3736847479012153);
/// ```
pub fn tanh(n: f64) -> f64 {
    return n.tanh();
}