use crate::SparseMatrix;

#[derive(Debug, Clone)]
pub struct LinearSystem {
    matrix: SparseMatrix,
    rhs: Vec<f64>,
}

impl LinearSystem {
    pub fn new(size: usize) -> Self {
        Self {
            matrix: SparseMatrix::new(size, size),
            rhs: vec![0.0; size],
        }
    }

    pub fn matrix(&self) -> &SparseMatrix {
        &self.matrix
    }

    pub fn matrix_mut(&mut self) -> &mut SparseMatrix {
        &mut self.matrix
    }

    pub fn rhs(&self) -> &[f64] {
        &self.rhs
    }

    pub fn rhs_mut(&mut self) -> &mut [f64] {
        &mut self.rhs
    }

    pub fn size(&self) -> usize {
        self.rhs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_linear_system() {
        let system = LinearSystem::new(5);

        assert_eq!(system.size(), 5);
        assert_eq!(system.matrix().rows(), 5);
        assert_eq!(system.matrix().cols(), 5);
        assert_eq!(system.rhs().len(), 5);
    }
}