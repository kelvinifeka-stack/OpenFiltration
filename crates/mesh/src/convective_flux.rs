use crate::{FaceFlux, Mesh};

pub struct ConvectiveFlux;

impl ConvectiveFlux {
    pub fn compute(mesh: &Mesh) -> FaceFlux {
        let mut flux = FaceFlux::new(mesh.face_count());

        // Placeholder implementation.
        //
        // Future implementation will evaluate:
        //
        //     F = ρ (u · S)
        //
        // together with interpolation schemes such as
        // Upwind, Central Difference and QUICK.
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
    fn compute_convective_flux() {
        let mesh = MeshBuilder::structured(
            3,
            3,
            1.0,
            1.0,
        );

        let flux = ConvectiveFlux::compute(&mesh);

        assert_eq!(flux.len(), mesh.face_count());

        for i in 0..flux.len() {
            assert_eq!(flux.get(i), 0.0);
        }
    }
}