use crate::{
    CoefficientCache,
    ResidualEngine,
};

pub trait EquationStage {
    fn name(&self) -> &'static str;

    fn execute(
        &self,
        coefficients: &mut CoefficientCache,
        residuals: &mut ResidualEngine,
    );
}

pub struct EquationPipeline {
    stages: Vec<Box<dyn EquationStage>>,
}

impl EquationPipeline {

    pub fn new() -> Self {
        Self {
            stages: Vec::new(),
        }
    }

    pub fn add_stage<T>(
        &mut self,
        stage: T,
    )
    where
        T: EquationStage + 'static,
    {
        self.stages.push(Box::new(stage));
    }

    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }

    pub fn execute(
        &self,
        coefficients: &mut CoefficientCache,
        residuals: &mut ResidualEngine,
    ) {
        for stage in &self.stages {
            stage.execute(
                coefficients,
                residuals,
            );
        }
    }

    pub fn reset(&mut self) {
        self.stages.clear();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    struct DummyStage;

    impl EquationStage for DummyStage {

        fn name(&self) -> &'static str {
            "Dummy"
        }

        fn execute(
            &self,
            coefficients: &mut CoefficientCache,
            residuals: &mut ResidualEngine,
        ) {

            coefficients.add_diagonal(
                0,
                5.0,
            );

            residuals.add(
                0,
                2.0,
            );
        }
    }

    #[test]
    fn pipeline_executes() {

        let mut pipeline =
            EquationPipeline::new();

        pipeline.add_stage(DummyStage);

        let mut coeffs =
            CoefficientCache::new(1);

        let mut residuals =
            ResidualEngine::new(1);

        pipeline.execute(
            &mut coeffs,
            &mut residuals,
        );

        assert_eq!(
            coeffs.diagonal(0),
            5.0,
        );

        assert_eq!(
            residuals.residual(0),
            2.0,
        );
    }

    #[test]
    fn reset_pipeline() {

        let mut pipeline =
            EquationPipeline::new();

        pipeline.add_stage(DummyStage);

        assert_eq!(
            pipeline.stage_count(),
            1,
        );

        pipeline.reset();

        assert_eq!(
            pipeline.stage_count(),
            0,
        );
    }
}