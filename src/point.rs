use std::ops::{Index, IndexMut};

/// A dimensionless `[y, x]` point in a two-dimensional cartesian space.
/// Has two parameters, `x`, and `y`, both are `f64`.
#[derive(Debug, Copy, Clone)]
pub struct Point {
    /// `x` coordinate of `Point`.
    pub x: f64,
    /// `y` coordinate of `Point`.
    pub y: f64,
}

impl Index<u8> for Point {
    type Output = f64;

    /// You can get the `x` and `y` values of a `Point` using indexing with `[]`.
    /// ```rust
    /// use mathrs::Point;
    /// assert_eq!(Point {x: 1.11, y: 9.99}[0], 9.99)
    /// ```
    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.y,
            1 => &self.x,
            _ => panic!("Index for Point must be 0 for y or 1 for x."),
        }
    }
}

impl IndexMut<u8> for Point {
    /// You can set the `x` and `y` values of a `Point` using indexing with `[]` and `=`.
    /// ```rust
    /// use mathrs::Point;
    /// let mut p1 = Point {x: 0.0, y: 0.0};
    /// p1[0] = 1.0;
    /// assert_eq!(p1, Point {x: 0.0, y: 1.0})
    /// ```
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.y,
            1 => &mut self.x,
            _ => panic!("Index for Point must be 0 for y or 1 for x."),
        }
    }
}

impl PartialEq for Point {
    /// You can compare two `Point` structs using `==`.
    /// ```rust
    /// use mathrs::Point;
    /// assert_eq!(Point {x: 0.0, y: 0.0} == Point {x: 0.0, y: 0.0}, true)
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}