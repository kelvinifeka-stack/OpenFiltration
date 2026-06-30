use crate::LinearSystem;

pub struct SystemAssembly;

impl SystemAssembly {
    pub fn apply_dirichlet(
        system: &mut LinearSystem,
        node: usize,
        value: f64,
    ) {
        system.matrix_mut().add(node, node, 1.0);
        system.rhs_mut()[node] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LinearSystem;

    #[test]
    fn apply_boundary_condition() {
        let mut system = LinearSystem::new(3);

        SystemAssembly::apply_dirichlet(
            &mut system,
            0,
            100.0,
        );

        assert_eq!(system.rhs()[0], 100.0);
    }
}