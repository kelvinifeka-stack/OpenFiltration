use crate::{
    DiffusionAssembler,
    FvmAssembler,
    LinearSystem,
    ScalarTransportEquation,
    TransportAssembler,
};

#[derive(Debug)]
pub struct ScalarTransportAssembler {
    equation: ScalarTransportEquation,
    assembler: FvmAssembler,
    diffusion: DiffusionAssembler,
}

impl ScalarTransportAssembler {
    pub fn new(size: usize) -> Self {
        Self {
            equation: ScalarTransportEquation::new(size),
            assembler: FvmAssembler::new(size),
            diffusion: DiffusionAssembler::new(1.0),
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

        if n >= 2 {

            for i in 0..(n - 1) {

                self.diffusion.assemble_pair(
                    &mut self.assembler,
                    i,
                    i + 1,
                    1.0,
                );
            }
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

impl TransportAssembler for ScalarTransportAssembler {

    fn assemble(&mut self) {
        ScalarTransportAssembler::assemble(self);
    }

    fn reset(&mut self) {
        ScalarTransportAssembler::reset(self);
    }

    fn build(self) -> LinearSystem {
        ScalarTransportAssembler::build(self)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn assemble_scalar_transport() {

        let mut assembler =
            ScalarTransportAssembler::new(5);

        assembler.assemble();

        let system = assembler.build();

        assert_eq!(system.size(), 5);

        assert!(system.matrix().nnz() > 0);
    }

    #[test]
    fn reset_scalar_transport() {

        let mut assembler =
            ScalarTransportAssembler::new(5);

        assembler.assemble();
        assembler.reset();

        assert_eq!(
            assembler.equation().size(),
            5
        );
    }
}