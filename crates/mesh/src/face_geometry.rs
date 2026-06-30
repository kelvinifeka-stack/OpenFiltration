use math::Scalar;

#[derive(Debug, Clone)]
pub struct FaceGeometry {
    area: Scalar,
    normal_x: Scalar,
    normal_y: Scalar,
    center_x: Scalar,
    center_y: Scalar,
}

impl FaceGeometry {
    pub fn new(
        area: Scalar,
        normal_x: Scalar,
        normal_y: Scalar,
        center_x: Scalar,
        center_y: Scalar,
    ) -> Self {
        Self {
            area,
            normal_x,
            normal_y,
            center_x,
            center_y,
        }
    }

    pub fn area(&self) -> Scalar {
        self.area
    }

    pub fn normal(&self) -> (Scalar, Scalar) {
        (self.normal_x, self.normal_y)
    }

    pub fn center(&self) -> (Scalar, Scalar) {
        (self.center_x, self.center_y)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_face_geometry() {

        let geo = FaceGeometry::new(
            Scalar(1.0),
            Scalar(1.0),
            Scalar(0.0),
            Scalar(0.5),
            Scalar(0.0),
        );

        assert_eq!(geo.area().0,1.0);
    }
}