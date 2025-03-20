use crate::materials::Material;
use crate::objects::objects::Physical;
use crate::objects::record::HitRecord;
use crate::ray::{Interval, Ray};
use crate::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub radius: f64,
    pub center: Vec3,
    pub mat: Material,
}

impl Sphere {
    pub fn new(radius: f64, center: Vec3, mat: Material) -> Self {
        Self {
            radius,
            center,
            mat,
        }
    }
}

impl Physical for Sphere {
    fn hit(&self, r: &Ray, rt: &Interval, record: &mut HitRecord) -> bool {
        let oc = self.center - r.origin;
        let a = r.direction * r.direction;
        let h = r.direction * oc;
        let c = oc * oc - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let mut root = (h - discriminant.sqrt()) / a;
        if rt.surrounds(root) == false {
            root = (h + discriminant.sqrt()) / a;
            if rt.surrounds(root) == false {
                return false;
            }
        }
        record.t = root;
        record.point = r.at(root);
        record.mat = Some(self.mat);
        let outward_normal = (r.at(root) - self.center) / self.radius;
        record.set_face_normal(r, outward_normal);
        return true;
    }
}
