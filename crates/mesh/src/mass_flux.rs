#[derive(Debug, Clone)]
pub struct MassFlux {
    values: Vec<f64>,
}

impl MassFlux {
    pub fn new(face_count: usize) -> Self {
        Self {
            values: vec![0.0; face_count],
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn set(
        &mut self,
        face: usize,
        value: f64,
    ) {
        self.values[face] = value;
    }

    pub fn get(
        &self,
        face: usize,
    ) -> f64 {
        self.values[face]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_mass_flux() {

        let mut flux = MassFlux::new(4);

        flux.set(2, 3.5);

        assert_eq!(flux.len(), 4);
        assert_eq!(flux.get(2), 3.5);
    }
}