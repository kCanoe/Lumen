#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};
    use Lumen::camera::CameraSettings;
    use Lumen::materials::{Dielectric, Diffuse, Material, Metal};
    use Lumen::objects::{ObjectList, Sphere};
    use Lumen::render::render;
    use Lumen::render::ChunkRenderer;
    use Lumen::vec3::Vec3;

    #[bench]
    fn bench_ray_trace(b: &mut Bencher) {
        let (camera, objects) = setup();
        b.iter(|| {
            let image = render(32, camera, objects.clone());
            black_box(image);
        });
    }

    fn setup() -> (CameraSettings, ObjectList) {
        let mut objects = ObjectList::new(Vec::new());
        let ground = Material::Diffuse(Diffuse {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        });
        objects.add(Sphere::new(1000.0, Vec3::new(0.0, -1000.0, 0.0), ground));
        let mat1 = Material::Dielectric(Dielectric { refraction: 1.50 });
        objects.add(Sphere::new(1.0, Vec3::new(0.0, 1.0, 0.0), mat1));
        let mat2 = Material::Diffuse(Diffuse {
            albedo: Vec3::new(0.2, 0.5, 0.7),
        });
        objects.add(Sphere::new(1.0, Vec3::new(-2.0, 1.0, 0.0), mat2));
        let mat3 = Material::Metal(Metal {
            albedo: Vec3::new(0.2, 0.7, 0.5),
            fuzz: 0.0,
        });
        objects.add(Sphere::new(1.0, Vec3::new(2.0, 1.0, 0.0), mat3));
        let mut camera = CameraSettings::new(512, 288);
        camera.vertical_fov = 20.0;
        camera.look_from = Vec3::new(8.0, 2.0, 10.0);
        camera.look_at = Vec3::new(0.0, 0.75, 0.0);
        camera.up = Vec3::new(0.0, 1.0, 0.0);
        camera.samples = 5;
        camera.max_depth = 5;
        camera.initialize();
        (camera, objects)
    }
}
