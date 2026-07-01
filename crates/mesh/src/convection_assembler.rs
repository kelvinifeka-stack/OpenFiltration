use crate::FvmAssembler;

/// Assembles the convective contribution to a finite-volume system.
#[derive(Debug, Default)]
pub struct ConvectionAssembler;

impl ConvectionAssembler {
    pub fn new() -> Self {
        Self
    }

    /// Assemble a first-order upwind convection contribution.
    ///
    /// Positive flux:
    ///   owner ----> neighbour
    ///
    /// Negative flux:
    ///   neighbour ----> owner
    pub fn assemble(
        &self,
        assembler: &mut FvmAssembler,
        owner: usize,
        neighbour: usize,
        flux: f64,
    ) {
        if flux >= 0.0 {
            assembler.add_to_diagonal(owner, flux);
            assembler.add_to_off_diagonal(owner, neighbour, -flux);
        } else {
            let f = -flux;

            assembler.add_to_diagonal(neighbour, f);
            assembler.add_to_off_diagonal(neighbour, owner, -f);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FvmAssembler;

    #[test]
    fn assemble_positive_flux() {
        let convection = ConvectionAssembler::new();

        let mut assembler = FvmAssembler::new(2);

        convection.assemble(
            &mut assembler,
            0,
            1,
            5.0,
        );

        let system = assembler.build();

        assert_eq!(system.size(), 2);
    }

    #[test]
    fn assemble_negative_flux() {
        let convection = ConvectionAssembler::new();

        let mut assembler = FvmAssembler::new(2);

        convection.assemble(
            &mut assembler,
            0,
            1,
            -5.0,
        );

        let system = assembler.build();

        assert_eq!(system.size(), 2);
    }
}