/// Returns the absolute value of the given number.
/// ```
/// use mathrs::syntax_sugar::abs;
/// assert_eq!(abs(-1.234), 1.234);
/// ```
pub fn abs(n: f64) -> f64 {
    return n.abs();
}

/// Returns the result of `x` powered to `y`.
/// ```
/// use mathrs::syntax_sugar::pow;
/// assert_eq!(pow(2.0, 2.0), 4.0);
/// ```
pub fn pow(n: f64, x: f64) -> f64 {
    return n.powf(x);
}

/// Returns the rounded value of the given number.
/// ```
/// use mathrs::syntax_sugar::round;
/// assert_eq!(round(1.234), 1.0);
/// ```
pub fn round(n: f64) -> f64 {
    return n.round();
}