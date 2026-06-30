#[derive(Debug, Clone)]
pub struct FaceConnectivity {
    owner: usize,
    neighbour: Option<usize>,
}

impl FaceConnectivity {
    pub fn new(owner: usize, neighbour: Option<usize>) -> Self {
        Self { owner, neighbour }
    }

    pub fn owner(&self) -> usize {
        self.owner
    }

    pub fn neighbour(&self) -> Option<usize> {
        self.neighbour
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_face_connectivity() {
        let conn = FaceConnectivity::new(3, Some(4));

        assert_eq!(conn.owner(), 3);
        assert_eq!(conn.neighbour(), Some(4));
    }
}