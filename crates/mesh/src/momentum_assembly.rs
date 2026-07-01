#[derive(Debug, Default, Clone)]
pub struct MomentumAssembly {
    assembled: bool,
}

impl MomentumAssembly {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn assemble(&mut self) {
        self.assembled = true;
    }

    pub fn assembled(&self) -> bool {
        self.assembled
    }

    pub fn reset(&mut self) {
        self.assembled = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assembly_runs() {
        let mut assembly = MomentumAssembly::new();

        assert!(!assembly.assembled());

        assembly.assemble();

        assert!(assembly.assembled());
    }

    #[test]
    fn reset() {
        let mut assembly = MomentumAssembly::new();

        assembly.assemble();
        assembly.reset();

        assert!(!assembly.assembled());
    }
}