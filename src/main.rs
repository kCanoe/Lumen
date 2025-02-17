mod types;
use types::*;

mod utils;
use utils::*;

mod camera;
use camera::Camera;

fn main() {
    let mut cam = Camera::default();

    cam.set_aspect(16.0, 9.0);
    cam.set_viewport_width(2.0);
    cam.set_image_width(1024);
    cam.set_sample_count(100);
    cam.initialize();

    let mut world = ObjectList::new();

    world.add(Box::new(Sphere::new(0.3, 0.3, 0.0, -1.0)));
    world.add(Box::new(Sphere::new(0.2, -0.3, -0.1, -1.0)));
    world.add(Box::new(Sphere::new(100.0, 0.0, -100.5, -1.0)));

    let image = cam.render(&world);

    println!("{}", image);
}

