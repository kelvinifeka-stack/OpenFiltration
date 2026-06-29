use crate::Face;

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    faces: Vec<Face>,
}

impl Cell {
    pub fn new(faces: Vec<Face>) -> Self {
        Self { faces }
    }

    pub fn faces(&self) -> &[Face] {
        &self.faces
    }

    pub fn face_count(&self) -> usize {
        self.faces.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Edge, EdgeId, Face, NodeId};

    #[test]
    fn create_cell() {
        let face = Face::new(vec![
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
        ]);

        let cell = Cell::new(vec![face]);

        assert_eq!(cell.face_count(), 1);
    }
}