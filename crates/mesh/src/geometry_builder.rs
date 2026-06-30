use math::{Point2, Scalar};

use crate::{
    FaceGeometry,
    Geometry,
    Mesh,
};

pub struct GeometryBuilder;

impl GeometryBuilder {
    pub fn build(mesh: &Mesh) -> Geometry {

        let mut geometry = Geometry::new();

        for node in mesh.nodes() {
            geometry.add_node_position(*node.position());
        }

        geometry
    }

    pub fn build_face_geometry(
        mesh: &Mesh,
    ) -> Vec<FaceGeometry> {

        let mut faces = Vec::new();

        for _ in 0..mesh.face_count() {

            faces.push(
                FaceGeometry::new(
                    Point2::new(
                        Scalar(0.0),
                        Scalar(0.0),
                    ),
                    Scalar(1.0),
                    (
                        Scalar(1.0),
                        Scalar(0.0),
                    ),
                )
            );
        }

        faces
    }
}