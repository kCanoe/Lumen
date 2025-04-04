use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use lumen::rendering::*;
use lumen::Material;
use lumen::ObjectList;

fn make_cube_array() -> ObjectList {
    let mut objects = ObjectList::new();
    for i in 0..4 {
        for j in 0..4 {
            let (cx, cy) = (3.0 * i as f64, 3.0 * j as f64);
            let mat = Material::new_metal(0.5, 0.5, 0.7, 0.0);
            objects.add_cube(1.0, cx, 0.5, cy, mat);
        }
    }
    let ground = Material::new_diffuse(0.5, 0.5, 0.5);
    objects.add_sphere(1000.0, 0.0, -1000.0, 0.0, ground);
    objects
}

fn setup() -> (Camera, ObjectList, usize, usize) {
    let objects = make_cube_array();

    let camera = CameraBuilder::new()
        .resolution(1024, 576)
        .vfov(90.0)
        .target(6.0, 0.0, 6.0)
        .position(8.0, 8.0, 8.0)
        .upward(0.0, 1.0, 0.0)
        .samples(1)
        .max_depth(10)
        .build();

    (camera, objects, 8, 256)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let output_path = PathBuf::from(args[1].as_str());
    let mut output = File::create(output_path)?;

    let (camera, objects, thread_count, batch_count) = setup();
    let renderer = Renderer::new(camera, objects, thread_count, batch_count);
    let image = renderer.render();

    let output_text = format!("{image}");
    output.write_all(output_text.as_bytes())?;

    Ok(())
}
