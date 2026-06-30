use math::Point2;

#[derive(Debug, Clone)]
pub struct FaceGeometry {
    center: Point2,
    normal: (f64, f64),
    length: f64,
}

impl FaceGeometry {
    pub fn new(
        center: Point2,
        normal: (f64, f64),
        length: f64,
    ) -> Self {
        Self {
            center,
            normal,
            length,
        }
    }

    pub fn center(&self) -> Point2 {
        self.center
    }

    pub fn normal(&self) -> (f64, f64) {
        self.normal
    }

    pub fn length(&self) -> f64 {
        self.length
    }
}