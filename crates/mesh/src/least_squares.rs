use crate::VectorField;

/// Least-squares gradient reconstruction.
///
/// This is the first production-oriented implementation.
/// For now it computes a consistent finite-difference style
/// gradient over a one-dimensional stencil. Later commits
/// will extend this to arbitrary polyhedral meshes.
pub struct LeastSquares;

impl LeastSquares {
    pub fn reconstruct(field: &[f64]) -> VectorField {
        let n = field.len();

        let mut gradient = VectorField::new(n);

        if n == 0 {
            return gradient;
        }

        if n == 1 {
            gradient.set(0, 0.0, 0.0);
            return gradient;
        }

        for i in 0..n {
            let gx = if i == 0 {
                field[1] - field[0]
            } else if i == n - 1 {
                field[n - 1] - field[n - 2]
            } else {
                (field[i + 1] - field[i - 1]) * 0.5
            };

            gradient.set(i, gx, 0.0);
        }

        gradient
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn reconstruct_linear_gradient() {

        let field = vec![0.0, 1.0, 2.0, 3.0, 4.0];

        let gradient = LeastSquares::reconstruct(&field);

        for i in 0..field.len() {
            let (gx, gy) = gradient.get(i);

            assert!((gx - 1.0).abs() < 1e-12);
            assert_eq!(gy, 0.0);
        }
    }

    #[test]
    fn reconstruct_constant_field() {

        let field = vec![5.0; 6];

        let gradient = LeastSquares::reconstruct(&field);

        for i in 0..6 {
            assert_eq!(
                gradient.get(i),
                (0.0, 0.0),
            );
        }
    }

    #[test]
    fn empty_field() {

        let gradient = LeastSquares::reconstruct(&[]);

        assert_eq!(gradient.len(), 0);
    }
}