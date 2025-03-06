use lumen::camera::CameraBuilder;
use lumen::materials::{Dielectric, Diffuse, Material, Metal};
use lumen::objects::{ObjectList, Sphere};
use lumen::render::Renderer;
use lumen::vec3::Vec3;

fn main() {
    let ground = Material::Diffuse(Diffuse {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    });
    let mat1 = Material::Dielectric(Dielectric { refraction: 1.50 });
    let mat2 = Material::Diffuse(Diffuse {
        albedo: Vec3::new(0.2, 0.5, 0.7),
    });
    let mat3 = Material::Metal(Metal {
        albedo: Vec3::new(0.2, 0.7, 0.5),
        fuzz: 0.0,
    });

    let mut objects = ObjectList::new(Vec::new());
    objects.add(Sphere::new(1000.0, Vec3::new(0.0, -1000.0, 0.0), ground));
    objects.add(Sphere::new(1.0, Vec3::new(0.0, 1.0, 0.0), mat1));
    objects.add(Sphere::new(1.0, Vec3::new(-2.0, 1.0, 0.0), mat2));
    objects.add(Sphere::new(1.0, Vec3::new(2.0, 1.0, 0.0), mat3));

    let camera = CameraBuilder::new()
        .resolution(1024, 576)
        .vfov(90.0)
        .target(Vec3::new(0.0, 0.0, -1.0))
        .position(Vec3::new(-2.0, 2.0, 1.0))
        .upward(Vec3::new(0.0, 1.0, 0.0))
        .samples(100)
        .max_depth(10)
        .build();

    let image = Renderer::new(camera, objects, 8).render();

    image.print();
}
