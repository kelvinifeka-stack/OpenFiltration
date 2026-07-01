#[derive(Debug, Default, Clone)]
pub struct MomentumPredictor {
    assembled: bool,
    solved: bool,
}

impl MomentumPredictor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn assemble(&mut self) {
        self.assembled = true;
    }

    pub fn solve(&mut self) {
        assert!(
            self.assembled,
            "Momentum system must be assembled before solving."
        );

        self.solved = true;
    }

    pub fn reset(&mut self) {
        self.assembled = false;
        self.solved = false;
    }

    pub fn assembled(&self) -> bool {
        self.assembled
    }

    pub fn solved(&self) -> bool {
        self.solved
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predictor_assembles() {
        let mut predictor = MomentumPredictor::new();

        predictor.assemble();

        assert!(predictor.assembled());
        assert!(!predictor.solved());
    }

    #[test]
    fn predictor_solves() {
        let mut predictor = MomentumPredictor::new();

        predictor.assemble();
        predictor.solve();

        assert!(predictor.solved());
    }

    #[test]
    fn predictor_resets() {
        let mut predictor = MomentumPredictor::new();

        predictor.assemble();
        predictor.solve();
        predictor.reset();

        assert!(!predictor.assembled());
        assert!(!predictor.solved());
    }

    #[test]
    #[should_panic]
    fn solve_requires_assembly() {
        let mut predictor = MomentumPredictor::new();

        predictor.solve();
    }
}