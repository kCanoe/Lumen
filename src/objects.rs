use crate::Vec3;
use crate::Ray;
use crate::Interval;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        match (ray.direction * outward_normal) < 0.0 {
            true => self.normal = outward_normal,
            false => self.normal = -1.0 * outward_normal,
        };
    }
}

#[derive(Debug, Clone)]
pub struct ObjectList {
    pub objects: Vec<Sphere>,
}

impl ObjectList {
    pub fn new(objects: Vec<Sphere>) -> Self {
        ObjectList { objects }
    }
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub radius: f64,
    pub center: Vec3,
}

impl Sphere {
    pub fn new(radius: f64, center: Vec3) -> Self {
        Sphere { radius, center }
    }

    pub fn hit(&self, r: &Ray, rt: &Interval, record: &mut HitRecord) -> bool {
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

        let outward_normal = (r.at(root) - self.center) / self.radius;

        record.set_face_normal(r, outward_normal);

        return true;
    }
}



