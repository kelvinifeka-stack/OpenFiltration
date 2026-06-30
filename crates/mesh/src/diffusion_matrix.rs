use crate::{LinearSystem, Mesh};

pub struct DiffusionMatrix;

impl DiffusionMatrix {
    pub fn assemble(mesh: &Mesh) -> LinearSystem {
        let n = mesh.node_count();

        LinearSystem::new(n)
    }
}