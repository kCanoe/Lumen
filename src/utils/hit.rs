use crate::types::*;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    #[allow(dead_code)]
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            point: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(1.0, 1.0, 1.0),
            t: 0.0,
            front_face: true,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        match (ray.direction * outward_normal) < 0.0 {
            true => self.normal = outward_normal,
            false => self.normal = -1.0 * outward_normal,
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &Interval, record: &mut HitRecord) -> bool;
}
