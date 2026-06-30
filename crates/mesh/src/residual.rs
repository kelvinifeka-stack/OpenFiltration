pub struct Residual;

impl Residual {
    pub fn l2(residual: &[f64]) -> f64 {
        residual
            .iter()
            .map(|r| r * r)
            .sum::<f64>()
            .sqrt()
    }

    pub fn max_norm(residual: &[f64]) -> f64 {
        residual
            .iter()
            .fold(0.0_f64, |m, &r| m.max(r.abs()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_l2_norm() {
        let r = vec![3.0, 4.0];
        assert!((Residual::l2(&r) - 5.0).abs() < 1e-12);
    }

    #[test]
    fn compute_max_norm() {
        let r = vec![1.0, -7.5, 2.0];

        assert_eq!(
            Residual::max_norm(&r),
            7.5
        );
    }
}