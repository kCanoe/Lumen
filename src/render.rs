use rand::rngs::ThreadRng;
use rand::distributions::{Distribution, Uniform};

use crate::Vec3;
use crate::Ray;
use crate::Image;
use crate::Interval;
use crate::HitRecord;
use crate::ObjectList;
use crate::CameraSettings;

pub fn get_ray(
    i: usize,
    j: usize,
    dist: &Uniform<f64>,
    rng: &mut ThreadRng,
    cam: &CameraSettings,
) -> Ray {
    let offset = Vec3::new(dist.sample(rng) - 0.5, dist.sample(rng) - 0.5, 0.0);

    let pixel_center = cam.pixel_origin
        + (i as f64 + offset.x) * cam.pixel_delta_u
        + (j as f64 + offset.y) * cam.pixel_delta_v;

    let ray_direction = pixel_center - cam.position;

    Ray::new(cam.position, ray_direction) 
}

pub fn cast_ray(r: Ray, depth: usize, objects: &ObjectList) -> Vec3 {
    if depth < 1 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let mut record = HitRecord::new();

    let mut hit = false;
    let mut tmp = HitRecord::new();
    let mut closest = Interval::new(0.001, f64::INFINITY);

    for object in &objects.objects {
        if object.hit(&r, &closest, &mut tmp) == true {
            hit = true;
            closest.max = tmp.t;
            record = tmp;
        }
    }

    if hit == true {
        let direction = record.normal + Vec3::random_unit_vector();
        let bounce = Ray::new(record.point, direction);
        return 0.5 * cast_ray(bounce, depth-1, objects);
    } else {
        let unit_direction = Vec3::unit_vector(r.direction);
        let a = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - a) * Vec3::new(0.5, 0.7, 1.0) + a * Vec3::new(1.0, 1.0, 1.0);
    }
}

pub fn process_pixel(
    i: usize,
    j: usize,
    camera: &CameraSettings,
    objects: &ObjectList,
) -> Vec3 {
    let mut color = Vec3::new(0.0, 0.0, 0.0);

    let dist = Uniform::new(0.0, 1.0);
    let mut rng = rand::thread_rng();

    for _ in 0..camera.samples {
        let r = get_ray(i, j, &dist, &mut rng, camera);
        color += cast_ray(r, camera.max_depth, objects);
    }
    
    color * camera.sample_scale
}

pub fn render(image: &mut Image, camera: &CameraSettings, objects: &ObjectList) {
    let mut colors = vec![
        vec![Vec3::new(0.0, 0.0, 0.0); camera.image_height]; camera.image_width
    ];

    for i in 0..camera.image_width {
        for j in 0..camera.image_height {
            colors[i][j] = process_pixel(i, j, camera, objects);
        }
    }

    for i in 0..camera.image_width {
        for j in 0..camera.image_height {
            image.set(i, j, &colors[i][j]);
        }
    }
}
