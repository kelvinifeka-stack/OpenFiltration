use math::{Point2, Scalar};

use crate::{Mesh, Node, NodeId};

#[derive(Debug)]
pub struct StructuredGrid {
    nx: usize,
    ny: usize,
    mesh: Mesh,
}

impl StructuredGrid {
    pub fn new(nx: usize, ny: usize) -> Self {
        let mut mesh = Mesh::new();

        let mut id = 0;

        for j in 0..ny {
            for i in 0..nx {
                mesh.add_node(
                    Node::new(
                        NodeId::new(id),
                        Point2::new(
                            Scalar(i as f64),
                            Scalar(j as f64),
                        ),
                    ),
                );

                id += 1;
            }
        }

        Self {
            nx,
            ny,
            mesh,
        }
    }

    pub fn nx(&self) -> usize {
        self.nx
    }

    pub fn ny(&self) -> usize {
        self.ny
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_grid() {
        let grid = StructuredGrid::new(3, 2);

        assert_eq!(grid.nx(), 3);
        assert_eq!(grid.ny(), 2);

        assert_eq!(grid.mesh().node_count(), 6);
        assert_eq!(grid.mesh().edge_count(), 0);
        assert_eq!(grid.mesh().face_count(), 0);
        assert_eq!(grid.mesh().cell_count(), 0);
    }

    #[test]
    fn node_positions_are_generated() {
        let grid = StructuredGrid::new(2, 2);

        let nodes = grid.mesh().nodes();

        assert_eq!(nodes.len(), 4);

        assert_eq!(nodes[0].id(), NodeId::new(0));
        assert_eq!(nodes[1].id(), NodeId::new(1));
        assert_eq!(nodes[2].id(), NodeId::new(2));
        assert_eq!(nodes[3].id(), NodeId::new(3));
    }
}