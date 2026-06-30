use math::Point2;

#[derive(Debug, Clone)]
pub struct CellGeometry {
    volume: f64,
    centroid: (f64, f64),
}

impl CellGeometry {
    pub fn new(
        volume: f64,
        centroid: (f64, f64),
    ) -> Self {
        Self {
            volume,
            centroid,
        }
    }

    pub fn volume(&self) -> f64 {
        self.volume
    }

    pub fn centroid(&self) -> (f64, f64) {
        self.centroid
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_cell_geometry() {

        let geometry =
            CellGeometry::new(
                2.5,
                (1.0, 3.0),
            );

        assert_eq!(geometry.volume(), 2.5);
        assert_eq!(geometry.centroid(), (1.0, 3.0));
    }
}