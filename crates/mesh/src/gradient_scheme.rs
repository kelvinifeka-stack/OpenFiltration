use crate::{
    Mesh,
    ScalarField,
    GradientField,
};

pub trait GradientScheme {
    fn compute(
        &self,
        mesh: &Mesh,
        field: &ScalarField,
    ) -> GradientField;
}