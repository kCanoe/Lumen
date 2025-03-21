use std::sync::Arc;
use std::thread;

use crate::math::*;
use crate::objects::*;
use crate::materials::Scatter;

use super::image::*;
use super::Camera;

pub struct Renderer {
    camera: Arc<Camera>,
    objects: Arc<ObjectList>,
    thread_count: usize,
}

impl Renderer {
    pub fn new(
        camera: Camera,
        objects: ObjectList,
        thread_count: usize,
    ) -> Self {
        Self {
            camera: Arc::new(camera),
            objects: Arc::new(objects),
            thread_count,
        }
    }

    pub fn render(&self) -> Image {
        let (w, h) = (self.camera.image_width, self.camera.image_height);
        let (chunk_rows, chunk_cols) = (h / self.thread_count, w);
        let handles = (0..self.thread_count).map(|n| {
            let renderer = ChunkRenderer::new(&self.objects, &self.camera);
            let (start_row, end_row) = (n * chunk_rows, (n + 1) * chunk_rows);
            let (start_col, end_col) = (0, chunk_cols);
            thread::spawn(move || {
                renderer.render_chunk(start_row, end_row, start_col, end_col)
            })
        });
        let mut result = Image::new(w, h);
        for handle in handles {
            let pixels = handle.join().unwrap();
            for pixel in pixels {
                result.data.push(pixel);
            }
        }
        result
    }
}

struct ChunkRenderer {
    objs: Arc<ObjectList>,
    cam: Arc<Camera>,
}

impl ChunkRenderer {
    pub fn new(objects: &Arc<ObjectList>, camera: &Arc<Camera>) -> Self {
        Self {
            objs: Arc::clone(objects),
            cam: Arc::clone(camera),
        }
    }

    fn indicies(
        row_start: usize,
        row_end: usize,
        col_start: usize,
        col_end: usize,
    ) -> Vec<(f64, f64)> {
        (row_start..row_end)
            .flat_map(|r| {
                (col_start..col_end).map(move |c| (r as f64, c as f64))
            })
            .collect::<Vec<_>>()
    }

    fn get_rays(&self, indicies: &[(f64, f64)], rays: &mut [Ray]) {
        let pixel_count = indicies.len();
        let default_direction = self.cam.pixel_origin - self.cam.position;
        let (du, dv) = (self.cam.pixel_delta_u, self.cam.pixel_delta_v);
        for i in 0..pixel_count {
            let direction_shift = dv * indicies[i].0 + du * indicies[i].1;
            rays[i].direction = default_direction + direction_shift;
        }
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

    fn lerp(r: &Ray) -> Vec3 {
        let unit_direction = Vec3::unit_vector(r.direction);
        let a = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - a) * Vec3::new(0.5, 0.7, 1.0)
            + a * Vec3::new(1.0, 1.0, 1.0);
    }

    fn cast_ray(&self, r: &Ray, depth: usize) -> Vec3 {
        if depth <= 0 {
            return Vec3::default();
        }
        let (hit, rec) = self.check_hit(&r);
        if !hit {
            return Self::lerp(&r);
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

    fn cast_rays(&self, rays: &[Ray], colors: &mut [Vec3], max_depth: usize) {
        let pixel_count = rays.len();
        for i in 0..pixel_count {
            colors[i] += self.cast_ray(&rays[i], max_depth);
        }
    }

    pub fn render_chunk(
        &self,
        row_start: usize,
        row_end: usize,
        col_start: usize,
        col_end: usize,
    ) -> Vec<Pixel> {
        let (samples, depth) = (self.cam.samples, self.cam.max_depth);
        let scale = 1.0 / self.cam.samples as f64;
        let position = self.cam.position;
        let pixel_count = (row_end - row_start) * (col_end - col_start);
        let indicies = Self::indicies(row_start, row_end, col_start, col_end);
        let mut colors = vec![Vec3::default(); pixel_count];
        let mut rays = vec![Ray::new(position, Vec3::default()); pixel_count];

        for _ in 0..samples {
            self.get_rays(&indicies[..], &mut rays[..]);
            self.cast_rays(&rays[..], &mut colors[..], depth);
        }

        for i in 0..pixel_count {
            colors[i] *= scale;
        }

        colors.into_iter()
            .map(|c| Pixel::from(c))
            .collect::<Vec<_>>()
    }
}
