use std::thread;

use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

use crate::camera::Camera;
use crate::image::Image;
use crate::image::Pixel;
use crate::materials::Scatter;
use crate::objects::HitRecord;
use crate::objects::ObjectList;
use crate::objects::Physical;
use crate::ray::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Renderer {
    camera: Camera,
    objects: ObjectList,
    thread_count: usize,
}

pub struct ChunkRenderer {
    objs: ObjectList,
    pixel_origin: Vec3,
    pixel_du: Vec3,
    pixel_dv: Vec3,
    cam_position: Vec3,
    samples: usize,
    depth: usize,
    rng: ThreadRng,
    dist: Uniform<f64>,
}

impl Renderer {
    pub fn new(
        camera: Camera,
        objects: ObjectList,
        thread_count: usize,
    ) -> Self {
        Self {
            camera,
            objects,
            thread_count,
        }
    }

    pub fn render(self) -> Image {
        let mut handles = Vec::with_capacity(self.thread_count);
        let chunk_rows = self.camera.image_height / self.thread_count;
        for n in 0..self.thread_count {
            let objects = self.objects.clone();
            let handle = thread::spawn(move || {
                let mut renderer = ChunkRenderer::new(objects, &self.camera);
                renderer.render_chunk(
                    n * chunk_rows,
                    (n + 1) * chunk_rows,
                    self.camera.image_width,
                )
            });
            handles.push(handle);
        }
        let pixel_count = self.camera.image_width * self.camera.image_height;
        let mut image: Vec<Pixel> = Vec::with_capacity(pixel_count);
        for handle in handles {
            let pixels = handle.join().unwrap();
            for pixel in pixels {
                image.push(pixel);
            }
        }
        Image {
            data: image,
            rows: self.camera.image_height,
            cols: self.camera.image_width,
        }
    }
}

impl ChunkRenderer {
    pub fn new(objects: ObjectList, camera: &Camera) -> Self {
        Self {
            objs: objects,
            pixel_origin: camera.pixel_origin,
            pixel_du: camera.pixel_delta_u,
            pixel_dv: camera.pixel_delta_v,
            cam_position: camera.position,
            samples: camera.samples,
            depth: camera.max_depth,
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
        let pixel_center = self.pixel_origin
            + (j as f64 + offset.x) * self.pixel_du
            + (i as f64 + offset.y) * self.pixel_dv;
        let ray_direction = pixel_center - self.cam_position;
        Ray::new(self.cam_position, ray_direction)
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

    fn compute_pixel(&mut self, i: usize, j: usize) -> Pixel {
        let (samples, scale) = (self.samples, 1.0 / self.samples as f64);
        let mut color = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..samples {
            let r = self.get_ray(i, j);
            color += self.cast_ray(r, self.depth);
        }
        Pixel::from_vec(color * scale)
    }

    pub fn render_chunk(
        &mut self,
        row_start: usize,
        row_end: usize,
        cols: usize,
    ) -> Vec<Pixel> {
        let pixel_count = cols * (row_end - row_start);
        let mut pixels = vec![Pixel::new(0, 0, 0); pixel_count];
        for i in row_start..row_end {
            for j in 0..cols {
                pixels[(i - row_start) * cols + j] = self.compute_pixel(i, j);
            }
        }
        pixels
    }
}
