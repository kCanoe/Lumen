use std::thread;

use rand::rngs::ThreadRng;
use rand::distributions::{Distribution, Uniform};

use crate::Vec3;
use crate::Ray;
use crate::Image;
use crate::Interval;
use crate::HitRecord;
use crate::ObjectList;
use crate::CameraSettings;

#[inline]
pub fn get_ray(
    i: usize,
    j: usize,
    dist: &Uniform<f64>,
    rng: &mut ThreadRng,
    cam: &CameraSettings,
) -> Ray {
    let offset = Vec3::new(dist.sample(rng) - 0.5, dist.sample(rng) - 0.5, 0.0);

    let pixel_center = cam.pixel_origin
        + (j as f64 + offset.x) * cam.pixel_delta_u
        + (i as f64 + offset.y) * cam.pixel_delta_v;

    let ray_direction = pixel_center - cam.position;

    Ray::new(cam.position, ray_direction) 
}

#[inline]
pub fn cast_ray(r: Ray, depth: usize, objects: &ObjectList) -> Vec3 {
    if depth <= 0 {
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
        return 0.5 * cast_ray(bounce, depth - 1, objects);
    } else {
        let unit_direction = Vec3::unit_vector(r.direction);
        let a = 0.5 * (unit_direction.y + 1.0);
        return Vec3::clamp((1.0 - a) * Vec3::new(0.5, 0.7, 1.0))
            + Vec3::clamp(a * Vec3::new(1.0, 1.0, 1.0));
    }
}

pub fn process_pixel(
    i: usize,
    j: usize,
    dist: &Uniform<f64>,
    rng: &mut ThreadRng,
    camera: &CameraSettings,
    objects: &ObjectList,
) -> Vec3 {
    let mut color = Vec3::new(0.0, 0.0, 0.0);

    for _ in 0..camera.samples {
        let r = get_ray(i, j, dist, rng, camera);
        color += cast_ray(r, camera.max_depth, objects);
    }
    
    color * camera.sample_scale
}

#[allow(dead_code)]
pub fn render_fast(image: &mut Image, camera: &CameraSettings, objects: &ObjectList) {
    let dist = Uniform::new(0.0, 1.0);
    let mut rng = rand::thread_rng();

    for i in 0..camera.image_height {
        for j in 0..camera.image_width {
            image.set(i, j, &process_pixel(i, j, &dist, &mut rng, camera, objects));
        }
    }
}

#[allow(dead_code)]
pub fn render(image: &mut Image, camera: &CameraSettings, objects: &ObjectList) {
    let mut colors = vec![
        vec![Vec3::new(0.0, 0.0, 0.0); camera.image_width]; camera.image_height
    ];

    let dist = Uniform::new(0.0, 1.0);
    let mut rng = rand::thread_rng();
    
    for i in 0..camera.image_height {
        for j in 0..camera.image_width {
            colors[i][j] = process_pixel(i, j, &dist, &mut rng, camera, objects);
        }
    }

    for i in 0..camera.image_height {
        for j in 0..camera.image_width {
            image.set(i, j, &colors[i][j]);
        }
    }
}

#[allow(dead_code)]
pub fn render_parallel(
    image: &mut Image, 
    camera: &CameraSettings, 
    objects: &ObjectList
) {
    let num_threads = 8;
    let mut handles = Vec::with_capacity(num_threads);

    let chunk_size = camera.image_height / num_threads;

    for n in 0..num_threads {
        let cam_copy = camera.clone();
        let obj_copy = objects.clone();

        let handle = thread::spawn(move || {
            let mut result =
                vec![vec![Vec3::new(0.0, 0.0, 0.0); cam_copy.image_width]; chunk_size];

            let start = n * chunk_size;
            
            let dist = Uniform::new(0.0, 1.0);
            let mut rng = rand::thread_rng();

            for i in 0..chunk_size {
                for j in 0..cam_copy.image_width {
                    result[i][j] = process_pixel(
                        i + start, 
                        j,
                        &dist,
                        &mut rng, 
                        &cam_copy, 
                        &obj_copy
                    );
                }
            }

            result
        });

        handles.push(handle);
    }

    let mut results: Vec<Vec<Vec3>> = Vec::with_capacity(num_threads);

    for handle in handles {
        let result = handle.join().unwrap();
        for r in result {
            results.push(r);
        }
    }

    for i in 0..camera.image_height {
        for j in 0..camera.image_width {
            image.set(i, j, &results[i][j]);
        }
    }
}


