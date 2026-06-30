use crate::{
    LinearSystem,
    Mesh,
};

pub struct Laplacian;

impl Laplacian {
    pub fn assemble(mesh: &Mesh) -> LinearSystem {
        let n = mesh.node_count();

        let mut system = LinearSystem::new(n);

        if n == 0 {
            return system;
        }

        for i in 0..n {

            if i > 0 {
                system.matrix_mut().add(i, i - 1, -1.0);
            }

            system.matrix_mut().add(i, i, 2.0);

            if i + 1 < n {
                system.matrix_mut().add(i, i + 1, -1.0);
            }

            system.rhs_mut()[i] = 0.0;
        }

        system
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::MeshBuilder;

    #[test]
    fn assemble_diffusion_matrix() {

        let mesh = MeshBuilder::structured(
            2,
            2,
            1.0,
            1.0,
        );

        let system = Laplacian::assemble(&mesh);

        assert_eq!(
            system.matrix().rows(),
            mesh.node_count(),
        );

        assert!(system.matrix().nnz() > mesh.node_count());
    }
}