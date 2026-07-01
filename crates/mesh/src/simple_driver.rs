use crate::{
    SimpleLoop,
    SolverControls,
};

#[derive(Debug)]
pub struct SimpleDriver {
    loop_solver: SimpleLoop,
    controls: SolverControls,
}

impl SimpleDriver {
    pub fn new(controls: SolverControls) -> Self {
        Self {
            loop_solver: SimpleLoop::new(),
            controls,
        }
    }

    pub fn run(&mut self) {
        for _ in 0..self.controls.max_iterations {
            self.loop_solver.iterate();
        }
    }

    pub fn reset(&mut self) {
        self.loop_solver.reset();
    }

    pub fn iterations(&self) -> usize {
        self.loop_solver.state().iteration()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn driver_runs_iterations() {
        let mut controls = SolverControls::default();
        controls.max_iterations = 5;

        let mut driver = SimpleDriver::new(controls);

        driver.run();

        assert_eq!(driver.iterations(), 5);
    }

    #[test]
    fn driver_reset() {
        let mut controls = SolverControls::default();
        controls.max_iterations = 3;

        let mut driver = SimpleDriver::new(controls);

        driver.run();
        driver.reset();

        assert_eq!(driver.iterations(), 0);
    }
}