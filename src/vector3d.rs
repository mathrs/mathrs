use std::ops::{Add, Div, Index, IndexMut, Mul, Not, Sub};

/// A `[z, y, x]` vector in a two-dimensional cartesian space.
/// Has three parameters, `z`, `y`, and `x`, all are `f64`.
#[derive(Debug, Copy, Clone)]
pub struct Vector3D {
    /// `x` coordinate of the `Vector`.
    pub x: f64,

    /// `y` coordinate of the `Vector`.
    pub y: f64,

    /// `y` coordinate of the `Vector`.
    pub z: f64,
}

impl PartialEq for Vector3D {
    /// You can compare two `Vector3D` structs using `==`.
    /// ```rust
    /// use mathrs::Vector3D;
    /// assert_eq!(Vector3D {x: 0.0, y: 0.0, z: 0.0} == Vector3D {x: 0.0, y: 0.0, z: 0.0}, true)
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Index<u8> for Vector3D {
    type Output = f64;

    /// You can get the `z`, `y`, and `x` values of a `Vector3D` using indexing with `[]`.
    /// ```rust
    /// use mathrs::Vector3D;
    /// assert_eq!(Vector3D {x: 1.11, y: 2.22, z: 3.33}[0], 3.33)
    /// ```
    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.z,
            1 => &self.y,
            2 => &self.x,
            _ => panic!("Index for Vector3D must be 0 for z, 1 for y or 2 for x."),
        }
    }
}

impl IndexMut<u8> for Vector3D {
    /// You can set the `z`, `y`, and `x` values of a `Vector3D` using indexing with `[]` and `=`.
    /// ```rust
    /// use mathrs::Vector3D;
    /// let mut v1 = Vector3D {x: 0.0, y: 0.0, z: 0.0};
    /// v1[0] = 1.0;
    /// assert_eq!(v1, Vector3D {x: 0.0, y: 0.0, z: 1.0})
    /// ```
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.z,
            1 => &mut self.y,
            2 => &mut self.x,
            _ => panic!("Index for Vector3D must be 0 for z, 1 for y or 2 for x."),
        }
    }
}

impl Not for Vector3D {
    type Output = Vector3D;

    /// You can negate a `Vector3D` struct using the `!` operator.
    /// This operation will give you the **oppositve vector**. When added together, **opposite vectors** cancel each other out.
    /// ```rust
    /// use mathrs::Vector3D;
    /// assert_eq!(!Vector3D {x: 1.0, y: 1.0, z: 1.0}, Vector3D {x: -1.0, y: -1.0, z: -1.0})
    /// ```
    fn not(self) -> Self::Output {
        Vector3D {
            x: self.x * -1.0,
            y: self.y * -1.0,
            z: self.z * -1.0
        }
    }
}

impl Add for Vector3D {
    type Output = Vector3D;

    /// You can add two `Vector3D` structs using the `+` operator.
    /// ```rust
    /// use mathrs::Vector3D;
    /// assert_eq!(Vector3D {x: 1.0, y: 1.0, z: 1.0} + Vector3D {x: 1.0, y: 1.0, z: 1.0}, Vector3D {x: 2.0, y: 2.0, z: 2.0})
    /// ```
    fn add(self, other: Vector3D) -> Self::Output {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;

    /// You can subtract two `Vector3D` structs using the `-` operator.
    /// ```rust
    /// use mathrs::Vector3D;
    /// assert_eq!(Vector3D {x: 1.0, y: 1.0, z: 1.0} - Vector3D {x: 1.0, y: 1.0, z: 1.0}, Vector3D {x: 0.0, y: 0.0, z: 0.0})
    /// ```
    fn sub(self, other: Vector3D) -> Self::Output {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vector3D {
    type Output = Vector3D;

    /// You can multiply two `Vector3D` structs using the `*` operator.
    /// ```rust
    /// use mathrs::Vector3D;
    /// assert_eq!(Vector3D {x: 0.5, y: 0.5, z: 0.5} * Vector3D {x: 2.0, y: 2.0, z: 2.0}, Vector3D {x: 1.0, y: 1.0, z: 1.0})
    /// ```
    fn mul(self, other: Vector3D) -> Self::Output {
        Vector3D {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vector3D {
    type Output = Vector3D;

    /// You can multiply a `Vector3D` struct with a scalar (`f64`) using the `*` operator.
    /// ```rust
    /// use mathrs::Vector3D;
    /// assert_eq!(Vector3D {x: 1.0, y: 1.0, z: 1.0} * 2.0, Vector3D {x: 2.0, y: 2.0, z: 2.0})
    /// ```
    fn mul(self, scalar: f64) -> Self::Output {
        Vector3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div for Vector3D {
    type Output = Vector3D;

    /// You can divide two `Vector3D` structs using the `/` operator.
    /// ```rust
    /// use mathrs::Vector3D;
    /// assert_eq!(Vector3D {x: 9.99, y: 9.99, z: 9.99} / Vector3D {x: 9.99, y: 9.99, z: 9.99}, Vector3D {x: 1.0, y: 1.0, z: 1.0})
    /// ```
    fn div(self, other: Vector3D) -> Self::Output {
        Vector3D {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Div<f64> for Vector3D {
    type Output = Vector3D;

    /// You can divide a `Vector` struct with a scalar (`f64`) using the `/` operator.
    /// ```rust
    /// use mathrs::Vector3D;
    /// assert_eq!(Vector3D {x: 9.99, y: 9.99, z: 9.99} / 9.99, Vector3D {x: 1.0, y: 1.0, z: 1.0})
    /// ```
    fn div(self, scalar: f64) -> Self::Output {
        Vector3D {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

#[allow(dead_code)]
impl Vector3D {
    /// You can get the dot product of a `Vector3D` with another `Vector3D` by using the method `dot()`.
    /// ```rust
    /// use mathrs::Vector3D;
    /// let v1 = Vector3D {x: 1.0, y: 0.5, z: 1.5};
    /// let v2 = Vector3D {x: 1.5, y: 1.0, z: 0.5};
    /// assert_eq!(v1.dot(v2), 2.75)
    /// ```
    pub fn dot(self, other: Vector3D) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    /// You can get the cross product of a `Vector3D` with another `Vector3D` by using the method `cross()`.
    /// ```rust
    /// use mathrs::Vector3D;
    /// let v1 = Vector3D {x: 1.0, y: 0.5, z: 1.5};
    /// let v2 = Vector3D {x: 0.5, y: 2.0, z: 1.0};
    /// assert_eq!(v1.cross(v2), Vector3D {x: -2.5, y: -0.25, z: 1.75})
    /// ```
    pub fn cross(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: (-1.0 * other.y) * self.z + self.y * other.z, // -y2 z1 + y1 z2
            y: other.x * self.z - self.x * other.z,          //  x2 z1 - x1 z2
            z: (-1.0 * other.x) * self.y + self.x * other.y, // -x2 y1 + x1 y2
        }
    }
}
