use math::{Point2, Scalar};

use crate::{
    Cell,
    Edge,
    EdgeId,
    Face,
    Mesh,
    Node,
    NodeId,
};

pub struct MeshBuilder;

impl MeshBuilder {
    pub fn structured(nx: usize, ny: usize, dx: f64, dy: f64) -> Mesh {
        let mut mesh = Mesh::new();

        // ---------------- Nodes ----------------

        let mut node_id = 0;

        for j in 0..=ny {
            for i in 0..=nx {
                mesh.add_node(Node::new(
                    NodeId::new(node_id),
                    Point2::new(
                        Scalar(i as f64 * dx),
                        Scalar(j as f64 * dy),
                    ),
                ));

                node_id += 1;
            }
        }

        // ---------------- Horizontal Edges ----------------

        let mut edge_id = 0;

        for j in 0..=ny {
            for i in 0..nx {
                let a = j * (nx + 1) + i;
                let b = a + 1;

                mesh.add_edge(
                    Edge::new(
                        EdgeId::new(edge_id),
                        NodeId::new(a),
                        NodeId::new(b),
                    )
                );

                edge_id += 1;
            }
        }

        // ---------------- Vertical Edges ----------------

        for j in 0..ny {
            for i in 0..=nx {
                let a = j * (nx + 1) + i;
                let b = a + (nx + 1);

                mesh.add_edge(
                    Edge::new(
                        EdgeId::new(edge_id),
                        NodeId::new(a),
                        NodeId::new(b),
                    )
                );

                edge_id += 1;
            }
        }

        // ---------------- Faces ----------------

        for _ in 0..nx * ny {
            mesh.add_face(
                Face::new(Vec::new())
            );
        }

        // ---------------- Cells ----------------

        for _ in 0..nx * ny {
            mesh.add_cell(
                Cell::new(Vec::new())
            );
        }

        mesh
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn build_two_by_two_grid() {

        let mesh = MeshBuilder::structured(2, 2, 1.0, 1.0);

        assert_eq!(mesh.node_count(), 9);
        assert_eq!(mesh.edge_count(), 12);
        assert_eq!(mesh.face_count(), 4);
        assert_eq!(mesh.cell_count(), 4);
    }
}