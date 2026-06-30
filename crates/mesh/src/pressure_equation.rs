#[derive(Debug, Clone)]
pub struct PressureEquation {
    diagonal: Vec<f64>,
    source: Vec<f64>,
}

impl PressureEquation {
    pub fn new(cell_count: usize) -> Self {
        Self {
            diagonal: vec![0.0; cell_count],
            source: vec![0.0; cell_count],
        }
    }

    pub fn len(&self) -> usize {
        self.diagonal.len()
    }

    pub fn set_diagonal(&mut self, cell: usize, value: f64) {
        self.diagonal[cell] = value;
    }

    pub fn diagonal(&self, cell: usize) -> f64 {
        self.diagonal[cell]
    }

    pub fn set_source(&mut self, cell: usize, value: f64) {
        self.source[cell] = value;
    }

    pub fn source(&self, cell: usize) -> f64 {
        self.source[cell]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_pressure_equation() {
        let mut eq = PressureEquation::new(5);

        eq.set_diagonal(2, 3.0);
        eq.set_source(2, -1.5);

        assert_eq!(eq.len(), 5);
        assert_eq!(eq.diagonal(2), 3.0);
        assert_eq!(eq.source(2), -1.5);
    }
}