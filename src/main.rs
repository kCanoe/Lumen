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

    let image = cam.render();

    println!("{}", image);
}



