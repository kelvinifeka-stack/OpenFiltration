use crate::LinearSystem;

#[derive(Debug)]
pub struct ConjugateGradient {
    max_iterations: usize,
    tolerance: f64,
}

impl ConjugateGradient {
    pub fn new(max_iterations: usize, tolerance: f64) -> Self {
        Self {
            max_iterations,
            tolerance,
        }
    }

    pub fn solve(
        &self,
        system: &LinearSystem,
    ) -> Vec<f64> {
        // Placeholder implementation.
        // The full CG algorithm will be added incrementally.
        vec![0.0; system.size()]
    }

    pub fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LinearSystem;

    #[test]
    fn create_solver() {
        let solver = ConjugateGradient::new(1000, 1e-8);

        assert_eq!(solver.max_iterations(), 1000);
        assert_eq!(solver.tolerance(), 1e-8);
    }

    #[test]
    fn solve_returns_correct_size() {
        let system = LinearSystem::new(4);

        let solver = ConjugateGradient::new(100, 1e-6);

        let solution = solver.solve(&system);

        assert_eq!(solution.len(), 4);
    }
}