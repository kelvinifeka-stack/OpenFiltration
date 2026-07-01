#[derive(Debug, Clone)]
pub struct ResidualEngine {
    residuals: Vec<f64>,
}

impl ResidualEngine {

    pub fn new(equation_count: usize) -> Self {
        Self {
            residuals: vec![0.0; equation_count],
        }
    }

    pub fn len(&self) -> usize {
        self.residuals.len()
    }

    pub fn set(
        &mut self,
        equation: usize,
        residual: f64,
    ) {
        self.residuals[equation] = residual;
    }

    pub fn add(
        &mut self,
        equation: usize,
        residual: f64,
    ) {
        self.residuals[equation] += residual;
    }

    pub fn residual(
        &self,
        equation: usize,
    ) -> f64 {
        self.residuals[equation]
    }

    pub fn max_residual(&self) -> f64 {

        self.residuals
            .iter()
            .copied()
            .fold(0.0, f64::max)
    }

    pub fn reset(&mut self) {
        self.residuals.fill(0.0);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_engine() {

        let engine =
            ResidualEngine::new(4);

        assert_eq!(engine.len(), 4);
    }

    #[test]
    fn accumulate_residuals() {

        let mut engine =
            ResidualEngine::new(2);

        engine.add(0, 1.5);
        engine.add(0, 2.5);

        assert_eq!(
            engine.residual(0),
            4.0,
        );
    }

    #[test]
    fn compute_maximum() {

        let mut engine =
            ResidualEngine::new(3);

        engine.set(0, 0.01);
        engine.set(1, 0.50);
        engine.set(2, 0.20);

        assert_eq!(
            engine.max_residual(),
            0.50,
        );
    }

    #[test]
    fn reset_engine() {

        let mut engine =
            ResidualEngine::new(2);

        engine.set(0, 5.0);

        engine.reset();

        assert_eq!(
            engine.residual(0),
            0.0,
        );
    }
}