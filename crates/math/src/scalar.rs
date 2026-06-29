use std::ops::{Add, Div, Mul, Neg, Sub};

/// A mathematical scalar.
///
/// A scalar has magnitude but no direction.
///
/// Examples:
/// - Pressure
/// - Temperature
/// - Density
/// - Concentration
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Scalar(pub f64);

impl Scalar {
    /// Creates a new scalar.
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    /// Returns the underlying value.
    pub fn value(self) -> f64 {
        self.0
    }

    /// Square root.
    pub fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }
}

impl Add for Scalar {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Scalar(self.0 + rhs.0)
    }
}

impl Sub for Scalar {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Scalar(self.0 - rhs.0)
    }
}

impl Mul for Scalar {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Scalar(self.0 * rhs.0)
    }
}

impl Div for Scalar {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Scalar(self.0 / rhs.0)
    }
}

impl Neg for Scalar {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Scalar(-self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        assert_eq!(Scalar(2.0) + Scalar(3.0), Scalar(5.0));
    }

    #[test]
    fn subtraction() {
        assert_eq!(Scalar(5.0) - Scalar(2.0), Scalar(3.0));
    }

    #[test]
    fn multiplication() {
        assert_eq!(Scalar(2.0) * Scalar(4.0), Scalar(8.0));
    }

    #[test]
    fn division() {
        assert_eq!(Scalar(8.0) / Scalar(2.0), Scalar(4.0));
    }

    #[test]
    fn negation() {
        assert_eq!(-Scalar(5.0), Scalar(-5.0));
    }
}