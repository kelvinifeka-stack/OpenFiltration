use crate::EdgeId;
use crate::BoundaryType;

#[derive(Debug, Clone)]
pub struct BoundaryPatch {
    name: String,
    boundary_type: BoundaryType,
    edges: Vec<EdgeId>,
}

impl BoundaryPatch {
    pub fn new(
        name: impl Into<String>,
        boundary_type: BoundaryType,
    ) -> Self {
        Self {
            name: name.into(),
            boundary_type,
            edges: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, edge: EdgeId) {
        self.edges.push(edge);
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn boundary_type(&self) -> BoundaryType {
        self.boundary_type
    }
}