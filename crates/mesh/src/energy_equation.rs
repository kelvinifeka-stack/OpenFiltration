use crate::FiniteVolumeEquation;

#[derive(Debug)]
pub struct EnergyEquation {
    equation: FiniteVolumeEquation,
}

impl EnergyEquation {
    pub fn new(size: usize) -> Self {
        Self {
            equation: FiniteVolumeEquation::new(size),
        }
    }

    pub fn equation(&self) -> &FiniteVolumeEquation {
        &self.equation
    }

    pub fn equation_mut(&mut self) -> &mut FiniteVolumeEquation {
        &mut self.equation
    }

    pub fn size(&self) -> usize {
        self.equation.size()
    }

    pub fn reset(&mut self) {
        self.equation.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_energy_equation() {
        let equation = EnergyEquation::new(5);

        assert_eq!(equation.size(), 5);
    }

    #[test]
    fn reset_energy_equation() {
        let mut equation = EnergyEquation::new(8);

        equation.reset();

        assert_eq!(equation.size(), 8);
    }
}