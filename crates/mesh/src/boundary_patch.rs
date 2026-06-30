use crate::{
    BoundaryCondition,
    BoundaryType,
    EdgeId,
};

#[derive(Debug, Clone)]
pub struct BoundaryPatch {
    name: String,
    boundary_type: BoundaryType,
    condition: BoundaryCondition,
    edges: Vec<EdgeId>,
}

impl BoundaryPatch {
    pub fn new(
        name: impl Into<String>,
        boundary_type: BoundaryType,
        condition: BoundaryCondition,
    ) -> Self {
        Self {
            name: name.into(),
            boundary_type,
            condition,
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

    pub fn condition(&self) -> &BoundaryCondition {
        &self.condition
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EdgeId;

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

        match patch.condition() {
            BoundaryCondition::FixedValue(v) => {
                assert_eq!(*v, 0.0);
            }
            _ => panic!("wrong boundary condition"),
        }
    }
}