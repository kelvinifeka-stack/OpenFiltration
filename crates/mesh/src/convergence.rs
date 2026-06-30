#[derive(Debug, Clone)]
pub struct Convergence {
    tolerance: f64,
    max_iterations: usize,
}

impl Convergence {
    pub fn new(tolerance: f64, max_iterations: usize) -> Self {
        Self {
            tolerance,
            max_iterations,
        }
    }

    pub fn converged(&self, residual: f64) -> bool {
        residual <= self.tolerance
    }

    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    pub fn max_iterations(&self) -> usize {
        self.max_iterations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convergence_detected() {
        let c = Convergence::new(1e-6, 1000);

        assert!(c.converged(1e-7));
        assert!(!c.converged(1e-4));
        assert_eq!(c.max_iterations(), 1000);
    }
}