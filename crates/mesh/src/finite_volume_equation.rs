#[derive(Debug, Clone)]
pub struct FiniteVolumeEquation {
    pub diagonal: f64,
    pub source: f64,
}

impl Default for FiniteVolumeEquation {
    fn default() -> Self {
        Self {
            diagonal: 0.0,
            source: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_equation() {
        let eq = FiniteVolumeEquation::default();

        assert_eq!(eq.diagonal, 0.0);
    }
}