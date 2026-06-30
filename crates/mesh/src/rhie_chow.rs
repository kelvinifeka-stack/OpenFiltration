#[derive(Debug, Clone)]
pub struct RhieChowInterpolation {
    enabled: bool,
}

impl RhieChowInterpolation {
    pub fn new() -> Self {
        Self {
            enabled: true,
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn interpolate(
        &self,
        owner_velocity: f64,
        neighbour_velocity: f64,
    ) -> f64 {
        if self.enabled {
            0.5 * (owner_velocity + neighbour_velocity)
        } else {
            owner_velocity
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolate_face_velocity() {
        let rc = RhieChowInterpolation::new();

        assert_eq!(rc.interpolate(2.0, 4.0), 3.0);
    }
}