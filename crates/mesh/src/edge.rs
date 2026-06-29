use crate::NodeId;

#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    start: NodeId,
    end: NodeId,
}

impl Edge {
    pub fn new(start: NodeId, end: NodeId) -> Self {
        Self { start, end }
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
    use crate::NodeId;

    #[test]
    fn create_edge() {
        let edge = Edge::new(
            NodeId::new(0),
            NodeId::new(1),
        );

        assert_eq!(edge.start(), NodeId::new(0));
        assert_eq!(edge.end(), NodeId::new(1));
    }
}