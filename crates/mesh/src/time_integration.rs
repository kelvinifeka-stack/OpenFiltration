pub struct ExplicitEuler;

impl ExplicitEuler {
    pub fn step(
        current: &[f64],
        residual: &[f64],
        dt: f64,
    ) -> Vec<f64> {
        assert_eq!(current.len(), residual.len());

        current
            .iter()
            .zip(residual.iter())
            .map(|(u, r)| u + dt * r)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_euler_step() {
        let u = vec![1.0, 2.0, 3.0];
        let rhs = vec![0.5, -1.0, 2.0];

        let next = ExplicitEuler::step(
            &u,
            &rhs,
            0.1,
        );

        assert!((next[0] - 1.05).abs() < 1e-12);
        assert!((next[1] - 1.90).abs() < 1e-12);
        assert!((next[2] - 3.20).abs() < 1e-12);
    }
}