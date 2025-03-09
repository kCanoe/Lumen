use lumen::camera::CameraBuilder;
use lumen::materials::Material;
use lumen::objects::ObjectList;
use lumen::render::Renderer;
use lumen::vec3::Vec3;

fn main() {
    let ground = Material::new_diffuse(Vec3::new(0.5, 0.5, 0.5));
    let mat1 = Material::new_dielectric(1.50);
    let mat2 = Material::new_diffuse(Vec3::new(0.2, 0.5, 0.7));
    let mat3 = Material::new_metal(Vec3::new(0.2, 0.7, 0.5), 0.0);

    let mut objects = ObjectList::new(Vec::new());

    objects.add_sphere(1000.0, Vec3::new(0.0, -1000.0, 0.0), ground);
    objects.add_sphere(1.0, Vec3::new(0.0, 1.0, 0.0), mat1);
    objects.add_sphere(1.0, Vec3::new(-2.0, 1.0, 0.0), mat2);
    objects.add_sphere(1.0, Vec3::new(2.0, 1.0, 0.0), mat3);

    let camera = CameraBuilder::new()
        .resolution(1024, 576)
        .vfov(90.0)
        .target(0.0, 0.0, -1.0)
        .position(3.0, 2.0, 3.0)
        .upward(0.0, 1.0, 0.0)
        .samples(500)
        .max_depth(10)
        .build();

    let image = Renderer::new(camera, objects, 8).render();

    //image.print();
    println!("{image}");
}
