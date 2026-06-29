/// Unique identifier for a node in a mesh.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

impl NodeId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn value(self) -> usize {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node_id() {
        let id = NodeId::new(7);
        assert_eq!(id.value(), 7);
    }
}