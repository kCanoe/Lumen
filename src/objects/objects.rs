use crate::materials::Material;

use crate::math::*;
use crate::objects::*;

#[derive(Debug, Clone)]
pub struct ObjectList {
    pub objects: Vec<Object>,
}

impl ObjectList {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ObjectList { objects: Vec::new() }
    }

    pub fn add_sphere(
        &mut self,
        r: f64,
        x: f64,
        y: f64,
        z: f64,
        mat: Material,
    ) {
        let position = Vec3::new(x, y, z);
        let sp = Object::Sphere(Sphere::new(r, position, mat));
        self.objects.push(sp);
    }

    pub fn add_cube(
        &mut self,
        length: f64,
        x: f64,
        y: f64,
        z: f64,
        mat: Material,
    ) {
        let center = Vec3::new(x, y, z);
        let cube = Object::Cube(Cube::new(length, center, mat));
        self.objects.push(cube);
    }

    pub fn add_quad(&mut self, q: Vec3, u: Vec3, v: Vec3, mat: Material) {
        let quad = Object::Quad(Quad::new(q, u, v, mat));
        self.objects.push(quad);
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
