pub struct UnderRelaxation;

impl UnderRelaxation {
    pub fn apply(
        old_value: f64,
        new_value: f64,
        alpha: f64,
    ) -> f64 {
        assert!((0.0..=1.0).contains(&alpha));

        alpha * new_value
            + (1.0 - alpha) * old_value
    }

    pub fn apply_field(
        old_values: &[f64],
        new_values: &mut [f64],
        alpha: f64,
    ) {
        assert_eq!(old_values.len(), new_values.len());

        for (old, new) in old_values.iter().zip(new_values.iter_mut()) {
            *new = Self::apply(
                *old,
                *new,
                alpha,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_under_relaxation() {
        let relaxed = UnderRelaxation::apply(
            10.0,
            20.0,
            0.5,
        );

        assert!((relaxed - 15.0).abs() < 1e-12);
    }

    #[test]
    fn field_under_relaxation() {
        let old = vec![1.0, 2.0, 3.0];
        let mut new = vec![3.0, 4.0, 5.0];

        UnderRelaxation::apply_field(
            &old,
            &mut new,
            0.5,
        );

        assert!((new[0] - 2.0).abs() < 1e-12);
        assert!((new[1] - 3.0).abs() < 1e-12);
        assert!((new[2] - 4.0).abs() < 1e-12);
    }
}