use crate::CellField;

use crate::flux_limiter::{
    FluxLimiter,
    VanLeerLimiter,
};

pub struct Muscl;

impl Muscl {
    pub fn reconstruct(
        field: &CellField,
        owner: usize,
        neighbour: usize,
    ) -> f64 {
        let phi_o = field.get(owner);
        let phi_n = field.get(neighbour);

        let delta = phi_n - phi_o;

        let r = 1.0;

        let psi = VanLeerLimiter::limiter(r);

        phi_o + 0.5 * psi * delta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reconstruct_between_two_cells() {
        let mut field = CellField::new(2);

        field.set(0, 1.0);
        field.set(1, 3.0);

        let value = Muscl::reconstruct(
            &field,
            0,
            1,
        );

        assert!(value > 1.0);
        assert!(value < 3.0);
    }
}