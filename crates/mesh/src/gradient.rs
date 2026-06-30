#[derive(Debug, Clone)]
pub struct Gradient {
    values: Vec<(f64, f64)>,
}

impl Gradient {
    pub fn new(cell_count: usize) -> Self {
        Self {
            values: vec![(0.0, 0.0); cell_count],
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn set(
        &mut self,
        cell: usize,
        gx: f64,
        gy: f64,
    ) {
        self.values[cell] = (gx, gy);
    }

    pub fn get(
        &self,
        cell: usize,
    ) -> (f64, f64) {
        self.values[cell]
    }

    pub fn values(&self) -> &[(f64, f64)] {
        &self.values
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_gradient() {

        let gradient = Gradient::new(10);

        assert_eq!(gradient.len(), 10);
        assert!(!gradient.is_empty());
    }

    #[test]
    fn set_gradient() {

        let mut gradient = Gradient::new(3);

        gradient.set(1, 2.0, -1.5);

        assert_eq!(
            gradient.get(1),
            (2.0, -1.5)
        );
    }
}