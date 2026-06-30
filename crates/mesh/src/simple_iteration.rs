use crate::{
    MassFlux,
    Mesh,
    MomentumEquation,
    PressureCorrection,
    PressureEquation,
    Residual,
    VelocityCorrection,
};

pub struct SimpleIteration;

impl SimpleIteration {
    pub fn execute(mesh: &Mesh) {
        let cell_count = mesh.cell_count();
        let face_count = mesh.face_count();

        // Assemble momentum equation
        let _momentum = MomentumEquation::new(cell_count);

        // Assemble mass fluxes
        let _flux = MassFlux::new(face_count);

        // Assemble pressure equation
        let _pressure = PressureEquation::new(cell_count);

        // Assemble pressure correction system
        let _pressure_system =
            PressureCorrection::assemble(cell_count);

        // Placeholder correction vectors
        let mut velocity = vec![0.0; cell_count];
        let pressure_correction = vec![0.0; cell_count];

        VelocityCorrection::correct(
            &mut velocity,
            &pressure_correction,
            1.0,
        );

        let _l2 = Residual::l2(&pressure_correction);
        let _max = Residual::max_norm(&pressure_correction);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::MeshBuilder;

    #[test]
    fn simple_iteration_executes() {

        let mesh =
            MeshBuilder::structured(
                2,
                2,
                1.0,
                1.0,
            );

        SimpleIteration::execute(&mesh);
    }
}