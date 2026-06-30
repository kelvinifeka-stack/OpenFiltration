#[derive(Debug, Clone)]
pub struct NonOrthogonalCorrection {
    enabled: bool,
    correction_factor: f64,
}

impl NonOrthogonalCorrection {
    pub fn new() -> Self {
        Self {
            enabled: true,
            correction_factor: 1.0,
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn correction_factor(&self) -> f64 {
        self.correction_factor
    }

    pub fn apply(&self, orthogonal_flux: f64) -> f64 {
        if self.enabled {
            orthogonal_flux * self.correction_factor
        } else {
            orthogonal_flux
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_correction() {
        let correction = NonOrthogonalCorrection::new();

        assert_eq!(correction.apply(5.0), 5.0);
    }
}