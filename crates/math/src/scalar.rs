/// A mathematical scalar.
///
/// A scalar has magnitude but no direction.
///
/// Examples:
/// - Pressure
/// - Temperature
/// - Density
/// - Concentration
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Scalar(pub f64);