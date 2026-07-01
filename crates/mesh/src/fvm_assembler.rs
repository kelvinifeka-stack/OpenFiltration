use crate::{LinearSystem, SparseMatrix};

#[derive(Debug)]
pub struct FvmAssembler {
    matrix: SparseMatrix,
    rhs: Vec<f64>,
}

impl FvmAssembler {
    pub fn new(size: usize) -> Self {
        Self {
            matrix: SparseMatrix::new(size, size),
            rhs: vec![0.0; size],
        }
    }

    pub fn add_to_diagonal(
        &mut self,
        row: usize,
        value: f64,
    ) {
        self.matrix.add(row, row, value);
    }

    pub fn add_to_off_diagonal(
        &mut self,
        row: usize,
        column: usize,
        value: f64,
    ) {
        self.matrix.add(row, column, value);
    }

    pub fn add_to_rhs(
        &mut self,
        row: usize,
        value: f64,
    ) {
        self.rhs[row] += value;
    }

    pub fn reset(&mut self) {
        self.matrix = SparseMatrix::new(
            self.matrix.rows(),
            self.matrix.cols(),
        );

        self.rhs.fill(0.0);
    }

    pub fn build(mut self) -> LinearSystem {

        self.matrix.finalize();

        let mut system =
            LinearSystem::new(self.rhs.len());

        *system.matrix_mut() = self.matrix;
        system.rhs_mut().copy_from_slice(&self.rhs);

        system
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn add_diagonal() {

        let mut assembler =
            FvmAssembler::new(3);

        assembler.add_to_diagonal(1,5.0);

        let system = assembler.build();

        assert_eq!(
            system.matrix().get(1,1),
            5.0
        );
    }

    #[test]
    fn add_off_diagonal() {

        let mut assembler =
            FvmAssembler::new(3);

        assembler.add_to_off_diagonal(
            0,
            1,
            -2.0,
        );

        let system = assembler.build();

        assert_eq!(
            system.matrix().get(0,1),
            -2.0
        );
    }

    #[test]
    fn add_rhs() {

        let mut assembler =
            FvmAssembler::new(2);

        assembler.add_to_rhs(0,7.0);

        let system = assembler.build();

        assert_eq!(system.rhs()[0],7.0);
    }

    #[test]
    fn assembler_builds_linear_system() {

        let mut assembler =
            FvmAssembler::new(2);

        assembler.add_to_diagonal(0,1.0);
        assembler.add_to_diagonal(1,2.0);

        let system = assembler.build();

        assert_eq!(
            system.matrix().get(0,0),
            1.0
        );

        assert_eq!(
            system.matrix().get(1,1),
            2.0
        );
    }

    #[test]
    fn reset() {

        let mut assembler =
            FvmAssembler::new(2);

        assembler.add_to_diagonal(0,5.0);

        assembler.reset();

        let system = assembler.build();

        assert_eq!(
            system.matrix().get(0,0),
            0.0
        );
    }
}