#[derive(Debug, Clone)]
pub struct SimpleState {
    iteration: usize,
    converged: bool,
    residual: f64,
}

impl Default for SimpleState {
    fn default() -> Self {
        Self {
            iteration: 0,
            converged: false,
            residual: f64::MAX,
        }
    }
}

impl SimpleState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iteration(&self) -> usize {
        self.iteration
    }

    pub fn residual(&self) -> f64 {
        self.residual
    }

    pub fn converged(&self) -> bool {
        self.converged
    }

    pub fn next_iteration(&mut self) {
        self.iteration += 1;
    }

    pub fn set_residual(&mut self, residual: f64) {
        self.residual = residual;
    }

    pub fn set_converged(&mut self, converged: bool) {
        self.converged = converged;
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_advances() {
        let mut state = SimpleState::new();

        assert_eq!(state.iteration(), 0);

        state.next_iteration();

        assert_eq!(state.iteration(), 1);
    }

    #[test]
    fn state_records_residual() {
        let mut state = SimpleState::new();

        state.set_residual(1e-6);

        assert!((state.residual() - 1e-6).abs() < 1e-12);
    }

    #[test]
    fn state_converges() {
        let mut state = SimpleState::new();

        state.set_converged(true);

        assert!(state.converged());
    }

    #[test]
    fn reset_restores_defaults() {
        let mut state = SimpleState::new();

        state.next_iteration();
        state.set_residual(0.1);
        state.set_converged(true);

        state.reset();

        assert_eq!(state.iteration(), 0);
        assert!(!state.converged());
        assert_eq!(state.residual(), f64::MAX);
    }
}