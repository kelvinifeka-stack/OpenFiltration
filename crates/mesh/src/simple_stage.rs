#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimpleStage {
    AssembleMomentum,
    SolveMomentum,
    AssemblePressureCorrection,
    SolvePressureCorrection,
    CorrectPressure,
    CorrectVelocity,
    UpdateFluxes,
    ComputeResiduals,
}

impl SimpleStage {
    pub fn next(self) -> Option<Self> {
        use SimpleStage::*;

        match self {
            AssembleMomentum => Some(SolveMomentum),
            SolveMomentum => Some(AssemblePressureCorrection),
            AssemblePressureCorrection => Some(SolvePressureCorrection),
            SolvePressureCorrection => Some(CorrectPressure),
            CorrectPressure => Some(CorrectVelocity),
            CorrectVelocity => Some(UpdateFluxes),
            UpdateFluxes => Some(ComputeResiduals),
            ComputeResiduals => None,
        }
    }

    pub fn name(&self) -> &'static str {
        use SimpleStage::*;

        match self {
            AssembleMomentum => "Assemble Momentum",
            SolveMomentum => "Solve Momentum",
            AssemblePressureCorrection => "Assemble Pressure Correction",
            SolvePressureCorrection => "Solve Pressure Correction",
            CorrectPressure => "Correct Pressure",
            CorrectVelocity => "Correct Velocity",
            UpdateFluxes => "Update Fluxes",
            ComputeResiduals => "Compute Residuals",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stages_progress_correctly() {
        let mut stage = SimpleStage::AssembleMomentum;

        stage = stage.next().unwrap();
        assert_eq!(stage, SimpleStage::SolveMomentum);

        stage = stage.next().unwrap();
        assert_eq!(stage, SimpleStage::AssemblePressureCorrection);
    }

    #[test]
    fn final_stage_has_no_successor() {
        assert_eq!(
            SimpleStage::ComputeResiduals.next(),
            None
        );
    }

    #[test]
    fn stage_names_exist() {
        assert_eq!(
            SimpleStage::CorrectVelocity.name(),
            "Correct Velocity"
        );
    }
}