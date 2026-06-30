use math::{Point2, Scalar};

use crate::{
    BoundaryPatch,
    BoundaryType,
    BoundaryCondition,
    Cell,
    Connectivity,
    Edge,
    EdgeId,
    Face,
    Field,
    Geometry,
    Node,
    NodeId,
};

#[derive(Debug)]
pub struct Mesh {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    faces: Vec<Face>,
    cells: Vec<Cell>,
    boundary_patches: Vec<BoundaryPatch>,

    connectivity: Connectivity,
    geometry: Geometry,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            faces: Vec::new(),
            cells: Vec::new(),
            boundary_patches: Vec::new(),

            connectivity: Connectivity::new(),
            geometry: Geometry::new(),
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

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    pub fn faces(&self) -> &[Face] {
        &self.faces
    }

    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn connectivity(&self) -> &Connectivity {
        &self.connectivity
    }

    pub fn connectivity_mut(&mut self) -> &mut Connectivity {
        &mut self.connectivity
    }

    pub fn geometry(&self) -> &Geometry {
        &self.geometry
    }

    pub fn geometry_mut(&mut self) -> &mut Geometry {
        &mut self.geometry
    }

    pub fn initialize_connectivity(&mut self) {
        self.connectivity.resize(
            self.node_count(),
            self.edge_count(),
            self.face_count(),
        );
    }

    pub fn add_boundary_patch(
        &mut self,
        patch: BoundaryPatch,
    ) {
        self.boundary_patches.push(patch);
    }

    pub fn boundary_patch_count(&self) -> usize {
        self.boundary_patches.len()
    }

    pub fn boundary_patches(&self) -> &[BoundaryPatch] {
        &self.boundary_patches
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

        mesh.initialize_connectivity();

        assert_eq!(mesh.node_count(), 1);
        assert_eq!(mesh.edge_count(), 0);
        assert_eq!(mesh.face_count(), 0);
        assert_eq!(mesh.cell_count(), 0);

        assert_eq!(
            mesh.connectivity()
                .node_edges(NodeId::new(0))
                .len(),
            0,
        );

        assert_eq!(
            mesh.geometry()
                .node_positions()
                .len(),
            0,
        );
    }

    #[test]
    fn create_boundary_patch() {
        let mut patch = BoundaryPatch::new(
            "wall",
            BoundaryType::Wall,
            BoundaryCondition::FixedValue(0.0),
        );

        patch.add_edge(EdgeId::new(0));

        assert_eq!(patch.edge_count(), 1);
        assert_eq!(patch.name(), "wall");
    }
}