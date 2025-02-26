use std::thread;

use rand::rngs::ThreadRng;

use crate::camera::CameraSettings;
use crate::image::Image;
use crate::image::Pixel;
use crate::objects::HitRecord;
use crate::objects::ObjectList;
use crate::ray::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;

use crate::materials::Scatter;

use rand::distributions::{Distribution, Uniform};

pub struct ChunkRenderer {
    pub objs: ObjectList,
    pub cam: CameraSettings,
    pub rng: ThreadRng,
    pub dist: Uniform<f64>,
}

impl ChunkRenderer {
    pub fn new(objects: ObjectList, camera: CameraSettings) -> Self {
        Self {
            objs: objects,
            cam: camera,
            rng: rand::thread_rng(),
            dist: Uniform::new(0.0, 1.0),
        }
    }

    fn get_ray(&mut self, i: usize, j: usize) -> Ray {
        let offset = Vec3 {
            x: self.dist.sample(&mut self.rng) - 0.5,
            y: self.dist.sample(&mut self.rng) - 0.5,
            z: 0.0,
        };
        let pixel_center = self.cam.pixel_origin
            + (j as f64 + offset.x) * self.cam.pixel_delta_u
            + (i as f64 + offset.y) * self.cam.pixel_delta_v;
        let ray_direction = pixel_center - self.cam.position;
        Ray::new(self.cam.position, ray_direction)
    }

    fn check_hit(&self, r: &Ray) -> (bool, HitRecord) {
        let mut record = HitRecord::new();
        let mut hit = false;
        let mut tmp = HitRecord::new();
        let mut closest = Interval::new(0.001, f64::INFINITY);
        for object in &self.objs.objects {
            if object.hit(r, &closest, &mut tmp) == true {
                hit = true;
                closest.max = tmp.t;
                record = tmp;
            }
        }
        (hit, record)
    }

    fn cast_ray(&self, r: Ray, depth: usize) -> Vec3 {
        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        let (hit, rec) = self.check_hit(&r);
        if hit != true {
            let unit_direction = Vec3::unit_vector(r.direction);
            let a = 0.5 * (unit_direction.y + 1.0);
            return (1.0 - a) * Vec3::new(0.5, 0.7, 1.0)
                + a * Vec3::new(1.0, 1.0, 1.0);
        }
        let mut at = Vec3::new(0.0, 0.0, 0.0);
        let mut scattered =
            Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        match rec.mat.scatter(&r, &rec, &mut at, &mut scattered) {
            true => {
                let cast = self.cast_ray(scattered, depth - 1);
                Vec3::new(at.x * cast.x, at.y * cast.y, at.z * cast.z)
            }
            false => Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn compute_pixel(&mut self, i: usize, j: usize) -> Pixel {
        let mut color = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        for _ in 0..self.cam.samples {
            let r = self.get_ray(i, j);
            color += self.cast_ray(r, self.cam.max_depth);
        }
        Pixel::from_vec(color * self.cam.sample_scale)
    }

    pub fn render_chunk(
        &mut self,
        row_start: usize,
        row_end: usize,
    ) -> Vec<Pixel> {
        let width = self.cam.image_width;
        let pixel_count = width * (row_end - row_start);
        let mut pixels = vec![Pixel::new(0, 0, 0); pixel_count];
        for i in row_start..row_end {
            for j in 0..width {
                pixels[(i-row_start) * width + j]  = self.compute_pixel(i, j);
            }
        }
        pixels
    }
}

pub fn render(
    n_threads: usize,
    camera: CameraSettings,
    objects: ObjectList,
) -> Image {
    let mut handles = Vec::with_capacity(n_threads);
    let chunk_rows = camera.image_height / n_threads;
    for n in 0..n_threads {
        let (obj, cam) = (objects.clone(), camera.clone());
        let handle = thread::spawn(move || {
            let mut renderer = ChunkRenderer::new(obj, cam);
            let pixels = renderer.render_chunk(n * chunk_rows, (n+1) * chunk_rows);
            pixels
        });
        handles.push(handle);
    }
    let pixel_count = camera.image_width * camera.image_height;
    let mut image: Vec<Pixel> = Vec::with_capacity(pixel_count);
    for handle in handles {
        let pixels = handle.join().unwrap();
        for pixel in pixels {
            image.push(pixel);
        }
    }
    Image {
        data: image,
        rows: camera.image_height,
        cols: camera.image_width,
    }
}
