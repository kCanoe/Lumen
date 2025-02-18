mod image;
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
use render::render_parallel;
//use render::render;
//use render::render_fast;

fn main() {
    let mut image = Image::new(1024, 576);
    let mut camera = CameraSettings::new(1024, 576);

    camera.initialize();

    let tmp: Vec<Sphere> = vec![
        Sphere::new(0.3, Vec3::new(0.3, 0.0, -1.0)),
        Sphere::new(0.2, Vec3::new(-0.3, -0.1, -1.0)),
        Sphere::new(100.0, Vec3::new(0.0, -100.5, -1.0)),
    ];

    let objects = ObjectList::new(tmp);

    render_parallel(&mut image, &camera, &objects);
    //render(&mut image, &camera, &objects);
    //render_fast(&mut image, &camera, &objects);

    image.print();
}




