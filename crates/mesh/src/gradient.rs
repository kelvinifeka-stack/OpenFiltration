#[derive(Debug, Clone)]
pub struct Gradient {
    values: Vec<(f64, f64)>,
}

impl Gradient {
    pub fn new(size: usize) -> Self {
        Self {
            values: vec![(0.0, 0.0); size],
        }
    }

    pub fn set(
        &mut self,
        index: usize,
        gx: f64,
        gy: f64,
    ) {
        self.values[index] = (gx, gy);
    }

    pub fn get(
        &self,
        index: usize,
    ) -> (f64, f64) {
        self.values[index]
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_gradient() {

        let mut g = Gradient::new(3);

        g.set(1, 2.5, -1.0);

        assert_eq!(g.get(1), (2.5, -1.0));
        assert_eq!(g.len(), 3);
    }
}