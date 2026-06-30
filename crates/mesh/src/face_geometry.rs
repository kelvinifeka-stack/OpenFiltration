use math::{Point2, Scalar};

#[derive(Debug, Clone)]
pub struct FaceGeometry {
    center: Point2,
    area: Scalar,
    normal: (Scalar, Scalar),
}

impl FaceGeometry {
    pub fn new(
        center: Point2,
        area: Scalar,
        normal: (Scalar, Scalar),
    ) -> Self {
        Self {
            center,
            area,
            normal,
        }
    }

    pub fn center(&self) -> &Point2 {
        &self.center
    }

    pub fn area(&self) -> Scalar {
        self.area
    }

    pub fn normal(&self) -> (Scalar, Scalar) {
        self.normal
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_face_geometry() {

        let geometry = FaceGeometry::new(
            Point2::new(
                Scalar(0.5),
                Scalar(0.0),
            ),
            Scalar(1.0),
            (
                Scalar(1.0),
                Scalar(0.0),
            ),
        );

        assert_eq!(geometry.area(), Scalar(1.0));
    }
}