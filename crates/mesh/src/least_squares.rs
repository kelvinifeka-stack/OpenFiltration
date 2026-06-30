use crate::{Gradient, Mesh};

pub struct LeastSquares;

impl LeastSquares {
    pub fn compute(mesh: &Mesh) -> Gradient {
        let mut gradient = Gradient::new(mesh.node_count());

        // Placeholder implementation.
        // Future versions will solve the local least-squares
        // system using neighboring cell values.
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
    fn compute_gradient() {
        let mesh = MeshBuilder::structured(
            3,
            3,
            1.0,
            1.0,
        );

        let gradient = LeastSquares::compute(&mesh);

        assert_eq!(gradient.len(), mesh.node_count());

        for i in 0..gradient.len() {
            assert_eq!(gradient.get(i), (0.0, 0.0));
        }
    }
}