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
        // Placeholder.
        //
        // Future implementation:
        // 1. Solve momentum equations
        // 2. Assemble pressure correction equation
        // 3. Solve pressure correction
        // 4. Correct pressure
        // 5. Correct velocity
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
}