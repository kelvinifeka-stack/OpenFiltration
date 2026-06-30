use crate::{
    CellField,
    muscl::Muscl,
};

pub struct TVD;

impl TVD {
    pub fn face_value(
        field: &CellField,
        owner: usize,
        neighbour: usize,
    ) -> f64 {
        Muscl::reconstruct(
            field,
            owner,
            neighbour,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CellField;

    #[test]
    fn tvd_face_value() {
        let mut field = CellField::new(2);

        field.set(0, 2.0);
        field.set(1, 4.0);

        let value = TVD::face_value(
            &field,
            0,
            1,
        );

        assert!(value > 2.0);
        assert!(value < 4.0);
    }
}