pub struct VelocityCorrection;

impl VelocityCorrection {
    pub fn correct(
        velocity: &mut [f64],
        pressure_correction: &[f64],
        relaxation: f64,
    ) {
        assert_eq!(velocity.len(), pressure_correction.len());

        for (u, p) in velocity.iter_mut().zip(pressure_correction.iter()) {
            *u -= relaxation * p;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_velocity_correction() {
        let mut velocity = vec![2.0, 2.0, 2.0];
        let correction = vec![0.5, -0.5, 1.0];

        VelocityCorrection::correct(
            &mut velocity,
            &correction,
            0.2,
        );

        assert!((velocity[0] - 1.9).abs() < 1e-12);
        assert!((velocity[1] - 2.1).abs() < 1e-12);
        assert!((velocity[2] - 1.8).abs() < 1e-12);
    }
}