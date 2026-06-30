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

    // Alias used by matrix assembly
    pub fn add(
        &mut self,
        row: usize,
        col: usize,
        value: f64,
    ) {
        for (r, c, v) in &mut self.values {
            if *r == row && *c == col {
                *v += value;
                return;
            }
        }

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

    pub fn multiply(
        &self,
        x: &[f64],
    ) -> Vec<f64> {
        assert_eq!(x.len(), self.cols);

        let mut result = vec![0.0; self.rows];

        for &(row, col, value) in &self.values {
            result[row] += value * x[col];
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_sparse_matrix() {
        let mut matrix = SparseMatrix::new(4, 4);

        matrix.add(0, 0, 2.0);
        matrix.add(0, 1, -1.0);

        assert_eq!(matrix.rows(), 4);
        assert_eq!(matrix.cols(), 4);
        assert_eq!(matrix.nnz(), 2);
    }

    #[test]
    fn add_accumulates_values() {
        let mut matrix = SparseMatrix::new(2, 2);

        matrix.add(0, 0, 2.0);
        matrix.add(0, 0, 3.5);

        assert_eq!(matrix.nnz(), 1);

        assert_eq!(matrix.entries()[0], (0, 0, 5.5));
    }

    #[test]
    fn matrix_vector_product() {

        let mut matrix = SparseMatrix::new(2,2);

        matrix.add(0,0,2.0);
        matrix.add(0,1,1.0);
        matrix.add(1,0,3.0);

        let x = vec![4.0,5.0];

        let y = matrix.multiply(&x);

        assert_eq!(y[0],13.0);
        assert_eq!(y[1],12.0);
    }
}