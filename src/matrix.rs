/// Two-dimensional `x * y` matrix.
pub struct Matrix2D {
    data: Vec<Vec<f64>>,
    shape: [usize; 2]
}

/// Two-dimensional `x * y` matrix.
pub struct Matrix3D {
    data: Vec<Vec<Vec<f64>>>,
    shape: [usize; 3]
}