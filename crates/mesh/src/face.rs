use crate::Edge;

#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    edges: Vec<Edge>,
}

impl Face {
    pub fn new(edges: Vec<Edge>) -> Self {
        Self { edges }
    }

    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Edge, EdgeId, NodeId};

    #[test]
    fn create_face() {
        let edges = vec![
            Edge::new(
                EdgeId::new(0),
                NodeId::new(0),
                NodeId::new(1),
            ),
            Edge::new(
                EdgeId::new(1),
                NodeId::new(1),
                NodeId::new(2),
            ),
            Edge::new(
                EdgeId::new(2),
                NodeId::new(2),
                NodeId::new(0),
            ),
        ];

        let face = Face::new(edges);

        assert_eq!(face.edge_count(), 3);
    }
}