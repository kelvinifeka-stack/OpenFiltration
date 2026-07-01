use crate::FvmAssembler;

#[derive(Debug)]
pub struct DiffusionAssembler {
    gamma: f64,
}

impl DiffusionAssembler {
    pub fn new(gamma: f64) -> Self {
        Self { gamma }
    }

    pub fn gamma(&self) -> f64 {
        self.gamma
    }

    pub fn assemble_pair(
        &self,
        assembler: &mut FvmAssembler,
        owner: usize,
        neighbour: usize,
        coefficient: f64,
    ) {
        let value = self.gamma * coefficient;

        assembler.add_to_diagonal(owner, value);
        assembler.add_to_diagonal(neighbour, value);

        assembler.add_to_off_diagonal(owner, neighbour, -value);
        assembler.add_to_off_diagonal(neighbour, owner, -value);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::FvmAssembler;

    #[test]
    fn assemble_single_diffusion_pair() {

        let mut assembler = FvmAssembler::new(2);

        let diffusion = DiffusionAssembler::new(2.0);

        diffusion.assemble_pair(
            &mut assembler,
            0,
            1,
            5.0,
        );

        let system = assembler.build();

        assert_eq!(system.matrix().get(0,0),10.0);
        assert_eq!(system.matrix().get(1,1),10.0);

        assert_eq!(system.matrix().get(0,1),-10.0);
        assert_eq!(system.matrix().get(1,0),-10.0);
    }
}