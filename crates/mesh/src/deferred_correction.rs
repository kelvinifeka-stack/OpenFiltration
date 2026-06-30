#[derive(Debug, Clone)]
pub struct DeferredCorrection {
    relaxation: f64,
}

impl DeferredCorrection {
    pub fn new(relaxation: f64) -> Self {
        Self { relaxation }
    }

    pub fn relaxation(&self) -> f64 {
        self.relaxation
    }

    pub fn apply(
        &self,
        previous: f64,
        corrected: f64,
    ) -> f64 {
        previous
            + self.relaxation
                * (corrected - previous)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_deferred_correction() {
        let dc = DeferredCorrection::new(0.5);

        let result = dc.apply(10.0, 14.0);

        assert_eq!(result, 12.0);
    }
}