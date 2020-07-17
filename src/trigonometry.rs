/// Returns the sine of the given number.
/// ```
/// use mathrs::trigonometry::sin;
/// use std::f64::consts::PI;
/// assert_eq!(sin(PI / 2.0), 1.0);
/// ```
pub fn sin(n: f64) -> f64 {
    return n.sin();
}

/// Returns the cosine of the given number.
/// ```
/// use mathrs::trigonometry::cos;
/// use std::f64::consts::PI;
/// assert_eq!(cos(PI), -1.0);
/// ```
pub fn cos(n: f64) -> f64 {
    return n.cos();
}

/// Returns the tangent of the given number.
/// ```
/// use mathrs::trigonometry::tan;
/// use std::f64::consts::PI;
/// assert_eq!(tan(PI / 3.0), 1.7320508075688767);
/// ```
pub fn tan(n: f64) -> f64 {
    return n.tan();
}

/// Returns the arcsine of the given number.
/// ```
/// use mathrs::trigonometry::arcsin;
/// use std::f64::consts::PI;
/// assert_eq!(arcsin(PI / 8.0), 0.40356460692486534);
/// ```
pub fn arcsin(n: f64) -> f64 {
    return n.asin();
}

/// Returns the arccosine of the given number.
/// ```
/// use mathrs::trigonometry::arccos;
/// use std::f64::consts::PI;
/// assert_eq!(arccos(PI / 6.0), 1.0197267436954502);
/// ```
pub fn arccos(n: f64) -> f64 {
    return n.acos();
}

/// Returns the arctangent of the given number.
/// ```
/// use mathrs::trigonometry::arctan;
/// use std::f64::consts::PI;
/// assert_eq!(arctan(PI / 6.0), 0.48234790710102493);
/// ```
pub fn arctan(n: f64) -> f64 {
    return n.atan();
}