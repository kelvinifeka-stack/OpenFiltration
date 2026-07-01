#[derive(Debug, Default, Clone)]
pub struct PressureCorrector {
    assembled: bool,
    solved: bool,
    corrected: bool,
}

impl PressureCorrector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn assemble(&mut self) {
        self.assembled = true;
    }

    pub fn solve(&mut self) {
        assert!(
            self.assembled,
            "Pressure system must be assembled before solving."
        );

        self.solved = true;
    }

    pub fn apply(&mut self) {
        assert!(
            self.solved,
            "Pressure must be solved before correction."
        );

        self.corrected = true;
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn assembled(&self) -> bool {
        self.assembled
    }

    pub fn solved(&self) -> bool {
        self.solved
    }

    pub fn corrected(&self) -> bool {
        self.corrected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_pressure_workflow() {
        let mut pc = PressureCorrector::new();

        pc.assemble();
        pc.solve();
        pc.apply();

        assert!(pc.assembled());
        assert!(pc.solved());
        assert!(pc.corrected());
    }

    #[test]
    fn reset() {
        let mut pc = PressureCorrector::new();

        pc.assemble();
        pc.solve();
        pc.apply();

        pc.reset();

        assert!(!pc.assembled());
        assert!(!pc.solved());
        assert!(!pc.corrected());
    }

    #[test]
    #[should_panic]
    fn cannot_solve_without_assembly() {
        let mut pc = PressureCorrector::new();
        pc.solve();
    }

    #[test]
    #[should_panic]
    fn cannot_correct_without_solution() {
        let mut pc = PressureCorrector::new();
        pc.assemble();
        pc.apply();
    }
}