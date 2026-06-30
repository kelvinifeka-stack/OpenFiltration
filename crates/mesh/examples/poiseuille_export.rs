use mesh::{MeshBuilder, VtkWriter};

fn main() -> std::io::Result<()> {
    let mesh = MeshBuilder::structured(
        20,
        10,
        1.0,
        0.5,
    );

    println!("Mesh contains {} nodes", mesh.node_count());

    let points = mesh.points();

    VtkWriter::write_points(
        "poiseuille.vtk",
        &points,
    )?;

    println!(
        "Exported {} mesh nodes.",
        points.len()
    );

    Ok(())
}