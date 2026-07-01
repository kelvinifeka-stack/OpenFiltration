use crate::{
    LinearSystem,
};

/// Common interface implemented by all transport equation assemblers.
///
/// This allows the solver to drive momentum,
/// energy, scalar, turbulence and future
/// transport equations through one API.
pub trait TransportAssembler {
    fn assemble(&mut self);

    fn reset(&mut self);

    fn build(self) -> LinearSystem
    where
        Self: Sized;
}