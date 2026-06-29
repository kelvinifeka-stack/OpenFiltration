use crate::{EdgeId, NodeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    id: EdgeId,
    start: NodeId,
    end: NodeId,
}

impl Edge {
    pub fn new(
        id: EdgeId,
        start: NodeId,
        end: NodeId,
    ) -> Self {
        Self {
            id,
            start,
            end,
        }
    }

    pub fn id(&self) -> EdgeId {
        self.id
    }

    pub fn start(&self) -> NodeId {
        self.start
    }

    pub fn end(&self) -> NodeId {
        self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_edge() {
        let edge = Edge::new(
            EdgeId::new(0),
            NodeId::new(1),
            NodeId::new(2),
        );

        assert_eq!(edge.id(), EdgeId::new(0));
        assert_eq!(edge.start(), NodeId::new(1));
        assert_eq!(edge.end(), NodeId::new(2));
    }
}