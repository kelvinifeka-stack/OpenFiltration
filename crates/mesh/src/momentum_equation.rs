#[derive(Debug, Clone)]
pub struct MomentumEquation {
    diagonal: Vec<f64>,
    source_x: Vec<f64>,
    source_y: Vec<f64>,
}

impl MomentumEquation {
    pub fn new(cell_count: usize) -> Self {
        Self {
            diagonal: vec![0.0; cell_count],
            source_x: vec![0.0; cell_count],
            source_y: vec![0.0; cell_count],
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

    pub fn set_source(
        &mut self,
        cell: usize,
        sx: f64,
        sy: f64,
    ) {
        self.source_x[cell] = sx;
        self.source_y[cell] = sy;
    }

    pub fn source(
        &self,
        cell: usize,
    ) -> (f64, f64) {
        (
            self.source_x[cell],
            self.source_y[cell],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_momentum_equation() {
        let mut eq = MomentumEquation::new(3);

        eq.set_diagonal(1, 2.5);
        eq.set_source(1, 4.0, -2.0);

        assert_eq!(eq.len(), 3);
        assert_eq!(eq.diagonal(1), 2.5);
        assert_eq!(eq.source(1), (4.0, -2.0));
    }
}