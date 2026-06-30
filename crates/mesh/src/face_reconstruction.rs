use crate::{
    CellField,
    tvd::TVD,
};

pub struct FaceReconstruction;

impl FaceReconstruction {
    pub fn reconstruct(
        field: &CellField,
        owner: usize,
        neighbour: usize,
    ) -> f64 {
        TVD::face_value(
            field,
            owner,
            neighbour,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reconstruct_face_value() {
        let mut field = CellField::new(2);

        field.set(0, 5.0);
        field.set(1, 9.0);

        let value = FaceReconstruction::reconstruct(
            &field,
            0,
            1,
        );

        assert!(value > 5.0);
        assert!(value < 9.0);
    }
}