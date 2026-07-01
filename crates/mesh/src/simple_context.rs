use crate::{
    CellField,
    FaceField,
    ResidualHistory,
    SolverControls,
    VectorField,
};

#[derive(Debug, Clone)]
pub struct SimpleContext {
    pressure: CellField,
    velocity: VectorField,
    face_flux: FaceField,
    residuals: ResidualHistory,
    controls: SolverControls,
}

impl SimpleContext {
    pub fn new(
        cells: usize,
        faces: usize,
        controls: SolverControls,
    ) -> Self {
        Self {
            pressure: CellField::new(cells),
            velocity: VectorField::new(cells),
            face_flux: FaceField::new(faces),
            residuals: ResidualHistory::new(),
            controls,
        }
    }

    pub fn pressure(&self) -> &CellField {
        &self.pressure
    }

    pub fn pressure_mut(&mut self) -> &mut CellField {
        &mut self.pressure
    }

    pub fn velocity(&self) -> &VectorField {
        &self.velocity
    }

    pub fn velocity_mut(&mut self) -> &mut VectorField {
        &mut self.velocity
    }

    pub fn face_flux(&self) -> &FaceField {
        &self.face_flux
    }

    pub fn face_flux_mut(&mut self) -> &mut FaceField {
        &mut self.face_flux
    }

    pub fn residuals(&self) -> &ResidualHistory {
        &self.residuals
    }

    pub fn residuals_mut(&mut self) -> &mut ResidualHistory {
        &mut self.residuals
    }

    pub fn controls(&self) -> &SolverControls {
        &self.controls
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_simple_context() {
        let controls = SolverControls::default();

        let ctx = SimpleContext::new(
            20,
            40,
            controls,
        );

        assert_eq!(ctx.pressure().len(), 20);
        assert_eq!(ctx.face_flux().len(), 40);
        assert!(ctx.residuals().is_empty());
    }
}