use crate::{
    FvmAssembler,
    LinearSystem,
    MomentumEquation,
};

#[derive(Debug)]
pub struct MomentumAssembler {
    equation: MomentumEquation,
    assembler: FvmAssembler,
}

impl MomentumAssembler {
    pub fn new(cell_count: usize) -> Self {
        Self {
            equation: MomentumEquation::new(cell_count),
            assembler: FvmAssembler::new(cell_count),
        }
    }

    pub fn equation(&self) -> &MomentumEquation {
        &self.equation
    }

    pub fn equation_mut(&mut self) -> &mut MomentumEquation {
        &mut self.equation
    }

    pub fn assemble(&mut self) {
        self.assembler.reset();

        for cell in 0..self.equation.len() {
            let diagonal = self.equation.diagonal(cell);
            let (sx, _) = self.equation.source(cell);

            self.assembler.add_to_diagonal(cell, diagonal);
            self.assembler.add_to_rhs(cell, sx);
        }
    }

    pub fn build(self) -> LinearSystem {
        self.assembler.build()
    }

    pub fn reset(&mut self) {
        self.equation = MomentumEquation::new(self.equation.len());
        self.assembler.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assemble_momentum_equation() {
        let mut assembler = MomentumAssembler::new(5);

        assembler.equation_mut().set_diagonal(0, 1.0);
        assembler.equation_mut().set_source(0, 2.0, 0.0);

        assembler.assemble();

        let system = assembler.build();

        assert_eq!(system.size(), 5);
    }

    #[test]
    fn reset_momentum_assembler() {
        let mut assembler = MomentumAssembler::new(4);

        assembler.equation_mut().set_diagonal(0, 5.0);

        assembler.reset();

        assert_eq!(assembler.equation().len(), 4);
        assert_eq!(assembler.equation().diagonal(0), 0.0);
    }
}