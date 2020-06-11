// Deny missing documentation! Also, cute logo.png and favicon.ico for docs.rs (ᵔᴥᵔ)
#![deny(missing_docs)]
#![warn(missing_doc_code_examples)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/mathrs/mathrs/master/logo.png")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/mathrs/mathrs/master/favicon.ico")]
#![doc(issue_tracker_base_url = "https://github.com/mathrs/mathrs/issues/")]

//! <img src="https://raw.githubusercontent.com/mathrs/mathrs/master/logo.png" width="256px" height="256px">
//!
//! <img src="https://img.shields.io/crates/v/mathrs?style=flat-square"> <img src="https://img.shields.io/crates/d/mathrs?style=flat-square">
//!
//!## About
//!
//! **Important!** This project is still in early development! All of its features implemented over the minor versions are fully working, but prone to changes.
//!
//! **mathrs** is a scientific, numeric and symbolic mathematical crate for computing with [Rust](https://rust-lang.org).

//! - **Website** ─ https://mathrs.github.io
//! - **Reference** ─ https://mathrs.github.io/reference
//! - **Documentation** ─ https://docs.rs/mathrs
//!
//! ## Features
//! - **Zero dependencies**, and **no unsafe code**.
//! - **Comprehensive** documentation with examples.
//! - **Array** and **N-Dimensional Array** computing with utilities.
//! - **2D Vector** and **3D Vector** computing.
//!
//! ## Installation
//! You can install mathrs on your project by inserting the following on your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! mathrs = "*"
//! ```

use std::ops::{Add, Div, Index, IndexMut, Mul, Not, Sub};

/// Mathematical constant, equivalent to `~0.577215`, has a precision of 64 digits.
pub const EULER_GAMMA: f64 = 0.5772156649015328606065120900824024310421593359399235988057672349;

/// A dimensionless `[y, x]` point in a two-dimensional cartesian space.
/// Has two parameters, `x`, and `y`, both are `f64`.
#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

/// You can get the `x` and `y` values of a `Point` using indexing with `[]`.
/// ```rust
/// assert_eq!(Point {x: 1.11, y: 9.99}[0], 9.99)
/// ```
impl Index<u8> for Point {
    type Output = f64;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.y,
            1 => &self.x,
            _ => panic!("Index for Point must be 0 for y or 1 for x."),
        }
    }
}

/// You can set the `x` and `y` values of a `Point` using indexing with `[]` and `=`.
/// ```rust
/// assert_eq!(Point {x: 0.0, y: 0.0}[0] = 1.0, Point {x: 0.0, y: 1.0})
/// ```
impl IndexMut<u8> for Point {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.y,
            1 => &mut self.x,
            _ => panic!("Index for Point must be 0 for y or 1 for x."),
        }
    }
}

/// You can compare two `Point` structs using `==`.
/// ```rust
/// assert_eq!(Point {x: 0.0, y: 0.0} == Point {x: 0.0, y: 0.0}, true)
/// ```
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

/// A `[y, x]` vector in a two-dimensional cartesian space.
/// Has two parameters, `x`, and `y`, both are `f64`.
#[derive(Debug, Copy, Clone)]
pub struct Vector {
    x: f64,
    y: f64,
}

/// You can compare two `Vector` structs using `==`.
/// ```rust
/// assert_eq!(Vector {x: 0.0, y: 0.0} == Vector {x: 0.0, y: 0.0}, true)
/// ```
impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

/// You can get the `x` and `y` values of a `Vector` using indexing with `[]`.
/// ```rust
/// assert_eq!(Vector {x: 1.11, y: 9.99}[0], 9.99)
/// ```
impl Index<u8> for Vector {
    type Output = f64;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.y,
            1 => &self.x,
            _ => panic!("Index for Vector must be 0 for y or 1 for x."),
        }
    }
}

/// You can set the `x` and `y` values of a `Vector` using indexing with `[]` and `=`.
/// ```rust
/// assert_eq!(Vector {x: 0.0, y: 0.0}[0] = 1.0, Vector {x: 0.0, y: 1.0})
/// ```
impl IndexMut<u8> for Vector {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.y,
            1 => &mut self.x,
            _ => panic!("Index for Vector must be 0 for y or 1 for x."),
        }
    }
}

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

/// You can get the dot product of a `Vector` with another `Vector` by using the method `dot()`.
/// ```rust
/// let v1 = Vector {x: 1.0, y: 0.5};
/// let v2 = Vector {x: 0.5, y: 1.0};
/// assert_eq!(v1.dot(v2), 3.0)
/// ```
#[allow(dead_code)]
impl Vector {
    fn dot(&self, other: Vector) -> f64 {
        (self.x + other.x) + (self.y + other.y)
    }
}

/// All of the test cases for the `structs` and `functions` of **mathrs**.
#[cfg(test)]
mod mathrs_tests {

    use super::*;

    #[test]
    fn vector_operations() {
        assert_eq!(
            Vector { x: 1.0, y: 1.0 }[0] * Vector { x: 1.0, y: 1.0 }[1],
            1.0
        );

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

        assert_eq!(
            Vector { x: 1.0, y: 0.5 }.dot(Vector { x: 0.5, y: 1.0 }),
            3.0
        )
    }
}
