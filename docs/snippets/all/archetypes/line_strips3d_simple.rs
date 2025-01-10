//! Log a simple line strip.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rec = rerun::RecordingStreamBuilder::new("rerun_example_line_strip3d").spawn()?;

    let points = [
        [0., 0., 0.],
        [0., 0., 1.],
        [1., 0., 0.],
        [1., 0., 1.],
        [1., 1., 0.],
        [1., 1., 1.],
        [0., 1., 0.],
        [0., 1., 1.],
    ];
    rec.log("strip", &rerun::LineStrips3D::new([points]))?;

    rec.log(
        "strip",
        &rerun::LineStrips3D::update_fields().with_radii([2.0]),
    )?;

    Ok(())
}
