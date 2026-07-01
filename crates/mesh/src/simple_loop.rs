use crate::{
    MomentumPredictor,
    PressureCorrector,
    SimpleState,
};

#[derive(Debug)]
pub struct SimpleLoop {
    predictor: MomentumPredictor,
    corrector: PressureCorrector,
    state: SimpleState,
}

impl SimpleLoop {
    pub fn new() -> Self {
        Self {
            predictor: MomentumPredictor::new(),
            corrector: PressureCorrector::new(),
            state: SimpleState::new(),
        }
    }

    pub fn iterate(&mut self) {
        self.predictor.assemble();
        self.predictor.solve();

        self.corrector.assemble();
        self.corrector.solve();
        self.corrector.apply();

        self.state.next_iteration();
    }

    pub fn state(&self) -> &SimpleState {
        &self.state
    }

    pub fn reset(&mut self) {
        self.predictor.reset();
        self.corrector.reset();
        self.state.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_iteration_advances_state() {
        let mut loop_solver = SimpleLoop::new();

        assert_eq!(loop_solver.state().iteration(), 0);

        loop_solver.iterate();

        assert_eq!(loop_solver.state().iteration(), 1);
    }

    #[test]
    fn multiple_iterations() {
        let mut loop_solver = SimpleLoop::new();

        for _ in 0..10 {
            loop_solver.iterate();
        }

        assert_eq!(loop_solver.state().iteration(), 10);
    }

    #[test]
    fn reset() {
        let mut loop_solver = SimpleLoop::new();

        loop_solver.iterate();
        loop_solver.iterate();

        loop_solver.reset();

        assert_eq!(loop_solver.state().iteration(), 0);
    }
}