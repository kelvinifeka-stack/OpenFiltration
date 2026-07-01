use crate::LinearSystem;

/// Generic finite-volume equation.
///
/// Represents the algebraic equation
///
/// aP * phiP = Σ(aN * phiN) + b
///
/// Every transport equation (momentum, pressure,
/// scalar, energy, turbulence...) builds upon
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

    pub fn size(&self) -> usize {
        self.system.size()
    }

    pub fn reset(&mut self) {
        self.system = LinearSystem::new(self.size());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_equation() {
        let equation = FiniteVolumeEquation::new(5);

        assert_eq!(equation.size(), 5);
    }

    #[test]
    fn reset_equation() {
        let mut equation = FiniteVolumeEquation::new(8);

        equation.reset();

        assert_eq!(equation.size(), 8);
    }
}