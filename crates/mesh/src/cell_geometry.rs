use math::Point2;

#[derive(Debug, Clone)]
pub struct CellGeometry {
    center: Point2,
    volume: f64,
}

impl CellGeometry {
    pub fn new(
        center: Point2,
        volume: f64,
    ) -> Self {
        Self {
            center,
            volume,
        }
    }

    pub fn center(&self) -> Point2 {
        self.center
    }

    pub fn volume(&self) -> f64 {
        self.volume
    }
}