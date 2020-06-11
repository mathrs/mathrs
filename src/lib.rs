/// # MathRS
/// Scientific, numeric and symbolic mathematical crate for computing with Rust.
mod mathrs {
    use std::ops::{Add, Div, Mul, Not, Sub};

    /// Point ─ A dimensionless `[x, y]` point in a two-dimensional cartesian space.
    /// Has two parameters, `x`, and `y`, both are `f64`.
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: f64,
        y: f64,
    }

    /// Vector ─ A `[x, y]` vector in a two-dimensional cartesian space.
    /// Has two parameters, `x`, and `y`, both are `f64`.
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Vector {
        x: f64,
        y: f64,
    }

    /// Negation Operator [!]
    /// You can negate a `Vector` struct using the `!` operator.
    /// This operation will give you the **oppositve vector**. When added together, **opposite vectors** cancel each other out.
    /// ```rust
    /// assert_eq!(Vector {x: 1.0, y: 1.0}!, Vector {x: -1.0, y: -1.0})
    /// ```
    impl Not for Vector {
        type Output = Vector;

        fn not(self) -> Self::Output {
            Vector {
                x: self.x * -1.0,
                y: self.y * -1.0,
            }
        }
    }

    /// Addition Operator [+]
    /// You can add two `Vector` structs using the `+` operator.
    /// ```rust
    /// assert_eq!(Vector {x: 1.0, y: 1.0} + Vector {x: 1.0, y: 1.0}, Vector {x: 2.0, y: 2.0})
    /// ```
    impl Add for Vector {
        type Output = Vector;

        fn add(self, other: Vector) -> Self::Output {
            Vector {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    /// Subtraction Operator [-]
    /// You can subtract two `Vector` structs using the `-` operator.
    /// ```rust
    /// assert_eq!(Vector {x: 1.0, y: 1.0} - Vector {x: 1.0, y: 1.0}, Vector {x: 0.0, y: 0.0})
    /// ```
    impl Sub for Vector {
        type Output = Vector;

        fn sub(self, other: Vector) -> Self::Output {
            Vector {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    /// Multiplication Operator [*]
    /// You can multiply two `Vector` structs using the `*` operator.
    /// ```rust
    /// assert_eq!(Vector {x: 0.5, y: 0.5} * Vector {x: 2.0, y: 2.0}, Vector {x: 1.0, y: 1.0})
    /// ```
    impl Mul for Vector {
        type Output = Vector;

        fn mul(self, other: Vector) -> Self::Output {
            Vector {
                x: self.x * other.x,
                y: self.y * other.y,
            }
        }
    }

    /// Scalar Multiplication Operator [*]
    /// You can multiply a `Vector` struct with a scalar (`f64`) using the `*` operator.
    /// ```rust
    /// assert_eq!(Vector {x: 1.0, y: 1.0} * 2.0, Vector {x: 2.0, y: 2.0})
    /// ```
    impl Mul<f64> for Vector {
        type Output = Vector;

        fn mul(self, scalar: f64) -> Self::Output {
            Vector {
                x: self.x * scalar,
                y: self.y * scalar,
            }
        }
    }

    /// Division Operator [/]
    /// You can divide two `Vector` structs using the `/` operator.
    /// ```rust
    /// assert_eq!(Vector {x: 9.99, y: 9.99} / Vector {x: 9.99, y: 9.99} Vector {x: 1.0, y: 1.0})
    /// ```
    impl Div for Vector {
        type Output = Vector;

        fn div(self, other: Vector) -> Self::Output {
            Vector {
                x: self.x / other.x,
                y: self.y / other.y,
            }
        }
    }

    /// Scalar Division Operator [/]
    /// You can divide a `Vector` struct with a scalar (`f64`) using the `/` operator.
    /// ```rust
    /// assert_eq!(Vector {x: 9.99, y: 9.99} / 9.99, Vector {x: 1.0, y: 1.0})
    /// ```
    impl Div<f64> for Vector {
        type Output = Vector;

        fn div(self, scalar: f64) -> Self::Output {
            Vector {
                x: self.x / scalar,
                y: self.y / scalar,
            }
        }
    }

    /// # MathRS Tests
    /// All of the test cases for the `structs` and `functions` of mathrs.
    #[cfg(test)]
    mod mathrs_tests {

        use super::*;

        #[test]
        fn point_operations() {
            assert_eq!(!Vector { x: 1.0, y: 1.0 }, Vector { x: -1.0, y: -1.0 });

            assert_eq!(
                Vector { x: 1.0, y: 1.0 } + Vector { x: 1.0, y: 1.0 },
                Vector { x: 2.0, y: 2.0 }
            );
            assert_eq!(
                Vector { x: 3.14, y: 3.14 } - Vector { x: 3.14, y: 3.14 },
                Vector { x: 0.0, y: 0.0 }
            );
            assert_eq!(
                Vector { x: 0.5, y: 0.5 } * Vector { x: 2.0, y: 2.0 },
                Vector { x: 1.0, y: 1.0 }
            );

            assert_eq!(
                Vector { x: 1.0, y: 1.0 } * 2.22,
                Vector { x: 2.22, y: 2.22 }
            );

            assert_eq!(
                Vector { x: 9.99, y: 9.99 } / Vector { x: 9.99, y: 9.99 },
                Vector { x: 1.0, y: 1.0 }
            );
        }
    }
}
