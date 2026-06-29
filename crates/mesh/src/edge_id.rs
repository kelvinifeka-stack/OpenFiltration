#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeId(pub usize);

impl EdgeId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_edge_id() {
        let id = EdgeId::new(7);

        assert_eq!(id.value(), 7);
    }
}