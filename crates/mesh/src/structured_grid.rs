use crate::Mesh;

#[derive(Debug)]
pub struct StructuredGrid {
    nx: usize,
    ny: usize,
    mesh: Mesh,
}

impl StructuredGrid {
    pub fn new(nx: usize, ny: usize) -> Self {
        Self {
            nx,
            ny,
            mesh: Mesh::new(),
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
        let grid = StructuredGrid::new(10, 20);

        assert_eq!(grid.nx(), 10);
        assert_eq!(grid.ny(), 20);

        assert_eq!(grid.mesh().node_count(), 0);
        assert_eq!(grid.mesh().edge_count(), 0);
        assert_eq!(grid.mesh().face_count(), 0);
        assert_eq!(grid.mesh().cell_count(), 0);
    }
}