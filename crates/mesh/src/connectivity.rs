use crate::{CellId, EdgeId, FaceId, NodeId};

#[derive(Debug, Clone)]
pub struct Connectivity {
    node_to_edges: Vec<Vec<EdgeId>>,
    node_to_faces: Vec<Vec<FaceId>>,
    node_to_cells: Vec<Vec<CellId>>,

    edge_to_faces: Vec<Vec<FaceId>>,
    edge_to_cells: Vec<Vec<CellId>>,

    face_to_cells: Vec<Vec<CellId>>,
}

impl Connectivity {
    pub fn new() -> Self {
        Self {
            node_to_edges: Vec::new(),
            node_to_faces: Vec::new(),
            node_to_cells: Vec::new(),

            edge_to_faces: Vec::new(),
            edge_to_cells: Vec::new(),

            face_to_cells: Vec::new(),
        }
    }

    pub fn resize(
        &mut self,
        node_count: usize,
        edge_count: usize,
        face_count: usize,
    ) {
        self.node_to_edges = vec![Vec::new(); node_count];
        self.node_to_faces = vec![Vec::new(); node_count];
        self.node_to_cells = vec![Vec::new(); node_count];

        self.edge_to_faces = vec![Vec::new(); edge_count];
        self.edge_to_cells = vec![Vec::new(); edge_count];

        self.face_to_cells = vec![Vec::new(); face_count];
    }

    pub fn node_edges(&self, id: NodeId) -> &[EdgeId] {
        &self.node_to_edges[id.value()]
    }

    pub fn node_faces(&self, id: NodeId) -> &[FaceId] {
        &self.node_to_faces[id.value()]
    }

    pub fn node_cells(&self, id: NodeId) -> &[CellId] {
        &self.node_to_cells[id.value()]
    }

    pub fn add_node_edge(&mut self, node: NodeId, edge: EdgeId) {
        self.node_to_edges[node.value()].push(edge);
    }

    pub fn add_node_face(&mut self, node: NodeId, face: FaceId) {
        self.node_to_faces[node.value()].push(face);
    }

    pub fn add_node_cell(&mut self, node: NodeId, cell: CellId) {
        self.node_to_cells[node.value()].push(cell);
    }

    pub fn add_edge_face(&mut self, edge: EdgeId, face: FaceId) {
        self.edge_to_faces[edge.value()].push(face);
    }

    pub fn add_edge_cell(&mut self, edge: EdgeId, cell: CellId) {
        self.edge_to_cells[edge.value()].push(cell);
    }

    pub fn add_face_cell(&mut self, face: FaceId, cell: CellId) {
        self.face_to_cells[face.value()].push(cell);
    }
}

impl Default for Connectivity {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_connectivity() {
        let mut c = Connectivity::new();

        c.resize(4, 4, 2);

        assert_eq!(c.node_edges(NodeId::new(0)).len(), 0);
        assert_eq!(c.node_faces(NodeId::new(0)).len(), 0);
        assert_eq!(c.node_cells(NodeId::new(0)).len(), 0);
    }
}