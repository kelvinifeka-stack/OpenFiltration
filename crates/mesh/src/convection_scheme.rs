use crate::{
    CellField,
    high_order_convection::HighOrderConvection,
};

pub struct ConvectionScheme;

impl ConvectionScheme {
    pub fn compute_face_flux(
        field: &CellField,
        owner: usize,
        neighbour: usize,
        mass_flux: f64,
    ) -> f64 {
        HighOrderConvection::face_flux(
            field,
            owner,
            neighbour,
            mass_flux,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convection_scheme_computes_flux() {
        let mut field = CellField::new(2);

        field.set(0, 1.0);
        field.set(1, 5.0);

        let flux = ConvectionScheme::compute_face_flux(
            &field,
            0,
            1,
            2.0,
        );

        assert!(flux > 2.0);
        assert!(flux < 10.0);
    }
}