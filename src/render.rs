use std::thread;

use crate::camera::Camera;
use crate::image::Image;
use crate::image::Pixel;
use crate::materials::Scatter;
use crate::objects::record::HitRecord;
use crate::objects::objects::ObjectList;
use crate::objects::objects::Physical;
use crate::ray::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;

use std::sync::Arc;

pub struct Renderer {
    camera: Arc<Camera>,
    objects: Arc<ObjectList>,
    thread_count: usize,
}

pub struct ChunkRenderer {
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

    pub fn render(self) -> Image {
        let mut handles = Vec::with_capacity(self.thread_count);
        let chunk_rows = self.camera.image_height / self.thread_count;
        for n in 0..self.thread_count {
            let renderer = ChunkRenderer::new(&self.objects, &self.camera);
            let handle = thread::spawn(move || {
                renderer.render_chunk(n * chunk_rows, (n + 1) * chunk_rows)
            });
            handles.push(handle);
        }
        let mut result =
            Image::new(self.camera.image_width, self.camera.image_height);
        for handle in handles {
            let pixels = handle.join().unwrap();
            for pixel in pixels {
                result.data.push(pixel);
            }
        }
        result
    }
}

impl ChunkRenderer {
    #[inline]
    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let ray_direction = self.cam.pixel_origin
            + (j as f64) * self.cam.pixel_delta_u
            + (i as f64) * self.cam.pixel_delta_v
            - self.cam.position;
        Ray::new(self.cam.position, ray_direction)
    }

    #[inline]
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

    #[inline]
    fn cast_ray(&self, r: Ray, depth: usize) -> Vec3 {
        if depth <= 0 {
            return Vec3::default();
        }

        let (hit, rec) = self.check_hit(&r);
        if hit != true {
            let unit_direction = Vec3::unit_vector(r.direction);
            let a = 0.5 * (unit_direction.y + 1.0);
            return (1.0 - a) * Vec3::new(0.5, 0.7, 1.0)
                + a * Vec3::new(1.0, 1.0, 1.0);
        }
        let mut at = Vec3::default();
        let mut scattered = Ray::new(Vec3::default(), Vec3::default());
        let mat = rec.mat.expect("Should collide with soemthing");
        match mat.scatter(&r, &rec, &mut at, &mut scattered) {
            true => {
                let cast = self.cast_ray(scattered, depth - 1);
                Vec3::new(at.x * cast.x, at.y * cast.y, at.z * cast.z)
            }
            false => Vec3::default(),
        }
    }

    fn compute_pixel(&self, i: usize, j: usize) -> Pixel {
        let (samples, scale) =
            (self.cam.samples, 1.0 / self.cam.samples as f64);
        let mut color = Vec3::default();
        for _ in 0..samples {
            let r = self.get_ray(i, j);
            color += self.cast_ray(r, self.cam.max_depth);
        }
        Pixel::from(color * scale)
    }

    pub fn render_chunk(&self, row_start: usize, row_end: usize) -> Vec<Pixel> {
        (row_start..row_end)
            .flat_map(|r| {
                (0..self.cam.image_width)
                    .map(move |c| (&self).compute_pixel(r, c))
            })
            .collect::<Vec<_>>()
    }
}
