use crate::{Scalar, Vector2};


/// A point in two-dimensional space.
///
/// Unlike a Vector2, a Point2 represents a location.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2 {
    pub x: Scalar,
    pub y: Scalar,
}

impl Point2 {
    pub fn new(x: Scalar, y: Scalar) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add<Vector2> for Point2 {
    type Output = Self;

    fn add(self, rhs: Vector2) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Point2 {
    type Output = Vector2;

    fn sub(self, rhs: Self) -> Vector2 {
        Vector2::new(
            self.x - rhs.x,
            self.y - rhs.y,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_difference_is_vector() {
        let p1 = Point2::new(Scalar(5.0), Scalar(6.0));
        let p2 = Point2::new(Scalar(2.0), Scalar(1.0));

        assert_eq!(
            p1 - p2,
            Vector2::new(Scalar(3.0), Scalar(5.0))
        );
    }
}