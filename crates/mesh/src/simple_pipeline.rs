#[derive(Debug, Clone, Default)]
pub struct SimplePipeline {
    steps: Vec<&'static str>,
}

impl SimplePipeline {
    pub fn new() -> Self {
        Self {
            steps: vec![
                "Assemble momentum equations",
                "Solve momentum equations",
                "Assemble pressure correction",
                "Solve pressure correction",
                "Correct pressure",
                "Correct velocity",
                "Update face fluxes",
                "Compute residuals",
            ],
        }
    }

    pub fn steps(&self) -> &[&'static str] {
        &self.steps
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_contains_expected_steps() {
        let pipeline = SimplePipeline::new();

        assert_eq!(pipeline.len(), 8);

        assert_eq!(
            pipeline.steps()[0],
            "Assemble momentum equations"
        );

        assert_eq!(
            pipeline.steps()[7],
            "Compute residuals"
        );
    }
}