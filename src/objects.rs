use crate::materials::{Diffuse, Material};
use crate::ray::{Interval, Ray};
use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub mat: Material,
    pub front_facing: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            mat: Material::Diffuse(Diffuse {
                albedo: Vec3::new(0.0, 0.0, 0.0),
            }),
            front_facing: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        match (ray.direction * outward_normal) < 0.0 {
            true => {
                self.normal = outward_normal;
                self.front_facing = true;
            }
            false => {
                self.normal = -1.0 * outward_normal;
                self.front_facing = false;
            }
        };
    }
}

#[derive(Debug, Clone)]
pub struct ObjectList {
    pub objects: Vec<Sphere>,
}

impl ObjectList {
    #[allow(dead_code)]
    pub fn new(objects: Vec<Sphere>) -> Self {
        ObjectList { objects }
    }

    pub fn add(&mut self, obj: Sphere) {
        self.objects.push(obj);
    }
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub radius: f64,
    pub center: Vec3,
    pub mat: Material,
}

impl Sphere {
    pub fn new(radius: f64, center: Vec3, mat: Material) -> Self {
        Sphere {
            radius,
            center,
            mat,
        }
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
        record.mat = self.mat;

        let outward_normal = (r.at(root) - self.center) / self.radius;

        record.set_face_normal(r, outward_normal);

        return true;
    }
}
