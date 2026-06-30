use crate::{Gradient, Mesh};

pub struct GreenGauss;

impl GreenGauss {
    pub fn compute(mesh: &Mesh) -> Gradient {
        let mut gradient = Gradient::new(mesh.node_count());

        // Placeholder implementation.
        // A future commit will integrate face normals,
        // interpolation, and cell values.
        for i in 0..mesh.node_count() {
            gradient.set(i, 0.0, 0.0);
        }

        gradient
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MeshBuilder;

    #[test]
    fn compute_zero_gradient() {
        let mesh = MeshBuilder::structured(2, 2, 1.0, 1.0);

        let gradient = GreenGauss::compute(&mesh);

        assert_eq!(gradient.len(), mesh.node_count());

        for i in 0..gradient.len() {
            assert_eq!(gradient.get(i), (0.0, 0.0));
        }
    }
}