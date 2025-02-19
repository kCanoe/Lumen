use std::thread;
use std::sync::{Arc, Mutex};

use rand::rngs::ThreadRng;
use rand::distributions::{Distribution, Uniform};

use crate::Vec3;
use crate::Ray;
use crate::Image;
use crate::Pixel;
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
        return (1.0 - a) * Vec3::new(0.5, 0.7, 1.0) + a * Vec3::new(1.0, 1.0, 1.0);
    }
}

pub fn process_pixel(
    i: usize,
    j: usize,
    dist: &Uniform<f64>,
    rng: &mut ThreadRng,
    camera: &CameraSettings,
    objects: &ObjectList,
) -> Pixel {
    let mut color = Vec3::new(0.0, 0.0, 0.0);

    for _ in 0..camera.samples {
        let r = get_ray(i, j, dist, rng, camera);
        color += cast_ray(r, camera.max_depth, objects);
    }
    
    Pixel::from_vec(color * camera.sample_scale)
}

pub fn render(n_threads: usize, camera: CameraSettings, objects: ObjectList) -> Image {
    let img = Arc::new(Mutex::new(Image::new(camera.image_width, camera.image_height)));
    let (a_cam, a_obj) = (Arc::new(camera), Arc::new(objects));

    let mut handles = Vec::with_capacity(n_threads);
    
    let chunk_rows = camera.image_height / n_threads;
    let chunk_cols = camera.image_width;

    for n in 0..n_threads {
        let img_clone = Arc::clone(&img);
        let cam = Arc::clone(&a_cam);
        let obj = Arc::clone(&a_obj);
        
        let handle = thread::spawn(move || {
            let (dist, mut rng) = (Uniform::new(0.0, 1.0), rand::thread_rng());
            let mut img_local = img_clone.lock().unwrap();

            let (start_row, end_row) = (n*chunk_rows, n*chunk_rows + chunk_rows);
            let (start_col, end_col) = (0, chunk_cols);

            for i in start_row..end_row {
                for j in start_col..end_col {
                    let px = process_pixel(i, j, &dist, &mut rng, &cam, &obj);
                    img_local.set(i, j, px);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(img).ok().and_then(|mutex| mutex.into_inner().ok()).unwrap()
}


