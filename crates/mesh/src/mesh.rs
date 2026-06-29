use math::{Point2, Scalar};

use crate::{Cell, Edge, Face, Node, NodeId};

pub struct Mesh {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    faces: Vec<Face>,
    cells: Vec<Cell>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            faces: Vec::new(),
            cells: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn add_face(&mut self, face: Face) {
        self.faces.push(face);
    }

    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_mesh() {
        let mut mesh = Mesh::new();

        let node = Node::new(
            NodeId::new(1),
            Point2::new(
                Scalar(0.0),
                Scalar(0.0),
            ),
        );

        mesh.add_node(node);

        assert_eq!(mesh.node_count(), 1);
        assert_eq!(mesh.edge_count(), 0);
        assert_eq!(mesh.face_count(), 0);
        assert_eq!(mesh.cell_count(), 0);
    }
}