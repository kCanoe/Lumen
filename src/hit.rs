use crate::Vec3;
use crate::Point3;
use crate::Ray;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            point: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(1.0, 1.0, 1.0),
            t: 0.0,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, record: &mut HitRecord) -> bool;
}
