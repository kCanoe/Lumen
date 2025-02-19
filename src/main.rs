use std::time::Instant;

mod image;
use image::Pixel;
use image::Image;

mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;
use ray::Interval;

mod objects;
use objects::Sphere;
use objects::HitRecord;
use objects::ObjectList;

mod camera;
use camera::CameraSettings;

mod render;
use render::render;

fn main() {
    let mut camera = CameraSettings::new(1024, 576);

    camera.initialize();

    let objects = ObjectList {
        objects: vec![
            Sphere::new(0.3, Vec3::new(0.3, 0.0, -1.0)),
            Sphere::new(0.2, Vec3::new(-0.3, -0.1, -1.0)),
            Sphere::new(100.0, Vec3::new(0.0, -100.5, -1.0)),
        ],
    };

    let image = render(32, &camera, &objects);

    image.print();
}




