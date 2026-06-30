use math::Point2;

#[derive(Debug, Clone)]
pub struct Geometry {
    node_positions: Vec<Point2>,
}

impl Geometry {
    pub fn new() -> Self {
        Self {
            node_positions: Vec::new(),
        }
    }

    pub fn add_node_position(&mut self, point: Point2) {
        self.node_positions.push(point);
    }

    pub fn node_positions(&self) -> &[Point2] {
        &self.node_positions
    }

    pub fn node_position(&self, index: usize) -> Option<&Point2> {
        self.node_positions.get(index)
    }

    pub fn node_count(&self) -> usize {
        self.node_positions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use math::{Point2, Scalar};

    #[test]
    fn create_geometry() {
        let mut geometry = Geometry::new();

        geometry.add_node_position(
            Point2::new(
                Scalar(0.0),
                Scalar(0.0),
            ),
        );

        geometry.add_node_position(
            Point2::new(
                Scalar(1.0),
                Scalar(0.0),
            ),
        );

        assert_eq!(geometry.node_count(), 2);
        assert_eq!(geometry.node_positions().len(), 2);
    }
}