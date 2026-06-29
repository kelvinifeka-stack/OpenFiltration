use crate::Scalar;

use std::ops::{Add, Mul, Sub};

/// A two-dimensional mathematical vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: Scalar,
    pub y: Scalar,
}

impl Vector2 {
    /// Creates a new vector.
    pub fn new(x: Scalar, y: Scalar) -> Self {
        Self { x, y }
    }

    /// Dot product.
    pub fn dot(self, rhs: Self) -> Scalar {
        self.x * rhs.x + self.y * rhs.y
    }

    /// Magnitude (length).
    pub fn magnitude(self) -> Scalar {
        self.dot(self).sqrt()
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<Scalar> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_addition() {
        let a = Vector2::new(Scalar(1.0), Scalar(2.0));
        let b = Vector2::new(Scalar(3.0), Scalar(4.0));

        assert_eq!(
            a + b,
            Vector2::new(Scalar(4.0), Scalar(6.0))
        );
    }

    #[test]
    fn vector_subtraction() {
        let a = Vector2::new(Scalar(5.0), Scalar(7.0));
        let b = Vector2::new(Scalar(2.0), Scalar(3.0));

        assert_eq!(
            a - b,
            Vector2::new(Scalar(3.0), Scalar(4.0))
        );
    }

    #[test]
    fn scalar_multiplication() {
        let v = Vector2::new(Scalar(2.0), Scalar(3.0));

        assert_eq!(
            v * Scalar(2.0),
            Vector2::new(Scalar(4.0), Scalar(6.0))
        );
    }

    #[test]
    fn dot_product() {
        let a = Vector2::new(Scalar(1.0), Scalar(2.0));
        let b = Vector2::new(Scalar(3.0), Scalar(4.0));

        assert_eq!(a.dot(b), Scalar(11.0));
    }

    #[test]
    fn magnitude() {
        let v = Vector2::new(Scalar(3.0), Scalar(4.0));

        assert_eq!(v.magnitude(), Scalar(5.0));
    }
}