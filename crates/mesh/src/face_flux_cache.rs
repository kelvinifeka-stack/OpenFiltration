#[derive(Debug, Clone)]
pub struct FaceFluxCache {
    convective_flux: Vec<f64>,
    diffusive_flux: Vec<f64>,
}

impl FaceFluxCache {
    pub fn new(face_count: usize) -> Self {
        Self {
            convective_flux: vec![0.0; face_count],
            diffusive_flux: vec![0.0; face_count],
        }
    }

    pub fn len(&self) -> usize {
        self.convective_flux.len()
    }

    pub fn is_empty(&self) -> bool {
        self.convective_flux.is_empty()
    }

    pub fn set_convective(
        &mut self,
        face: usize,
        value: f64,
    ) {
        self.convective_flux[face] = value;
    }

    pub fn set_diffusive(
        &mut self,
        face: usize,
        value: f64,
    ) {
        self.diffusive_flux[face] = value;
    }

    pub fn convective(
        &self,
        face: usize,
    ) -> f64 {
        self.convective_flux[face]
    }

    pub fn diffusive(
        &self,
        face: usize,
    ) -> f64 {
        self.diffusive_flux[face]
    }

    pub fn reset(&mut self) {
        self.convective_flux.fill(0.0);
        self.diffusive_flux.fill(0.0);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_cache() {

        let cache = FaceFluxCache::new(8);

        assert_eq!(cache.len(), 8);
    }

    #[test]
    fn store_fluxes() {

        let mut cache = FaceFluxCache::new(4);

        cache.set_convective(1, 12.5);
        cache.set_diffusive(1, -0.8);

        assert_eq!(
            cache.convective(1),
            12.5,
        );

        assert_eq!(
            cache.diffusive(1),
            -0.8,
        );
    }

    #[test]
    fn reset_fluxes() {

        let mut cache = FaceFluxCache::new(3);

        cache.set_convective(0, 5.0);
        cache.set_diffusive(0, 2.0);

        cache.reset();

        for i in 0..3 {

            assert_eq!(
                cache.convective(i),
                0.0,
            );

            assert_eq!(
                cache.diffusive(i),
                0.0,
            );
        }
    }
}