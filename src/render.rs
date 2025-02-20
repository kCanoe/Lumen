use std::thread;
use std::sync::{Arc, Mutex};

use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;

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
    s: usize,
    floats: &Vec<f64>,
    cam: &CameraSettings,
) -> Ray {
    let offset = Vec3::new(floats[s*2] - 0.5, floats[s*2+1] - 0.5, 0.0);

    let pixel_center = cam.pixel_origin
        + (j as f64 + offset.x) * cam.pixel_delta_u
        + (i as f64 + offset.y) * cam.pixel_delta_v;

    let ray_direction = pixel_center - cam.position;

    Ray::new(cam.position, ray_direction) 
}

#[inline]
pub fn cast_ray(
    r: Ray,
    s: usize,
    vecs: &Vec<Vec3>,
    depth: usize,
    objects: &ObjectList
) -> Vec3 {
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
        let direction = record.normal + vecs[s*10 + depth];
        let bounce = Ray::new(record.point, direction);
        return 0.5 * cast_ray(bounce, s, vecs, depth - 1, objects);
    } else {
        let unit_direction = Vec3::unit_vector(r.direction);
        let a = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - a) * Vec3::new(0.5, 0.7, 1.0) + a * Vec3::new(1.0, 1.0, 1.0);
    }
}

pub fn process_pixel(
    i: usize,
    j: usize,
    floats: &Vec<f64>,
    vecs: &Vec<Vec3>,
    camera: &CameraSettings,
    objects: &ObjectList,
) -> Pixel {
    let mut color = Vec3::new(0.0, 0.0, 0.0);

    for s in 0..camera.samples {
        let r = get_ray(i, j, s, floats, camera);
        color += cast_ray(r, s, vecs, camera.max_depth, objects);
    }
    
    Pixel::from_vec(color * camera.sample_scale)
}

pub fn render(n_threads: usize, camera: CameraSettings, objects: ObjectList) -> Image {
    let mut rng = StdRng::from_entropy();
    let r_floats: Vec<f64> = (0..200).map(|_| rng.gen_range(0.0..=1.0)).collect(); 
    let r_vecs: Vec<Vec3> = (0..1100).map(|_| Vec3::random_unit_vector()).collect();

    let a_vecs = Arc::new(r_vecs);
    let a_floats = Arc::new(r_floats);
    let img = Arc::new(Mutex::new(Image::new(camera.image_width, camera.image_height)));
    let (a_cam, a_obj) = (Arc::new(camera), Arc::new(objects));

    let mut handles = Vec::with_capacity(n_threads);
    
    let chunk_rows = camera.image_height / n_threads;
    let chunk_cols = camera.image_width;

    for n in 0..n_threads {
        let img_clone = Arc::clone(&img);
        let cam = Arc::clone(&a_cam);
        let obj = Arc::clone(&a_obj);
        let floats = Arc::clone(&a_floats);
        let vecs = Arc::clone(&a_vecs);
        
        let handle = thread::spawn(move || {
            let (start_row, end_row) = (n*chunk_rows, n*chunk_rows + chunk_rows);
            let (start_col, end_col) = (0, chunk_cols);
            let mut img_local = img_clone.lock().unwrap();

            for i in start_row..end_row {
                for j in start_col..end_col {
                    let px = process_pixel(i, j, &floats, &vecs, &cam, &obj);
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


