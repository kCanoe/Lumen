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
use objects::random_double;

mod camera;
use camera::CameraSettings;

mod render;
use render::render;

fn main() {
    // creating the world to render
    let mut objects = ObjectList::new(Vec::new());

    let ground_material = Material::Diffuse(Vec3::new(0.5, 0.5, 0.5));
    objects.add(Sphere::new(1000.0, Vec3::new(0.0, -1000.0, 0.0), ground_material));

    for i in -11..11 {
        for j in -11..11 {
            let choose_mat = random_double();
            let center = Vec3 {
                x: i as f64 + 0.9*random_double(),
                y: 0.2,
                z: j as f64 + 0.9*random_double()
            };

            if (center-Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mut sphere_material: Material;
                if choose_mat < 0.8 {
                    let albedo = Vec3::random_vector();
                    sphere_material = Material::Diffuse(albedo);
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_vector();
                    let fuzz = random_double() * 0.5;
                    sphere_material = Material::Metal(albedo, fuzz);
                } else {
                    sphere_material = Material::Dielectric(1.5);
                }
                objects.add(Sphere::new(0.2, center, sphere_material));
            }
        }
    }

    let mat1 = Material::Dielectric(1.5);
    objects.add(Sphere::new(1.0, Vec3::new(0.0, 1.0, 0.0), mat1));

    let mat2 = Material::Diffuse(Vec3::new(0.2, 0.5, 0.7));
    objects.add(Sphere::new(1.0, Vec3::new(-4.0, 1.0, 0.0), mat2));

    let mat3 = Material::Metal(Vec3::new(0.2, 0.7, 0.5), 0.1);
    objects.add(Sphere::new(1.0, Vec3::new(4.0, 1.0, 0.0), mat3));



    // camera
    let mut camera = CameraSettings::new(1024, 576);
    camera.vertical_fov = 20.0;
    camera.look_from = Vec3::new(13.0, 2.0, 3.0);
    camera.look_at = Vec3::new(0.0, 0.0, 0.0);
    camera.up = Vec3::new(0.0, 1.0, 0.0);
    camera.samples = 50;
    camera.sample_scale = 1.0 / 50.0;
    camera.max_depth = 10;
    camera.initialize();

    // create and print image
    let image = render(32, camera, objects);
    image.print();
}




