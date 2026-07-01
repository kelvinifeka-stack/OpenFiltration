use crate::SimpleStage;

#[derive(Debug, Clone)]
pub struct SimpleEngine {
    stage: Option<SimpleStage>,
}

impl SimpleEngine {
    pub fn new() -> Self {
        Self {
            stage: Some(SimpleStage::AssembleMomentum),
        }
    }

    pub fn current_stage(&self) -> Option<SimpleStage> {
        self.stage
    }

    pub fn advance(&mut self) {
        if let Some(stage) = self.stage {
            self.stage = stage.next();
        }
    }

    pub fn finished(&self) -> bool {
        self.stage.is_none()
    }

    pub fn reset(&mut self) {
        self.stage = Some(SimpleStage::AssembleMomentum);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SimpleStage;

    #[test]
    fn engine_advances_through_pipeline() {
        let mut engine = SimpleEngine::new();

        assert_eq!(
            engine.current_stage(),
            Some(SimpleStage::AssembleMomentum)
        );

        while !engine.finished() {
            engine.advance();
        }

        assert!(engine.finished());
    }

    #[test]
    fn engine_can_reset() {
        let mut engine = SimpleEngine::new();

        while !engine.finished() {
            engine.advance();
        }

        engine.reset();

        assert_eq!(
            engine.current_stage(),
            Some(SimpleStage::AssembleMomentum)
        );
    }
}