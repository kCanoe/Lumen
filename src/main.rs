mod vec3;
use vec3::Vec3;

mod point3;
use point3::Point3;

mod color;
use color::Color;

mod interval;
use interval::Interval;

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

    world.add(Box::new(Sphere::new(0.3, 0.3, 0.0, -1.0)));
    world.add(Box::new(Sphere::new(0.2, -0.3, -0.1, -1.0)));
    world.add(Box::new(Sphere::new(100.0, 0.0, -100.5, -1.0)));

    let image = cam.render(&world);

    println!("{}", image);
}

