use crate::Mesh;

pub struct SurfaceInterpolation;

impl SurfaceInterpolation {
    pub fn interpolate(_mesh: &Mesh) {
        // placeholder
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::MeshBuilder;

    #[test]
    fn interpolation_runs() {
        let mesh = MeshBuilder::structured(
            2,
            2,
            1.0,
            1.0,
        );

        SurfaceInterpolation::interpolate(&mesh);
    }
}