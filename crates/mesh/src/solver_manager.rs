use crate::{
    ConjugateGradient,
    LinearSystem,
};

/// Central dispatcher for linear system solvers.
///
/// Currently wraps the Conjugate Gradient solver.
/// Future commits can extend this with BiCGSTAB,
/// GMRES and AMG.
pub struct SolverManager;

impl SolverManager {
    pub fn new() -> Self {
        Self
    }

    pub fn solve(
        &self,
        system: &LinearSystem,
    ) -> Vec<f64> {
        ConjugateGradient::solve(
            system,
            1.0e-10,
            1000,
        )
    }

    pub fn reset(&mut self) {
        // Stateless for now.
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::LinearSystem;

    #[test]
    fn create_solver_manager() {
        let _manager = SolverManager::new();
    }

    #[test]
    fn reset_solver_manager() {
        let mut manager = SolverManager::new();
        manager.reset();
    }

    #[test]
    fn manager_can_dispatch_solver() {

        let manager = SolverManager::new();

        let mut system = LinearSystem::new(3);

        system.matrix_mut().add(0,0,1.0);
        system.matrix_mut().add(1,1,1.0);
        system.matrix_mut().add(2,2,1.0);

        system.matrix_mut().finalize();

        system.rhs_mut()[0] = 1.0;
        system.rhs_mut()[1] = 2.0;
        system.rhs_mut()[2] = 3.0;

        let solution = manager.solve(&system);

        assert!((solution[0] - 1.0).abs() < 1e-10);
        assert!((solution[1] - 2.0).abs() < 1e-10);
        assert!((solution[2] - 3.0).abs() < 1e-10);
    }
}