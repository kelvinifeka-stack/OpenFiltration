use crate::{
    CoefficientCache,
    FvmAssembler,
};

pub struct MatrixAssemblyEngine {
    assembler: FvmAssembler,
}

impl MatrixAssemblyEngine {

    pub fn new(size: usize) -> Self {
        Self {
            assembler: FvmAssembler::new(size),
        }
    }

    pub fn assemble(
        &mut self,
        coefficients: &CoefficientCache,
    ) {

        self.assembler.reset();

        let n = coefficients.len();

        for row in 0..n {

            self.assembler.add_diagonal(
                row,
                coefficients.diagonal(row),
            );

            for &(col, value) in
                coefficients.neighbours(row)
            {
                self.assembler.add_off_diagonal(
                    row,
                    col,
                    value,
                );
            }
        }
    }

    pub fn assembler(&self) -> &FvmAssembler {
        &self.assembler
    }

    pub fn assembler_mut(&mut self) -> &mut FvmAssembler {
        &mut self.assembler
    }

    pub fn reset(&mut self) {
        self.assembler.reset();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn assemble_matrix() {

        let mut cache =
            CoefficientCache::new(2);

        cache.add_diagonal(0, 4.0);
        cache.add_diagonal(1, 5.0);

        cache.add_neighbour(
            0,
            1,
            -1.0,
        );

        cache.add_neighbour(
            1,
            0,
            -1.0,
        );

        let mut engine =
            MatrixAssemblyEngine::new(2);

        engine.assemble(&cache);

        assert_eq!(
            engine
                .assembler()
                .diagonal(0),
            4.0,
        );

        assert_eq!(
            engine
                .assembler()
                .diagonal(1),
            5.0,
        );
    }

    #[test]
    fn reset_engine() {

        let mut engine =
            MatrixAssemblyEngine::new(4);

        engine
            .assembler_mut()
            .add_diagonal(
                0,
                10.0,
            );

        engine.reset();

        assert_eq!(
            engine
                .assembler()
                .diagonal(0),
            0.0,
        );
    }
}