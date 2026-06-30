use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SparseMatrix {
    rows: usize,
    cols: usize,

    // Assembly storage
    entries: HashMap<(usize, usize), f64>,

    // CSR storage
    finalized: bool,
    row_offsets: Vec<usize>,
    column_indices: Vec<usize>,
    values: Vec<f64>,
}

impl SparseMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            entries: HashMap::new(),
            finalized: false,
            row_offsets: Vec::new(),
            column_indices: Vec::new(),
            values: Vec::new(),
        }
    }

    /// Accumulate coefficients during assembly.
    pub fn add(
        &mut self,
        row: usize,
        col: usize,
        value: f64,
    ) {
        *self.entries.entry((row, col)).or_insert(0.0) += value;
        self.finalized = false;
    }

    /// Compatibility alias.
    pub fn insert(
        &mut self,
        row: usize,
        col: usize,
        value: f64,
    ) {
        self.add(row, col, value);
    }

    /// Convert triplet assembly into CSR.
    pub fn finalize(&mut self) {
        if self.finalized {
            return;
        }

        let mut rows: Vec<Vec<(usize, f64)>> =
            vec![Vec::new(); self.rows];

        for (&(r, c), &v) in &self.entries {
            rows[r].push((c, v));
        }

        self.row_offsets.clear();
        self.column_indices.clear();
        self.values.clear();

        self.row_offsets.push(0);

        for row in &mut rows {
            row.sort_by_key(|&(c, _)| c);

            for &(c, v) in row.iter() {
                self.column_indices.push(c);
                self.values.push(v);
            }

            self.row_offsets.push(self.values.len());
        }

        self.finalized = true;
    }

    pub fn multiply(
        &self,
        x: &[f64],
    ) -> Vec<f64> {

        assert!(
            self.finalized,
            "SparseMatrix::multiply() called before finalize()."
        );

        let mut result = vec![0.0; self.rows];

        for row in 0..self.rows {

            let start = self.row_offsets[row];
            let end = self.row_offsets[row + 1];

            for k in start..end {

                result[row] +=
                    self.values[k] *
                    x[self.column_indices[k]];
            }
        }

        result
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn nnz(&self) -> usize {
        self.entries.len()
    }

    pub fn get(
        &self,
        row: usize,
        col: usize,
    ) -> f64 {

        *self.entries
            .get(&(row, col))
            .unwrap_or(&0.0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn accumulate_entries() {

        let mut matrix = SparseMatrix::new(4,4);

        matrix.add(0,0,2.0);
        matrix.add(0,0,3.0);

        assert_eq!(matrix.get(0,0),5.0);
    }

    #[test]
    fn csr_matrix_vector_product() {

        let mut matrix = SparseMatrix::new(2,2);

        matrix.add(0,0,2.0);
        matrix.add(0,1,1.0);
        matrix.add(1,0,3.0);
        matrix.add(1,1,4.0);

        matrix.finalize();

        let x = vec![1.0, 2.0];

        let y = matrix.multiply(&x);

        assert_eq!(y[0],4.0);
        assert_eq!(y[1],11.0);
    }
}