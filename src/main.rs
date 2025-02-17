mod vec3;
use vec3::Vec3;

mod point3;
use point3::Point3;

mod color;
use color::Color;

mod hit;
use hit::HitRecord;

mod shapes;
use shapes::Sphere;
use shapes::ObjectList;

mod ray;
use ray::Ray;

mod image;
use image::Image;

mod camera;
use camera::Camera;

fn main() {
    let mut cam = Camera::default();

    cam.set_aspect(16.0, 9.0);
    cam.set_viewport_width(2.0);
    cam.set_image_width(1024);
    cam.initialize();

    let mut world = ObjectList::new();
    
    let sphere = Sphere::new(0.2, 0.0, 0.0, -1.0);
    let ground = Sphere::new(1000.0, 0.0, -101.5, -1.0);

    world.add(&sphere);
    world.add(&ground);

    let image = cam.render(&world);

    println!("{}", image);
}

