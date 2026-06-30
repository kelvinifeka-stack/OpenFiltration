use crate::VectorField;

/// Green-Gauss gradient reconstruction.
///
/// This implementation provides a simple finite-volume style
/// approximation that will later be extended to arbitrary
/// polygonal meshes using real face geometry.
pub struct GreenGauss;

impl GreenGauss {
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

            let left = if i == 0 {
                field[i]
            } else {
                field[i - 1]
            };

            let right = if i == n - 1 {
                field[i]
            } else {
                field[i + 1]
            };

            let gx = (right - left) * 0.5;

            gradient.set(i, gx, 0.0);
        }

        gradient
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn reconstruct_linear_field() {

        let field = vec![0.0, 1.0, 2.0, 3.0];

        let gradient = GreenGauss::reconstruct(&field);

        assert_eq!(gradient.len(), 4);

        for i in 1..3 {
            let (gx, gy) = gradient.get(i);

            assert!((gx - 1.0).abs() < 1e-12);
            assert_eq!(gy, 0.0);
        }
    }

    #[test]
    fn reconstruct_constant_field() {

        let field = vec![5.0; 8];

        let gradient = GreenGauss::reconstruct(&field);

        for i in 0..8 {
            assert_eq!(
                gradient.get(i),
                (0.0, 0.0),
            );
        }
    }

    #[test]
    fn empty_field() {

        let gradient = GreenGauss::reconstruct(&[]);

        assert_eq!(gradient.len(), 0);
    }
}