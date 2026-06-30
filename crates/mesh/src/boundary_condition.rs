#[derive(Debug, Clone)]
pub enum BoundaryCondition {
    FixedValue(f64),
    ZeroGradient,
    FixedGradient(f64),
    Symmetry,
    Empty,
}