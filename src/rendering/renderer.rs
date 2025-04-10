use std::sync::Arc;

use crate::materials::Scatter;
use crate::math::*;
use crate::objects::*;

use crate::runtime::Job;
use crate::runtime::Manager;
use crate::runtime::workpool::BaseQueue;
use crate::runtime::WorkConfig;

use super::image::*;
use super::Camera;

pub struct Renderer {
    camera: Arc<Camera>,
    objects: Arc<ObjectList>,
}

struct PixelRenderer {
    objs: Arc<ObjectList>,
    cam: Arc<Camera>,
}

struct RendererWorkConfig;

impl WorkConfig for RendererWorkConfig {
    const THREAD_COUNT: usize = 8;
    const BATCH_COUNT: usize = 512;

    type Input = (usize, usize);
    type Output = Pixel;
    type Job = PixelRenderer; 
    type Queue = BaseQueue<(usize, usize)>; 
}

impl Renderer {
    pub fn new(
        camera: Camera,
        objects: ObjectList,
        ) -> Self {
        Self {
            camera: Arc::new(camera),
            objects: Arc::new(objects),
        }
    }

    pub fn render(&self) -> Image {
        let (w, h) = (self.camera.image_width, self.camera.image_height);
        let renderer = PixelRenderer::new(&self.objects, &self.camera);
        let rendering = Arc::new(renderer);
        let mut indexes = Vec::with_capacity(w * h);
        for i in 0..h {
            for j in 0..w {
                let idx = (i, j);
                indexes.push(idx);
            }
        }
        let manager = Manager::<RendererWorkConfig>::new(&rendering, indexes);
        manager.execute();
        let mut result = Image::new(w, h);
        result.data = manager.join();
        result
    }
}

impl Job<(usize, usize), Pixel> for PixelRenderer {
    fn run(&self, pixel_index: &(usize, usize)) -> Pixel {
        let (i, j) = *pixel_index;
        self.render_pixel(i, j)
    }
}

impl PixelRenderer {
    pub fn new(objects: &Arc<ObjectList>, camera: &Arc<Camera>) -> Self {
        Self {
            objs: Arc::clone(objects),
            cam: Arc::clone(camera),
        }
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let default_direction = self.cam.pixel_origin - self.cam.position;
        let (du, dv) = (self.cam.pixel_delta_u, self.cam.pixel_delta_v);
        let direction_shift = (dv * i as f64) + (du * j as f64);
        Ray::new(self.cam.position, default_direction + direction_shift)
    }

    fn check_hit(&self, r: &Ray) -> (bool, HitRecord) {
        let mut record = HitRecord::default();
        let mut hit = false;
        let mut tmp = HitRecord::default();
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

    fn cast_ray(&self, r: &Ray, depth: usize) -> Vec3 {
        if depth <= 0 {
            return Vec3::default();
        }
        let (hit, rec) = self.check_hit(&r);
        if !hit {
            return lerp(
                &r,
                Vec3::new(0.5, 0.7, 1.0),
                Vec3::new(1.0, 1.0, 1.0),
                );
        }
        let (mut at, mut scattered) = (Vec3::default(), Ray::default());
        if let Some(mat) = rec.mat {
            mat.scatter(&r, &rec, &mut at, &mut scattered);
            let cast = self.cast_ray(&scattered, depth - 1);
            Vec3::new(at.x * cast.x, at.y * cast.y, at.z * cast.z)
        } else {
            Vec3::default()
        }
    }

    pub fn render_pixel(&self, i: usize, j: usize) -> Pixel {
        let (samples, depth) = (self.cam.samples, self.cam.max_depth);
        let scale = 1.0 / self.cam.samples as f64;
        let mut color = Vec3::default();
        for _ in 0..samples {
            let ray = self.get_ray(i, j);
            color += self.cast_ray(&ray, depth);
        }
        Pixel::from(color * scale)
    }
}


