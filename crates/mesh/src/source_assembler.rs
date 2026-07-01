use crate::FvmAssembler;

#[derive(Debug)]
pub struct SourceAssembler;

impl SourceAssembler {
    pub fn assemble(
        assembler: &mut FvmAssembler,
        cell: usize,
        implicit: f64,
        explicit: f64,
    ) {
        assembler.add_to_diagonal(cell, implicit);
        assembler.add_to_rhs(cell, explicit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FvmAssembler;

    #[test]
    fn assemble_positive_source() {
        let mut assembler = FvmAssembler::new(1);

        SourceAssembler::assemble(
            &mut assembler,
            0,
            2.0,
            5.0,
        );

        let system = assembler.build();

        assert_eq!(system.matrix().get(0, 0), 2.0);
        assert_eq!(system.rhs()[0], 5.0);
    }

    #[test]
    fn assemble_negative_source() {
        let mut assembler = FvmAssembler::new(1);

        SourceAssembler::assemble(
            &mut assembler,
            0,
            -3.0,
            -7.0,
        );

        let system = assembler.build();

        assert_eq!(system.matrix().get(0, 0), -3.0);
        assert_eq!(system.rhs()[0], -7.0);
    }

    #[test]
    fn multiple_source_terms_accumulate() {
        let mut assembler = FvmAssembler::new(1);

        SourceAssembler::assemble(
            &mut assembler,
            0,
            1.0,
            2.0,
        );

        SourceAssembler::assemble(
            &mut assembler,
            0,
            3.0,
            4.0,
        );

        let system = assembler.build();

        assert_eq!(system.matrix().get(0, 0), 4.0);
        assert_eq!(system.rhs()[0], 6.0);
    }
}