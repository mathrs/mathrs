use std::ops::{Add, Div, Index, IndexMut, Mul, Not, Sub};

/// A `[y, x]` vector in a two-dimensional cartesian space.
/// Has two parameters, `x`, and `y`, both are `f64`.
#[derive(Debug, Copy, Clone)]
pub struct Vector {
    /// `x` coordinate of the `Vector`.
    pub x: f64,

    /// `y` coordinate of the `Vector`.
    pub y: f64,
}

impl PartialEq for Vector {
    /// You can compare two `Vector` structs using `==`.
    /// ```rust
    /// use mathrs::Vector;
    /// assert_eq!(Vector {x: 0.0, y: 0.0} == Vector {x: 0.0, y: 0.0}, true)
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Index<u8> for Vector {
    type Output = f64;

    /// You can get the `y` and `x` values of a `Vector` using indexing with `[]`.
    /// ```rust
    /// use mathrs::Vector;
    /// assert_eq!(Vector {x: 1.11, y: 9.99}[0], 9.99)
    /// ```
    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.y,
            1 => &self.x,
            _ => panic!("Index for Vector must be 0 for y or 1 for x."),
        }
    }
}

impl IndexMut<u8> for Vector {

    /// You can set the `y` and `x` values of a `Vector` using indexing with `[]` and `=`.
    /// ```rust
    /// use mathrs::Vector;
    /// let mut v1 = Vector {x: 0.0, y: 0.0};
    /// v1[0] = 1.0;
    /// assert_eq!(v1, Vector {x: 0.0, y: 1.0})
    /// ```
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.y,
            1 => &mut self.x,
            _ => panic!("Index for Vector must be 0 for y or 1 for x."),
        }
    }
}

impl Not for Vector {
    type Output = Vector;

    /// You can negate a `Vector` struct using the `!` operator.
    /// This operation will give you the **oppositve vector**. When added together, **opposite vectors** cancel each other out.
    /// ```rust
    /// use mathrs::Vector;
    /// assert_eq!(!Vector {x: 1.0, y: 1.0}, Vector {x: -1.0, y: -1.0})
    /// ```
    fn not(self) -> Self::Output {
        Vector {
            x: self.x * -1.0,
            y: self.y * -1.0,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    /// You can add two `Vector` structs using the `+` operator.
    /// ```rust
    /// use mathrs::Vector;
    /// assert_eq!(Vector {x: 1.0, y: 1.0} + Vector {x: 1.0, y: 1.0}, Vector {x: 2.0, y: 2.0})
    /// ```
    fn add(self, other: Vector) -> Self::Output {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    /// You can subtract two `Vector` structs using the `-` operator.
    /// ```rust
    /// use mathrs::Vector;
    /// assert_eq!(Vector {x: 1.0, y: 1.0} - Vector {x: 1.0, y: 1.0}, Vector {x: 0.0, y: 0.0})
    /// ```
    fn sub(self, other: Vector) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Vector {
    type Output = Vector;

    /// You can multiply two `Vector` structs using the `*` operator.
    /// ```rust
    /// use mathrs::Vector;
    /// assert_eq!(Vector {x: 0.5, y: 0.5} * Vector {x: 2.0, y: 2.0}, Vector {x: 1.0, y: 1.0})
    /// ```
    fn mul(self, other: Vector) -> Self::Output {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    /// You can multiply a `Vector` struct with a scalar (`f64`) using the `*` operator.
    /// ```rust
    /// use mathrs::Vector;
    /// assert_eq!(Vector {x: 1.0, y: 1.0} * 2.0, Vector {x: 2.0, y: 2.0})
    /// ```
    fn mul(self, scalar: f64) -> Self::Output {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Div for Vector {
    type Output = Vector;

    /// You can divide two `Vector` structs using the `/` operator.
    /// ```rust
    /// use mathrs::Vector;
    /// assert_eq!(Vector {x: 9.99, y: 9.99} / Vector {x: 9.99, y: 9.99}, Vector {x: 1.0, y: 1.0})
    /// ```
    fn div(self, other: Vector) -> Self::Output {
        Vector {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    /// You can divide a `Vector` struct with a scalar (`f64`) using the `/` operator.
    /// ```rust
    /// use mathrs::Vector;
    /// assert_eq!(Vector {x: 9.99, y: 9.99} / 9.99, Vector {x: 1.0, y: 1.0})
    /// ```
    fn div(self, scalar: f64) -> Self::Output {
        Vector {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

#[allow(dead_code)]
impl Vector {
    /// You can get the dot product of a `Vector` with another `Vector` by using the method `dot()`.
    /// ```rust
    /// use mathrs::Vector;
    /// let v1 = Vector {x: 1.0, y: 0.5};
    /// let v2 = Vector {x: 0.5, y: 1.0};
    /// assert_eq!(v1.dot(v2), 1.0)
    /// ```
    pub fn dot(self, other: Vector) -> f64 {
        (self.x * other.x) + (self.y * other.y)
    }

    /// You can get the cross product of a `Vector` by using the method `cross()`.
    /// ```rust
    /// use mathrs::Vector;
    /// let v1 = Vector {x: 1.0, y: 1.0};
    /// assert_eq!(v1.cross(), Vector {x: -1.0, y: 1.0})
    /// ```
    pub fn cross(self) -> Vector {
        Vector {
            x: self.y * -1.0,
            y: self.x,
        }
    }
}
