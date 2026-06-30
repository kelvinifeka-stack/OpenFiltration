use std::fs::File;
use std::io::{Result, Write};

pub struct VtkWriter;

impl VtkWriter {
    pub fn write_points(
        path: &str,
        points: &[(f64, f64)],
    ) -> Result<()> {
        let mut file = File::create(path)?;

        writeln!(file, "# vtk DataFile Version 3.0")?;
        writeln!(file, "OpenFiltration")?;
        writeln!(file, "ASCII")?;
        writeln!(file, "DATASET POLYDATA")?;

        writeln!(file, "POINTS {} float", points.len())?;

        for (x, y) in points {
            writeln!(file, "{} {} 0.0", x, y)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn write_vtk_file() {
        let path = "test_output.vtk";

        VtkWriter::write_points(
            path,
            &[(0.0, 0.0), (1.0, 0.0)],
        )
        .unwrap();

        assert!(fs::metadata(path).is_ok());

        fs::remove_file(path).unwrap();
    }
}