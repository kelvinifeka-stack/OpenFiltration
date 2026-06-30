use crate::{FaceFlux, Mesh};

pub struct DiffusionFlux;

impl DiffusionFlux {
    pub fn compute(mesh: &Mesh) -> FaceFlux {
        let mut flux = FaceFlux::new(mesh.face_count());

        // Placeholder implementation.
        //
        // Future versions will evaluate:
        //
        //     q = -Γ ∇φ · S
        //
        // using reconstructed gradients and face geometry.
        for i in 0..mesh.face_count() {
            flux.set(i, 0.0);
        }

        flux
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MeshBuilder;

    #[test]
    fn compute_diffusion_flux() {
        let mesh = MeshBuilder::structured(
            3,
            3,
            1.0,
            1.0,
        );

        let flux = DiffusionFlux::compute(&mesh);

        assert_eq!(flux.len(), mesh.face_count());

        for i in 0..flux.len() {
            assert_eq!(flux.get(i), 0.0);
        }
    }
}