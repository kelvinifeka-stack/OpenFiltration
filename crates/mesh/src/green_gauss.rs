use crate::{
    Gradient,
    Mesh,
};

pub struct GreenGauss;

impl GreenGauss {
    pub fn compute(mesh: &Mesh) -> Gradient {

        let mut gradient =
            Gradient::new(mesh.cell_count());

        for cell in 0..mesh.cell_count() {

            // Placeholder for actual Green-Gauss
            gradient.set(
                cell,
                0.0,
                0.0,
            );
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

        let mesh =
            MeshBuilder::structured(
                4,
                4,
                1.0,
                1.0,
            );

        let gradient =
            GreenGauss::compute(&mesh);

        assert_eq!(
            gradient.len(),
            mesh.cell_count()
        );

        for i in 0..gradient.len() {

            assert_eq!(
                gradient.get(i),
                (0.0, 0.0)
            );
        }
    }
}