//! Log a simple colored triangle.

use rerun::{archetypes::Mesh3D, RecordingStreamBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (rec, storage) = RecordingStreamBuilder::new("rerun_example_mesh3d_simple").memory()?;

    rec.log(
        "triangle",
        &Mesh3D::new([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]])
            .with_vertex_normals([[0.0, 0.0, 1.0]])
            .with_vertex_colors([0xFF0000FF, 0x00FF00FF, 0x0000FFFF]),
    )?;

    rerun::native_viewer::show(storage.take())?;
    Ok(())
}
