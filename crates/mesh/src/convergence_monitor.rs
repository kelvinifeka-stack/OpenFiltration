use crate::ResidualHistory;

pub struct ConvergenceMonitor {
    tolerance: f64,
}

impl ConvergenceMonitor {
    pub fn new(tolerance: f64) -> Self {
        assert!(tolerance > 0.0);

        Self { tolerance }
    }

    pub fn converged(
        &self,
        history: &ResidualHistory,
    ) -> bool {
        match history.latest() {
            Some(value) => value <= self.tolerance,
            None => false,
        }
    }

    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ResidualHistory;

    #[test]
    fn detects_convergence() {
        let mut history = ResidualHistory::new();

        history.record(1e-3);
        history.record(1e-7);

        let monitor = ConvergenceMonitor::new(1e-6);

        assert!(monitor.converged(&history));
    }

    #[test]
    fn detects_non_convergence() {
        let mut history = ResidualHistory::new();

        history.record(1e-2);

        let monitor = ConvergenceMonitor::new(1e-6);

        assert!(!monitor.converged(&history));
    }
}