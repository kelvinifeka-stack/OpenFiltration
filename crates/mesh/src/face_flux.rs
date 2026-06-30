use crate::Mesh;

#[derive(Debug, Clone)]
pub struct FaceFlux {
    values: Vec<f64>,
}

impl FaceFlux {
    pub fn new(count: usize) -> Self {
        Self {
            values: vec![0.0; count],
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn get(&self, index: usize) -> f64 {
        self.values[index]
    }

    pub fn set(
        &mut self,
        index: usize,
        value: f64,
    ) {
        self.values[index] = value;
    }

    pub fn compute(mesh: &Mesh) -> Self {
        let mut flux = Self::new(mesh.face_count());

        // Placeholder implementation.
        // Future commits will use interpolation,
        // gradients, normals and velocities.
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
    fn compute_face_flux() {
        let mesh = MeshBuilder::structured(
            3,
            3,
            1.0,
            1.0,
        );

        let flux = FaceFlux::compute(&mesh);

        assert_eq!(flux.len(), mesh.face_count());

        for i in 0..flux.len() {
            assert_eq!(flux.get(i), 0.0);
        }
    }
}