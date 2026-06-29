use crate::Face;

/// A three-dimensional topological cell.
///
/// A cell is bounded by one or more faces.
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    faces: Vec<Face>,
}

impl Cell {
    /// Creates a new cell.
    pub fn new(faces: Vec<Face>) -> Self {
        Self { faces }
    }

    /// Returns all faces.
    pub fn faces(&self) -> &[Face] {
        &self.faces
    }

    /// Number of faces.
    pub fn face_count(&self) -> usize {
        self.faces.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Edge, Face, NodeId};

    #[test]
    fn create_cell() {
        let face = Face::new(vec![
            Edge::new(NodeId::new(0), NodeId::new(1)),
            Edge::new(NodeId::new(1), NodeId::new(2)),
            Edge::new(NodeId::new(2), NodeId::new(0)),
        ]);

        let cell = Cell::new(vec![face]);

        assert_eq!(cell.face_count(), 1);
    }
}