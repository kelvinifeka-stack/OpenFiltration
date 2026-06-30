#[derive(Debug, Clone)]
pub struct SolverControls {
    pub max_iterations: usize,
    pub tolerance: f64,
    pub pressure_relaxation: f64,
    pub velocity_relaxation: f64,
}

impl Default for SolverControls {
    fn default() -> Self {
        Self {
            max_iterations: 500,
            tolerance: 1e-6,
            pressure_relaxation: 0.30,
            velocity_relaxation: 0.70,
        }
    }
}

impl SolverControls {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_controls() {
        let controls = SolverControls::default();

        assert_eq!(controls.max_iterations, 500);
        assert!((controls.tolerance - 1e-6).abs() < 1e-12);
        assert!((controls.pressure_relaxation - 0.30).abs() < 1e-12);
        assert!((controls.velocity_relaxation - 0.70).abs() < 1e-12);
    }

    #[test]
    fn constructor_matches_default() {
        let a = SolverControls::new();
        let b = SolverControls::default();

        assert_eq!(a.max_iterations, b.max_iterations);
        assert_eq!(a.tolerance, b.tolerance);
    }
}