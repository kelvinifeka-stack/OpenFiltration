use crate::Scalar;

/// A three-dimensional mathematical vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
}

impl Vector3 {
    pub fn new(x: Scalar, y: Scalar, z: Scalar) -> Self {
        Self { x, y, z }
    }
}