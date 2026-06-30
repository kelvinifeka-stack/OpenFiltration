pub struct PoiseuilleBenchmark;

impl PoiseuilleBenchmark {
    pub fn analytical_velocity(
        y: f64,
        height: f64,
        dpdx: f64,
        viscosity: f64,
    ) -> f64 {
        -(dpdx)
            * y
            * (height - y)
            / (2.0 * viscosity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn centerline_velocity_is_maximum() {
        let h = 1.0;
        let mu = 1.0;
        let dpdx = -1.0;

        let wall = PoiseuilleBenchmark::analytical_velocity(
            0.0,
            h,
            dpdx,
            mu,
        );

        let center = PoiseuilleBenchmark::analytical_velocity(
            0.5,
            h,
            dpdx,
            mu,
        );

        assert!(center > wall);
    }
}