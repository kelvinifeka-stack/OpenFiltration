use crate::LinearSystem;

/// Generic finite-volume equation.
///
/// Represents the algebraic equation
///
/// aP * phiP = Σ(aN * phiN) + b
///
/// Every transport equation (momentum, pressure,
/// scalar, energy, turbulence...) will build upon
/// this structure.
#[derive(Debug, Clone)]
pub struct FiniteVolumeEquation {
    system: LinearSystem,
}

impl FiniteVolumeEquation {
    pub fn new(size: usize) -> Self {
        Self {
            system: LinearSystem::new(size),
        }
    }

    pub fn system(&self) -> &LinearSystem {
        &self.system
    }

    pub fn system_mut(&mut self) -> &mut LinearSystem {
        &mut self.system
    }

    pub fn reset(&mut self) {
        let size = self.system.size();
        self.system = LinearSystem::new(size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_equation() {
        let equation = FiniteVolumeEquation::new(5);

        assert_eq!(equation.system().size(), 5);
    }

    #[test]
    fn reset_equation() {
        let mut equation = FiniteVolumeEquation::new(8);

        equation.reset();

        assert_eq!(equation.system().size(), 8);
    }
}