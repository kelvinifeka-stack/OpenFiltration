use crate::Edge;

/// A two-dimensional topological face.
///
/// Initially a face is simply an ordered collection
/// of edges that form a closed boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    edges: Vec<Edge>,
}

impl Face {
    /// Creates a new face.
    pub fn new(edges: Vec<Edge>) -> Self {
        Self { edges }
    }

    /// Returns all edges.
    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    /// Number of edges.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Edge, NodeId};

    #[test]
    fn create_face() {
        let edges = vec![
            Edge::new(NodeId::new(0), NodeId::new(1)),
            Edge::new(NodeId::new(1), NodeId::new(2)),
            Edge::new(NodeId::new(2), NodeId::new(0)),
        ];

        let face = Face::new(edges);

        assert_eq!(face.edge_count(), 3);
    }
}