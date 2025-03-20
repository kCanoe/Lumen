use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub u: f64,
    pub v: f64,
    pub t: f64,
    pub mat: Option<Material>,
    pub front_facing: bool,
}

impl HitRecord {
    pub fn default() -> Self {
        HitRecord {
            point: Vec3::default(),
            normal: Vec3::default(),
            u: 0.0,
            v: 0.0,
            t: 0.0,
            mat: None,
            front_facing: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        let normal_direction = ray.direction * outward_normal < 0.0;
        (self.normal, self.front_facing) = match normal_direction {
            true => (outward_normal, true),
            false => (-1.0 * outward_normal, false),
        }
    }
}
