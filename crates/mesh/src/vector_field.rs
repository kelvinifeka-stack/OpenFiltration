#[derive(Debug, Clone)]
pub struct VectorField {
    values: Vec<(f64, f64)>,
}

impl VectorField {
    pub fn new(size: usize) -> Self {
        Self {
            values: vec![(0.0, 0.0); size],
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn get(
        &self,
        index: usize,
    ) -> (f64, f64) {
        self.values[index]
    }

    pub fn set(
        &mut self,
        index: usize,
        x: f64,
        y: f64,
    ) {
        self.values[index] = (x, y);
    }

    pub fn values(&self) -> &[(f64, f64)] {
        &self.values
    }

    pub fn values_mut(&mut self) -> &mut [(f64, f64)] {
        &mut self.values
    }

    pub fn fill(
        &mut self,
        x: f64,
        y: f64,
    ) {
        for value in &mut self.values {
            *value = (x, y);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_vector_field() {

        let mut field = VectorField::new(5);

        assert_eq!(field.len(), 5);
        assert!(!field.is_empty());

        field.set(2, 4.0, -3.0);

        assert_eq!(
            field.get(2),
            (4.0, -3.0),
        );

        field.fill(1.0, 2.0);

        for value in field.values() {
            assert_eq!(*value, (1.0, 2.0));
        }
    }
}