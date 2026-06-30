use mesh::{MeshBuilder, VtkWriter};

fn main() -> std::io::Result<()> {
    let mesh = MeshBuilder::structured(
        20,
        10,
        1.0,
        0.5,
    );

    println!("Mesh contains {} nodes", mesh.node_count());

    // Temporary point cloud for testing the exporter.
    // Replace these with real mesh node coordinates later.
    let points = vec![
        (0.0, 0.0),
        (1.0, 0.0),
        (1.0, 1.0),
        (0.0, 1.0),
    ];

    VtkWriter::write_points(
        "poiseuille.vtk",
        &points,
    )?;

    println!("VTK file written to poiseuille.vtk");

    Ok(())
}