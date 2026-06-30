use crate::{
    CellField,
    face_reconstruction::FaceReconstruction,
};

pub struct HighOrderConvection;

impl HighOrderConvection {
    pub fn face_flux(
        field: &CellField,
        owner: usize,
        neighbour: usize,
        mass_flux: f64,
    ) -> f64 {
        let phi = FaceReconstruction::reconstruct(
            field,
            owner,
            neighbour,
        );

        mass_flux * phi
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_high_order_face_flux() {
        let mut field = CellField::new(2);

        field.set(0, 2.0);
        field.set(1, 6.0);

        let flux = HighOrderConvection::face_flux(
            &field,
            0,
            1,
            3.0,
        );

        assert!(flux > 6.0);
        assert!(flux < 18.0);
    }
}