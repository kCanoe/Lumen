use lumen::camera::{Camera, CameraBuilder};
use lumen::materials::Material;
use lumen::objects::ObjectList;
use lumen::render::Renderer;
use lumen::vec3::Vec3;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn setup() -> (Camera, ObjectList, usize) {
    let ground = Material::new_diffuse(Vec3::new(0.5, 0.5, 0.5));
    let mat1 = Material::new_dielectric(1.50);
    let mat2 = Material::new_diffuse(Vec3::new(0.2, 0.5, 0.7));
    let mat3 = Material::new_metal(Vec3::new(0.2, 0.7, 0.5), 0.0);

    let mut objects = ObjectList::new(Vec::new());

    objects.add_sphere(1000.0, Vec3::new(0.0, -1000.0, 0.0), ground);
    objects.add_sphere(1.0, Vec3::new(0.0, 1.0, 0.0), mat1);
    objects.add_sphere(1.0, Vec3::new(-2.0, 1.0, 0.0), mat2);
    objects.add_sphere(1.0, Vec3::new(2.0, 1.0, 0.0), mat3);

    let red = Material::new_diffuse(Vec3::new(1.0, 0.2, 0.2));

    objects.add_quad(
        Vec3::new(-1.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        red,
    );

    let camera = CameraBuilder::new()
        .resolution(1024, 576)
        .vfov(90.0)
        .target(0.0, 0.0, -1.0)
        .position(3.0, 2.0, 3.0)
        .upward(0.0, 1.0, 0.0)
        .samples(10)
        .max_depth(10)
        .build();

    (camera, objects, 8)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let output_path = PathBuf::from(args[1].as_str());
    let mut output = File::create(output_path)?;

    let (camera, objects, thread_count) = setup();
    let image = Renderer::new(camera, objects, thread_count).render();

    let output_text = format!("{image}");
    output.write_all(output_text.as_bytes())?;

    Ok(())
}
