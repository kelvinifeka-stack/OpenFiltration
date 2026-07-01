use crate::{
    FiniteVolumeEquation,
    FvmAssembler,
    LinearSystem,
    ScalarTransportEquation,
};

#[derive(Debug)]
pub struct ScalarTransportAssembler {
    equation: ScalarTransportEquation,
    assembler: FvmAssembler,
}

impl ScalarTransportAssembler {
    pub fn new(size: usize) -> Self {
        Self {
            equation: ScalarTransportEquation::new(size),
            assembler: FvmAssembler::new(size),
        }
    }

    pub fn equation(&self) -> &ScalarTransportEquation {
        &self.equation
    }

    pub fn equation_mut(&mut self) -> &mut ScalarTransportEquation {
        &mut self.equation
    }

    pub fn assemble(&mut self) {
        self.assembler.reset();

        let n = self.equation.size();

        for i in 0..n {
            self.assembler.add_to_diagonal(i, 1.0);
            self.assembler.add_to_rhs(i, 0.0);
        }
    }

    pub fn build(self) -> LinearSystem {
        self.assembler.build()
    }

    pub fn reset(&mut self) {
        self.equation.reset();
        self.assembler.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assemble_scalar_transport() {
        let mut assembler = ScalarTransportAssembler::new(5);

        assembler.assemble();

        let system = assembler.build();

        assert_eq!(system.size(), 5);
    }

    #[test]
    fn reset_scalar_transport() {
        let mut assembler = ScalarTransportAssembler::new(5);

        assembler.assemble();
        assembler.reset();

        assert_eq!(assembler.equation().size(), 5);
    }
}