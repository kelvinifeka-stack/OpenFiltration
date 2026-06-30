#[derive(Debug, Clone)]
pub struct SimpleController {
    iteration: usize,
    max_iterations: usize,
}

impl SimpleController {
    pub fn new(max_iterations: usize) -> Self {
        Self {
            iteration: 0,
            max_iterations,
        }
    }

    pub fn iteration(&self) -> usize {
        self.iteration
    }

    pub fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    pub fn has_finished(&self) -> bool {
        self.iteration >= self.max_iterations
    }

    pub fn advance(&mut self) {
        if !self.has_finished() {
            self.iteration += 1;
        }
    }

    pub fn reset(&mut self) {
        self.iteration = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn controller_advances() {
        let mut controller = SimpleController::new(5);

        assert_eq!(controller.iteration(), 0);

        controller.advance();

        assert_eq!(controller.iteration(), 1);
    }

    #[test]
    fn controller_stops() {
        let mut controller = SimpleController::new(2);

        controller.advance();
        controller.advance();
        controller.advance();

        assert_eq!(controller.iteration(), 2);
        assert!(controller.has_finished());
    }

    #[test]
    fn controller_resets() {
        let mut controller = SimpleController::new(4);

        controller.advance();
        controller.advance();

        controller.reset();

        assert_eq!(controller.iteration(), 0);
    }
}