#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InterpolationWeight {
    weight: f64,
}

impl InterpolationWeight {
    pub fn new(weight: f64) -> Self {
        assert!(
            (0.0..=1.0).contains(&weight),
            "Interpolation weight must be between 0 and 1."
        );

        Self { weight }
    }

    pub fn value(&self) -> f64 {
        self.weight
    }

    pub fn from_distances(
        owner_distance: f64,
        neighbour_distance: f64,
    ) -> Self {
        let total = owner_distance + neighbour_distance;

        assert!(total > 0.0);

        Self::new(neighbour_distance / total)
    }
}

#[derive(Debug, Default)]
pub struct InterpolationWeights {
    weights: Vec<InterpolationWeight>,
}

impl InterpolationWeights {
    pub fn new() -> Self {
        Self {
            weights: Vec::new(),
        }
    }

    pub fn add(
        &mut self,
        weight: InterpolationWeight,
    ) {
        self.weights.push(weight);
    }

    pub fn get(
        &self,
        index: usize,
    ) -> Option<InterpolationWeight> {
        self.weights.get(index).copied()
    }

    pub fn len(&self) -> usize {
        self.weights.len()
    }

    pub fn reset(&mut self) {
        self.weights.clear();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn compute_weight() {

        let w = InterpolationWeight::from_distances(
            1.0,
            3.0,
        );

        assert!((w.value() - 0.75).abs() < 1e-12);
    }

    #[test]
    fn store_weights() {

        let mut weights = InterpolationWeights::new();

        weights.add(
            InterpolationWeight::new(0.5),
        );

        assert_eq!(weights.len(),1);
    }

    #[test]
    fn reset_weights() {

        let mut weights = InterpolationWeights::new();

        weights.add(
            InterpolationWeight::new(0.25),
        );

        weights.reset();

        assert_eq!(weights.len(),0);
    }
}