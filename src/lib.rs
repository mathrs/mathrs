/// # MathRS
/// Scientific, numeric and symbolic mathematical crate for computing with Rust.
mod mathrs {
    use std::ops::{Add, Div, Mul, Sub};

    /// Point ─ A dimensionless point in a two-dimensional cartesian space.
    /// Has two parameters, `x`, and `y`, both are `f64`.
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl Sub for Point {
        type Output = Point;

        fn sub(self, other: Point) -> Point {
            Point {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    impl Mul for Point {
        type Output = Point;

        fn mul(self, other: Point) -> Point {
            Point {
                x: self.x * other.x,
                y: self.y * other.y,
            }
        }
    }

    impl Div for Point {
        type Output = Point;

        fn div(self, other: Point) -> Point {
            Point {
                x: self.x / other.x,
                y: self.y / other.y,
            }
        }
    }

    /// Point3D ─ A dimensionless point in a three-dimensional cartesian space.
    /// Has three parameters, `x`, `y`, and `z`, all are `f64`.
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point3D {
        x: f64,
        y: f64,
        z: f64,
    }

    impl Add for Point3D {
        type Output = Point3D;

        fn add(self, other: Point3D) -> Point3D {
            Point3D {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }

    impl Sub for Point3D {
        type Output = Point3D;

        fn sub(self, other: Point3D) -> Point3D {
            Point3D {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }
    }

    impl Mul for Point3D {
        type Output = Point3D;

        fn mul(self, other: Point3D) -> Point3D {
            Point3D {
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z,
            }
        }
    }

    impl Div for Point3D {
        type Output = Point3D;

        fn div(self, other: Point3D) -> Point3D {
            Point3D {
                x: self.x / other.x,
                y: self.y / other.y,
                z: self.z / other.z,
            }
        }
    }
}
