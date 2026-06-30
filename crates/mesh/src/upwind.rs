pub struct Upwind;

impl Upwind {
    pub fn interpolate(
        owner: f64,
        neighbour: f64,
        face_flux: f64,
    ) -> f64 {
        if face_flux >= 0.0 {
            owner
        } else {
            neighbour
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_flux_uses_owner() {
        let value = Upwind::interpolate(
            5.0,
            10.0,
            2.0,
        );

        assert_eq!(value, 5.0);
    }

    #[test]
    fn negative_flux_uses_neighbour() {
        let value = Upwind::interpolate(
            5.0,
            10.0,
            -2.0,
        );

        assert_eq!(value, 10.0);
    }
}