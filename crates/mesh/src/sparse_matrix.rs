#[derive(Debug, Clone)]
pub struct SparseMatrix {
    rows: usize,
    cols: usize,
    values: Vec<(usize, usize, f64)>,
}

impl SparseMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            values: Vec::new(),
        }
    }

    pub fn insert(
        &mut self,
        row: usize,
        col: usize,
        value: f64,
    ) {
        self.values.push((row, col, value));
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn nnz(&self) -> usize {
        self.values.len()
    }

    pub fn entries(&self) -> &[(usize, usize, f64)] {
        &self.values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_sparse_matrix() {
        let mut matrix = SparseMatrix::new(4, 4);

        matrix.insert(0, 0, 2.0);
        matrix.insert(0, 1, -1.0);

        assert_eq!(matrix.rows(), 4);
        assert_eq!(matrix.cols(), 4);
        assert_eq!(matrix.nnz(), 2);
    }
}