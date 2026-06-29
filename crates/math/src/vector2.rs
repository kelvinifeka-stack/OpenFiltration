use crate::Scalar;

/// A two-dimensional mathematical vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: Scalar,
    pub y: Scalar,
}

impl Vector2 {
    pub fn new(x: Scalar, y: Scalar) -> Self {
        Self { x, y }
    }
}