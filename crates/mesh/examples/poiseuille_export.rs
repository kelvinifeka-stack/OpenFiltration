use mesh::{MeshBuilder, VtkWriter};

fn main() {
    let mesh = MeshBuilder::structured(
        20,
        10,
        1.0,
        0.5,
    );

    println!("Mesh contains {} nodes", mesh.node_count());

    // TODO:
    // Create pressure field
    // Create velocity field
    // Export using VtkWriter
}