use crate::LinearSystem;

pub struct PressureCorrection;

impl PressureCorrection {
    pub fn assemble(size: usize) -> LinearSystem {
        let mut system = LinearSystem::new(size);

        for i in 0..size {
            system.matrix_mut().add(i, i, 1.0);
            system.rhs_mut()[i] = 0.0;
        }

        system.matrix_mut().finalize();

        system
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assemble_pressure_system() {
        let system = PressureCorrection::assemble(12);

        assert_eq!(system.size(), 12);
        assert_eq!(system.matrix().rows(), 12);
        assert!(system.matrix().nnz() >= 12);
    }
}