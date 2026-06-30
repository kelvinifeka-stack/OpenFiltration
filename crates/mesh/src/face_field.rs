#[derive(Debug, Clone)]
pub struct FaceField {
    values: Vec<f64>,
}

impl FaceField {
    pub fn new(face_count: usize) -> Self {
        Self {
            values: vec![0.0; face_count],
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn get(&self, face: usize) -> f64 {
        self.values[face]
    }

    pub fn set(
        &mut self,
        face: usize,
        value: f64,
    ) {
        self.values[face] = value;
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
    fn create_face_field() {

        let mut field = FaceField::new(12);

        assert_eq!(field.len(), 12);
        assert!(!field.is_empty());

        field.set(5, 7.25);

        assert_eq!(
            field.get(5),
            7.25,
        );

        field.fill(2.0);

        for value in field.values() {
            assert_eq!(*value, 2.0);
        }
    }
}