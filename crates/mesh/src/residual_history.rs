#[derive(Debug, Default, Clone)]
pub struct ResidualHistory {
    values: Vec<f64>,
}

impl ResidualHistory {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
        }
    }

    pub fn push(&mut self, residual: f64) {
        self.values.push(residual);
    }

    // Alias used by solver infrastructure
    pub fn record(&mut self, residual: f64) {
        self.push(residual);
    }

    pub fn latest(&self) -> Option<f64> {
        self.values.last().copied()
    }

    pub fn values(&self) -> &[f64] {
        &self.values
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_history() {
        let mut history = ResidualHistory::new();

        history.push(1.0);
        history.push(0.5);
        history.push(0.1);

        assert_eq!(history.len(), 3);
        assert_eq!(history.latest(), Some(0.1));
    }
}