use crate::Scalar;

/// A scalar field.
///
/// A scalar field assigns one scalar value to each point
/// in a computational domain.
///
/// The mesh will be introduced later.
/// For now, this struct stores only the values.
#[derive(Debug, Clone, PartialEq)]
pub struct ScalarField {
    values: Vec<Scalar>,
}

impl ScalarField {
    /// Creates a new scalar field.
    pub fn new(values: Vec<Scalar>) -> Self {
        Self { values }
    }

    /// Number of stored values.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns true if the field contains no values.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns the value at an index.
    pub fn get(&self, index: usize) -> Option<Scalar> {
        self.values.get(index).copied()
    }

    /// Immutable access to all values.
    pub fn values(&self) -> &[Scalar] {
        &self.values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_field() {
        let field = ScalarField::new(vec![
            Scalar(1.0),
            Scalar(2.0),
            Scalar(3.0),
        ]);

        assert_eq!(field.len(), 3);
    }

    #[test]
    fn access_value() {
        let field = ScalarField::new(vec![
            Scalar(10.0),
            Scalar(20.0),
        ]);

        assert_eq!(field.get(1), Some(Scalar(20.0)));
    }

    #[test]
    fn empty_field() {
        let field = ScalarField::new(vec![]);

        assert!(field.is_empty());
    }
}