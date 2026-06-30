use crate::LinearSystem;

pub struct SimpleSolver {
    pressure: LinearSystem,
    momentum: LinearSystem,
}

impl SimpleSolver {
    pub fn new(size: usize) -> Self {
        Self {
            pressure: LinearSystem::new(size),
            momentum: LinearSystem::new(size),
        }
    }

    pub fn pressure_system(&self) -> &LinearSystem {
        &self.pressure
    }

    pub fn pressure_system_mut(&mut self) -> &mut LinearSystem {
        &mut self.pressure
    }

    pub fn momentum_system(&self) -> &LinearSystem {
        &self.momentum
    }

    pub fn momentum_system_mut(&mut self) -> &mut LinearSystem {
        &mut self.momentum
    }

    pub fn iterate(&mut self) {
        // Future momentum solve
        self.momentum.matrix_mut().finalize();

        // Future pressure correction solve
        self.pressure.matrix_mut().finalize();

        // Placeholder for SIMPLE cycle:
        //
        // 1. Assemble momentum equations
        // 2. Solve momentum equations
        // 3. Assemble pressure correction
        // 4. Solve pressure correction
        // 5. Correct pressure
        // 6. Correct velocity
        // 7. Check residuals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_simple_solver() {
        let solver = SimpleSolver::new(25);

        assert_eq!(solver.pressure_system().size(), 25);
        assert_eq!(solver.momentum_system().size(), 25);
    }

    #[test]
    fn simple_iteration_executes() {
        let mut solver = SimpleSolver::new(10);

        solver.iterate();

        assert_eq!(solver.pressure_system().size(), 10);
        assert_eq!(solver.momentum_system().size(), 10);
    }
}