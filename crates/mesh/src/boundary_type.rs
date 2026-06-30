#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundaryType {
    Wall,
    Inlet,
    Outlet,
    Symmetry,
    Empty,
}