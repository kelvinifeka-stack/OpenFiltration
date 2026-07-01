#[derive(Debug, Clone)]
pub struct FaceInterpolationCache {
    owner_weights: Vec<f64>,
    neighbour_weights: Vec<f64>,
}

impl FaceInterpolationCache {
    pub fn new(face_count: usize) -> Self {
        Self {
            owner_weights: vec![0.0; face_count],
            neighbour_weights: vec![0.0; face_count],
        }
    }

    pub fn len(&self) -> usize {
        self.owner_weights.len()
    }

    pub fn is_empty(&self) -> bool {
        self.owner_weights.is_empty()
    }

    pub fn set(
        &mut self,
        face: usize,
        owner: f64,
        neighbour: f64,
    ) {
        self.owner_weights[face] = owner;
        self.neighbour_weights[face] = neighbour;
    }

    pub fn get(
        &self,
        face: usize,
    ) -> (f64, f64) {
        (
            self.owner_weights[face],
            self.neighbour_weights[face],
        )
    }

    pub fn reset(&mut self) {
        self.owner_weights.fill(0.0);
        self.neighbour_weights.fill(0.0);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_cache() {

        let cache = FaceInterpolationCache::new(12);

        assert_eq!(cache.len(), 12);
        assert!(!cache.is_empty());
    }

    #[test]
    fn store_weights() {

        let mut cache = FaceInterpolationCache::new(4);

        cache.set(2, 0.75, 0.25);

        assert_eq!(
            cache.get(2),
            (0.75, 0.25),
        );
    }

    #[test]
    fn reset_cache() {

        let mut cache = FaceInterpolationCache::new(3);

        cache.set(0, 0.6, 0.4);
        cache.set(1, 0.8, 0.2);

        cache.reset();

        for i in 0..3 {
            assert_eq!(
                cache.get(i),
                (0.0, 0.0),
            );
        }
    }
}