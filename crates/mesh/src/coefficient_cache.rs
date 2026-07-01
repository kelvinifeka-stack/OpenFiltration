#[derive(Debug, Clone)]
pub struct CoefficientCache {
    diagonal: Vec<f64>,
    source: Vec<f64>,
    neighbours: Vec<Vec<(usize, f64)>>,
}

impl CoefficientCache {

    pub fn new(cell_count: usize) -> Self {

        Self {
            diagonal: vec![0.0; cell_count],
            source: vec![0.0; cell_count],
            neighbours: vec![Vec::new(); cell_count],
        }
    }

    pub fn len(&self) -> usize {
        self.diagonal.len()
    }

    pub fn add_diagonal(
        &mut self,
        cell: usize,
        value: f64,
    ) {
        self.diagonal[cell] += value;
    }

    pub fn add_source(
        &mut self,
        cell: usize,
        value: f64,
    ) {
        self.source[cell] += value;
    }

    pub fn add_neighbour(
        &mut self,
        owner: usize,
        neighbour: usize,
        coefficient: f64,
    ) {
        self.neighbours[owner]
            .push((neighbour, coefficient));
    }

    pub fn diagonal(
        &self,
        cell: usize,
    ) -> f64 {
        self.diagonal[cell]
    }

    pub fn source(
        &self,
        cell: usize,
    ) -> f64 {
        self.source[cell]
    }

    pub fn neighbours(
        &self,
        cell: usize,
    ) -> &[(usize, f64)] {
        &self.neighbours[cell]
    }

    pub fn reset(&mut self) {

        self.diagonal.fill(0.0);

        self.source.fill(0.0);

        for list in &mut self.neighbours {
            list.clear();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_cache() {

        let cache = CoefficientCache::new(5);

        assert_eq!(cache.len(), 5);
    }

    #[test]
    fn accumulate_coefficients() {

        let mut cache =
            CoefficientCache::new(2);

        cache.add_diagonal(0, 2.0);
        cache.add_diagonal(0, 3.0);

        cache.add_source(0, 5.0);
        cache.add_source(0, 1.5);

        assert_eq!(
            cache.diagonal(0),
            5.0,
        );

        assert_eq!(
            cache.source(0),
            6.5,
        );
    }

    #[test]
    fn store_neighbours() {

        let mut cache =
            CoefficientCache::new(3);

        cache.add_neighbour(
            0,
            1,
            -2.4,
        );

        cache.add_neighbour(
            0,
            2,
            -0.8,
        );

        assert_eq!(
            cache.neighbours(0).len(),
            2,
        );
    }

    #[test]
    fn reset_cache() {

        let mut cache =
            CoefficientCache::new(2);

        cache.add_diagonal(0, 8.0);

        cache.add_source(0, 5.0);

        cache.add_neighbour(
            0,
            1,
            -3.0,
        );

        cache.reset();

        assert_eq!(
            cache.diagonal(0),
            0.0,
        );

        assert_eq!(
            cache.source(0),
            0.0,
        );

        assert!(
            cache.neighbours(0).is_empty()
        );
    }
}