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
        if ray.direction * outward_normal < 0.0 {
            self.normal = outward_normal;
            self.front_facing = true;
        } else {
            self.normal = -1.0 * outward_normal;
            self.front_facing = false;
        }
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
pub enum Object {
    Sphere(Sphere),
    Cube(Cube),
}

#[derive(Debug, Clone)]
pub struct Cube {
    pub length: f64,
    pub center: Vec3,
    pub mat: Material,
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub radius: f64,
    pub center: Vec3,
    pub mat: Material,
}

pub trait Physical {
    fn hit(&self, r: &Ray, rt: &Interval, record: &mut HitRecord) -> bool;
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

impl Sphere {
    pub fn new(radius: f64, center: Vec3, mat: Material) -> Self {
        Self {
            radius,
            center,
            mat,
        }
    }
}

impl Physical for Cube {
    fn hit(&self, r: &Ray, rt: &Interval, record: &mut HitRecord) -> bool {
        // solve for time points t where the ray will intersect with the cube.
        // Early return false if there are no solutions. Otherwise, return true
        // after setting the passed in record.

        

        // set the settings for the record for the intersection with the cube
        // set record.t to the solved t, record.point to r.at(t), and
        // record.mat to self.mat. Set the face normal for the record as well.
        return true;
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
        record.mat = self.mat;
        let outward_normal = (r.at(root) - self.center) / self.radius;
        record.set_face_normal(r, outward_normal);
        return true;
    }
}
