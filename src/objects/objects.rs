use crate::materials::Material;
use crate::ray::{Interval, Ray};
use crate::vec3::Vec3;

use crate::objects::sphere::Sphere;
use crate::objects::quad::Quad;
use crate::objects::cube::Cube;
use crate::objects::record::HitRecord;


#[derive(Debug, Clone)]
pub struct ObjectList {
    pub objects: Vec<Object>,
}

impl ObjectList {
    #[allow(dead_code)]
    pub fn new(objects: Vec<Object>) -> Self {
        ObjectList { objects }
    }

    pub fn add_sphere(&mut self, r: f64, position: Vec3, mat: Material) {
        let sp = Object::Sphere(Sphere::new(r, position, mat));
        self.objects.push(sp);
    }

    pub fn add_quad(&mut self, q: Vec3, u: Vec3, v: Vec3, mat: Material) {
        let quad = Object::Quad(Quad::new(q, u, v, mat));
        self.objects.push(quad);
    }

    pub fn add_cube(&mut self, length: f64, center: Vec3, mat: Material) {
        let cube = Object::Cube(Cube::new(length, center, mat));
        self.objects.push(cube);
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    Sphere(Sphere),
    Cube(Cube),
    Quad(Quad),
}
pub trait Physical {
    fn hit(&self, r: &Ray, rt: &Interval, record: &mut HitRecord) -> bool;
}

impl Physical for Object {
    fn hit(&self, r: &Ray, rt: &Interval, record: &mut HitRecord) -> bool {
        match self {
            Self::Sphere(obj) => obj.hit(r, rt, record),
            Self::Quad(obj) => obj.hit(r, rt, record),
            Self::Cube(obj) => obj.hit(r, rt, record),
        }
    }
}

