use crate::{
    LinearSystem,
    SparseMatrix,
};

pub struct FvmAssembler {
    matrix: SparseMatrix,
    rhs: Vec<f64>,
}

impl FvmAssembler {
    pub fn new(size: usize) -> Self {
        Self {
            matrix: SparseMatrix::new(size),
            rhs: vec![0.0; size],
        }
    }

    pub fn matrix(&self) -> &SparseMatrix {
        &self.matrix
    }

    pub fn rhs(&self) -> &[f64] {
        &self.rhs
    }

    pub fn rhs_mut(&mut self) -> &mut [f64] {
        &mut self.rhs
    }

    pub fn matrix_mut(&mut self) -> &mut SparseMatrix {
        &mut self.matrix
    }

    pub fn build(self) -> LinearSystem {
        LinearSystem {
            matrix: self.matrix,
            rhs: self.rhs,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn assembler_builds_linear_system() {

        let assembler = FvmAssembler::new(5);

        let system = assembler.build();

        assert_eq!(system.rhs.len(),5);
    }

}