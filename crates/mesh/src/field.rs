#[derive(Debug, Clone)]
pub struct Field<T> {
    values: Vec<T>,
}

impl<T: Clone> Field<T> {
    pub fn new(size: usize, value: T) -> Self {
        Self {
            values: vec![value; size],
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn get(&self, index: usize) -> &T {
        &self.values[index]
    }

    pub fn set(&mut self, index: usize, value: T) {
        self.values[index] = value;
    }

    pub fn values(&self) -> &[T] {
        &self.values
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_field() {

        let mut field = Field::new(5, 0.0);

        assert_eq!(field.len(), 5);

        field.set(2, 8.0);

        assert_eq!(*field.get(2), 8.0);
    }
}