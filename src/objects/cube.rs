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
    sides: Vec<Quad>,
}

impl Cube {
    pub fn new(length: f64, center: Vec3, mat: Material) -> Self {
        let far_corner =
            Vec3::new(center.x + length, center.y + length, center.z + length);
        let mut sides: Vec<Quad> = Vec::with_capacity(6);
        sides.push(Quad::new(
            center,
            Vec3::new(0.0, length, 0.0),
            Vec3::new(0.0, 0.0, length),
            mat,
        ));
        sides.push(Quad::new(
            center,
            Vec3::new(0.0, length, 0.0),
            Vec3::new(length, 0.0, 0.0),
            mat,
        ));
        sides.push(Quad::new(
            center,
            Vec3::new(length, 0.0, 0.0),
            Vec3::new(0.0, 0.0, length),
            mat,
        ));
        sides.push(Quad::new(
            far_corner,
            Vec3::new(0.0, -length, 0.0),
            Vec3::new(0.0, 0.0, -length),
            mat,
        ));
        sides.push(Quad::new(
            far_corner,
            Vec3::new(0.0, -length, 0.0),
            Vec3::new(-length, 0.0, 0.0),
            mat,
        ));
        sides.push(Quad::new(
            far_corner,
            Vec3::new(-length, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -length),
            mat,
        ));
        Self {
            length,
            center,
            mat,
            sides,
        }
    }

    fn get_quads(&self) -> &Vec<Quad> {
        &self.sides
    }
}

impl Physical for Cube {
    fn hit(&self, r: &Ray, rt: &Interval, record: &mut HitRecord) -> bool {
        let quads = self.get_quads();
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
