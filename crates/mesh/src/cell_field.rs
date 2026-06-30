#[derive(Debug, Clone)]
pub struct CellField {
    values: Vec<f64>,
}

impl CellField {
    pub fn new(cell_count: usize) -> Self {
        Self {
            values: vec![0.0; cell_count],
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn get(&self, cell: usize) -> f64 {
        self.values[cell]
    }

    pub fn set(
        &mut self,
        cell: usize,
        value: f64,
    ) {
        self.values[cell] = value;
    }

    pub fn values(&self) -> &[f64] {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut [f64] {
        &mut self.values
    }

    pub fn fill(&mut self, value: f64) {
        for v in &mut self.values {
            *v = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_cell_field() {
        let mut field = CellField::new(8);

        assert_eq!(field.len(), 8);

        field.set(3, 12.5);

        assert_eq!(field.get(3), 12.5);

        field.fill(4.0);

        for v in field.values() {
            assert_eq!(*v, 4.0);
        }
    }
}