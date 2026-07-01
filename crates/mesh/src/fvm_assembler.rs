use crate::LinearSystem;

#[derive(Debug, Clone)]
pub struct FvmAssembler {
    system: LinearSystem,
}

impl FvmAssembler {
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

    pub fn add_to_diagonal(
        &mut self,
        row: usize,
        value: f64,
    ) {
        self.system
            .matrix_mut()
            .add(row, row, value);
    }

    pub fn add_to_rhs(
        &mut self,
        row: usize,
        value: f64,
    ) {
        self.system.rhs_mut()[row] += value;
    }

    pub fn reset(&mut self) {
        let size = self.system.size();
        self.system = LinearSystem::new(size);
    }

    pub fn build(self) -> LinearSystem {
        self.system
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assembler_builds_linear_system() {
        let assembler = FvmAssembler::new(5);

        let system = assembler.build();

        assert_eq!(system.size(), 5);
    }

    #[test]
    fn add_diagonal() {
        let mut assembler = FvmAssembler::new(4);

        assembler.add_to_diagonal(2, 5.0);

        assert_eq!(
            assembler.system().matrix().get(2, 2),
            5.0
        );
    }

    #[test]
    fn add_rhs() {
        let mut assembler = FvmAssembler::new(4);

        assembler.add_to_rhs(1, 3.5);

        assert_eq!(
            assembler.system().rhs()[1],
            3.5
        );
    }

    #[test]
    fn reset() {
        let mut assembler = FvmAssembler::new(3);

        assembler.add_to_diagonal(0, 10.0);
        assembler.reset();

        assert_eq!(
            assembler.system().matrix().get(0, 0),
            0.0
        );
    }
}