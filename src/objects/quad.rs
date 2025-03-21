use crate::materials::Material;
use crate::math::*;
use crate::objects::*;

#[derive(Debug, Clone)]
pub struct Quad {
    pub q: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub mat: Material,
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat: Material) -> Self {
        Self { q, u, v, mat }
    }

    fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit = Interval::new(0.0, 1.0);
        if !unit.contains(a) || !unit.contains(b) {
            return false;
        }
        rec.u = a;
        rec.v = b;
        true
    }
}

impl Physical for Quad {
    fn hit(&self, r: &Ray, rt: &Interval, record: &mut HitRecord) -> bool {
        let normal = Vec3::unit_vector(Vec3::cross(self.u, self.v));
        let w = normal / (normal * normal);
        let quot = normal * r.direction;
        if quot.abs() < 1e-8 {
            return false;
        }
        let t = ((normal * self.q) - (normal * r.origin)) / quot;
        if rt.contains(t) == false {
            return false;
        }
        let intersection = r.at(t);
        let planar_hit_vec = intersection - self.q;
        let alpha = w * Vec3::cross(planar_hit_vec, self.v);
        let beta = w * Vec3::cross(self.u, planar_hit_vec);
        if !Self::is_interior(alpha, beta, record) {
            return false;
        }
        record.t = t;
        record.point = intersection;
        record.mat = Some(self.mat);
        record.set_face_normal(r, normal);
        return true;
    }
}
