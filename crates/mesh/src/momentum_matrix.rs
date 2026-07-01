use crate::{
    LinearSystem,
    Mesh,
};

#[derive(Debug)]
pub struct MomentumMatrix {
    system: LinearSystem,
}

impl MomentumMatrix {
    pub fn new(cell_count: usize) -> Self {
        Self {
            system: LinearSystem::new(cell_count),
        }
    }

    pub fn system(&self) -> &LinearSystem {
        &self.system
    }

    pub fn system_mut(&mut self) -> &mut LinearSystem {
        &mut self.system
    }

    pub fn assemble_identity(
        &mut self,
        mesh: &Mesh,
    ) {
        for cell in 0..mesh.cell_count() {
            self.system.add_diagonal(cell, 1.0);
            self.system.set_rhs(cell, 0.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StructuredGrid;

    #[test]
    fn assemble_identity_matrix() {
        let mesh = StructuredGrid::new(2, 2).build();

        let mut matrix =
            MomentumMatrix::new(mesh.cell_count());

        matrix.assemble_identity(&mesh);

        assert_eq!(
            matrix.system().matrix().rows(),
            mesh.cell_count()
        );
    }
}