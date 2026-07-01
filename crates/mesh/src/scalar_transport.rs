use crate::FiniteVolumeEquation;

#[derive(Debug)]
pub struct ScalarTransportEquation {
    equation: FiniteVolumeEquation,
}

impl ScalarTransportEquation {
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

    pub fn reset(&mut self) {
        self.equation.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_scalar_transport_equation() {
        let eq = ScalarTransportEquation::new(10);

        assert_eq!(eq.equation().system().size(), 10);
    }

    #[test]
    fn reset_equation() {
        let mut eq = ScalarTransportEquation::new(5);

        eq.reset();

        assert_eq!(eq.equation().system().size(), 5);
    }
}