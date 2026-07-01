use crate::{CellGeometry, Mesh};

#[derive(Debug, Default)]
pub struct CellGeometryCalculator;

impl CellGeometryCalculator {
    pub fn new() -> Self {
        Self
    }

    pub fn compute(
        &self,
        mesh: &Mesh,
    ) -> Vec<CellGeometry> {
        let mut geometries =
            Vec::with_capacity(mesh.cell_count());

        for _ in 0..mesh.cell_count() {
            geometries.push(
                CellGeometry::new(
                    0.0,
                    (0.0, 0.0),
                ),
            );
        }

        geometries
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::Mesh;

    #[test]
    fn compute_empty_mesh() {

        let mesh = Mesh::new();

        let calculator =
            CellGeometryCalculator::new();

        let geometry =
            calculator.compute(&mesh);

        assert_eq!(geometry.len(),0);
    }
}