use std::ops::{Add, Div, Index, IndexMut, Mul, Not, Sub};

/// A `[z, y, x]` vector in a two-dimensional cartesian space.
/// Has three parameters, `x`, `y`, and `z`, all are `f64`.
#[derive(Debug, Copy, Clone)]
pub struct Vector3D {
    /// `x` coordinate of the `Vector`.
    pub x: f64,

    /// `y` coordinate of the `Vector`.
    pub y: f64,

    /// `y` coordinate of the `Vector`.
    pub z: f64,
}

/// You can compare two `Vector3D` structs using `==`.
/// ```rust
/// use mathrs::Vector3D;
/// assert_eq!(Vector3D {x: 0.0, y: 0.0, z: 0.0} == Vector3D {x: 0.0, y: 0.0, z: 0.0}, true)
/// ```
impl PartialEq for Vector3D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

/// You can get the `x`, `y`, and `z` values of a `Vector3D` using indexing with `[]`.
/// ```rust
/// use mathrs::Vector3D;
/// assert_eq!(Vector3D {x: 1.11, y: 2.22, z: 3.33}[0], 3.33)
/// ```
impl Index<u8> for Vector3D {
    type Output = f64;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.z,
            1 => &self.y,
            2 => &self.x,
            _ => panic!("Index for Vector3D must be 0 for z, 1 for y or 2 for x."),
        }
    }
}

/// You can set the `x`, `y`, and `z` values of a `Vector3D` using indexing with `[]` and `=`.
/// ```rust
/// use mathrs::Vector3D;
/// assert_eq!(Vector3D {x: 0.0, y: 0.0, z: 0.0}[0] = 1.0, Vector3D {x: 0.0, y: 0.0, z: 1.0})
/// ```
impl IndexMut<u8> for Vector3D {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.z,
            1 => &mut self.y,
            2 => &mut self.x,
            _ => panic!("Index for Vector must be 0 for y or 1 for x."),
        }
    }
}
