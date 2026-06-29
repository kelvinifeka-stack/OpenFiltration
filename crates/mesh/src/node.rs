use crate::NodeId;
use math::Point2;

/// A node in a computational mesh.
///
/// A node owns a geometric position but is itself
/// a topological object within the mesh.
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    id: NodeId,
    position: Point2,
}

impl Node {
    pub fn new(id: NodeId, position: Point2) -> Self {
        Self { id, position }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn position(&self) -> Point2 {
        self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NodeId;
    use math::{Point2, Scalar};

    #[test]
    fn create_node() {
        let node = Node::new(
            NodeId::new(42),
            Point2::new(
                Scalar::new(1.0),
                Scalar::new(2.0),
            ),
        );

        assert_eq!(node.id(), NodeId::new(42));
    }
}