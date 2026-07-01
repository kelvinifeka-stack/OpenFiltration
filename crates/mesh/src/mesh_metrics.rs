use crate::{
    FaceGeometry,
    CellGeometry,
};

#[derive(Debug, Clone)]
pub struct MeshMetrics {
    face_geometries: Vec<FaceGeometry>,
    cell_geometries: Vec<CellGeometry>,
}

impl MeshMetrics {
    pub fn new() -> Self {
        Self {
            face_geometries: Vec::new(),
            cell_geometries: Vec::new(),
        }
    }

    pub fn add_face_geometry(
        &mut self,
        geometry: FaceGeometry,
    ) {
        self.face_geometries.push(geometry);
    }

    pub fn add_cell_geometry(
        &mut self,
        geometry: CellGeometry,
    ) {
        self.cell_geometries.push(geometry);
    }

    pub fn face_geometry(
        &self,
        index: usize,
    ) -> Option<&FaceGeometry> {
        self.face_geometries.get(index)
    }

    pub fn cell_geometry(
        &self,
        index: usize,
    ) -> Option<&CellGeometry> {
        self.cell_geometries.get(index)
    }

    pub fn face_count(&self) -> usize {
        self.face_geometries.len()
    }

    pub fn cell_count(&self) -> usize {
        self.cell_geometries.len()
    }

    pub fn reset(&mut self) {
        self.face_geometries.clear();
        self.cell_geometries.clear();
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use math::{Point2, Scalar};

    #[test]
    fn create_mesh_metrics() {

        let mut metrics = MeshMetrics::new();

        metrics.add_face_geometry(
            FaceGeometry::new(
                Point2::new(
                    Scalar(0.5),
                    Scalar(0.0),
                ),
                Scalar(1.0),
                (
                    Scalar(1.0),
                    Scalar(0.0),
                ),
            ),
        );

        metrics.add_cell_geometry(
            CellGeometry::new(
                1.0,
                (0.5,0.5),
            ),
        );

        assert_eq!(metrics.face_count(),1);
        assert_eq!(metrics.cell_count(),1);
    }

    #[test]
    fn reset_metrics() {

        let mut metrics = MeshMetrics::new();

        metrics.add_cell_geometry(
            CellGeometry::new(
                1.0,
                (0.0,0.0),
            ),
        );

        metrics.reset();

        assert_eq!(metrics.cell_count(),0);
    }
}