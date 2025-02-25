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
    // creating the world to render
    let mut objects = ObjectList::new(Vec::new());
    let ground_material = Material::Diffuse(Vec3::new(0.5, 0.5, 0.5));
    objects.add(Sphere::new(1000.0, Vec3::new(0.0, -1000.0, 0.0), ground_material));
    let mat1 = Material::Dielectric(1.5);
    objects.add(Sphere::new(1.0, Vec3::new(0.0, 1.0, 0.0), mat1));
    let mat2 = Material::Diffuse(Vec3::new(0.2, 0.5, 0.7));
    objects.add(Sphere::new(1.0, Vec3::new(-2.0, 1.0, 0.0), mat2));
    let mat3 = Material::Metal(Vec3::new(0.2, 0.7, 0.5), 0.0);
    objects.add(Sphere::new(1.0, Vec3::new(2.0, 1.0, 0.0), mat3));

    // camera
    let mut camera = CameraSettings::new(1024, 576);
    camera.vertical_fov = 20.0;
    camera.look_from = Vec3::new(8.0, 2.0, 10.0);
    camera.look_at = Vec3::new(0.0, 0.75, 0.0);
    camera.up = Vec3::new(0.0, 1.0, 0.0);
    camera.samples = 500;
    camera.sample_scale = 1.0 / 500.0;
    camera.max_depth = 20;
    camera.initialize();

    // create and print image
    let image = render(144, camera, objects);
    image.print();
}




