#[derive(Debug, Clone, Copy)]
pub enum InterpolationScheme {
    Upwind,
    Central,
}

pub struct Interpolation;

impl Interpolation {
    pub fn interpolate(
        left: f64,
        right: f64,
        scheme: InterpolationScheme,
    ) -> f64 {
        match scheme {
            InterpolationScheme::Upwind => left,
            InterpolationScheme::Central => {
                0.5 * (left + right)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn central_interpolation() {

        let value =
            Interpolation::interpolate(
                2.0,
                4.0,
                InterpolationScheme::Central,
            );

        assert_eq!(value, 3.0);
    }

    #[test]
    fn upwind_interpolation() {

        let value =
            Interpolation::interpolate(
                2.0,
                4.0,
                InterpolationScheme::Upwind,
            );

        assert_eq!(value, 2.0);
    }
}