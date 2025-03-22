use crate::materials::Material;
use crate::math::*;

use super::HitRecord;
use super::Physical;
use super::Quad;

#[derive(Debug, Clone)]
pub struct Cube {
    pub length: f64,
    pub center: Vec3,
    pub mat: Material,
}

impl Cube {
    pub fn new(length: f64, center: Vec3, mat: Material) -> Self {
        Self {
            length,
            center,
            mat,
        }
    }
}

impl Physical for Cube {
    fn hit(&self, r: &Ray, rt: &Interval, record: &mut HitRecord) -> bool {
        let furthest_corner = Vec3 {
            x: self.center.x + 1.0,
            y: self.center.y + 1.0,
            z: self.center.z + 1.0,
        };
        let q1 = Quad::new(
            self.center,
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            self.mat,
        );
        let q2 = Quad::new(
            self.center,
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            self.mat,
        );
        let q3 = Quad::new(
            self.center,
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            self.mat,
        );
        let q4 = Quad::new(
            furthest_corner,
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
            self.mat,
        );
        let q5 = Quad::new(
            furthest_corner,
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            self.mat,
        );
        let q6 = Quad::new(
            furthest_corner,
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
            self.mat,
        );

        let quads = vec![q1, q2, q3, q4, q5, q6];

        let mut tmp = HitRecord::default();
        let mut hit = false;
        let mut closest_q = 0;
        record.t = std::f64::MAX;
        for (q_idx, quad) in quads.iter().enumerate() {
            if quad.hit(r, rt, &mut tmp) {
                hit = true;
                if tmp.t < record.t {
                    record.t = tmp.t;
                    closest_q = q_idx
                }
            }
        }
        if !hit {
            return false;
        }
        let cross = Vec3::cross(quads[closest_q].u, quads[closest_q].v);
        let normal = Vec3::unit_vector(cross);
        record.set_face_normal(r, normal);
        record.point = r.at(record.t);
        record.mat = Some(self.mat);
        return true;
    }
}
