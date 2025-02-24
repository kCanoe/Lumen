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
use objects::Material;
use objects::HitRecord;
use objects::ObjectList;

mod camera;
use camera::CameraSettings;

mod render;
use render::render;

fn main() {
    let ground = Material::Diffuse(Vec3::new(0.8, 0.8, 0.0));
    let center = Material::Diffuse(Vec3::new(0.1, 0.2, 0.5));
    let left = Material::Dielectric(1.00 / 1.33);
    let right = Material::Metal(Vec3::new(0.8, 0.6, 0.2), 1.0);

    let mut camera = CameraSettings::new(1024, 576);

    camera.initialize();

    let objects = ObjectList {
        objects: vec![
            Sphere::new(100.0, Vec3::new(0.0, -100.5, -1.0), ground),
            Sphere::new(0.5, Vec3::new(0.0, 0.0, -1.2), center),
            Sphere::new(0.5, Vec3::new(1.0, 0.0, -1.0), right),
            Sphere::new(0.5, Vec3::new(-1.0, 0.0, -1.0), left),
        ],
    };

    let image = render(32, camera, objects);

    image.print();
}




