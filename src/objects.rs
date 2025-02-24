use rand::distributions::{ Distribution, Uniform };

use crate::Vec3;
use crate::Ray;
use crate::Interval;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Diffuse(Vec3),
    Metal(Vec3, f64),
    Dielectric(f64),
}

impl Material {
    pub fn scatter(
        &self,
        r: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray
    ) -> bool {
        match self {
            Self::Diffuse(albedo) => {
                let mut scatter_direction = record.normal + Vec3::random_unit_vector(); 
                if scatter_direction.near_zero() == true {
                    scatter_direction = record.normal
                }
                *scattered = Ray::new(record.point, scatter_direction);
                *attenuation = albedo.clone();
                return true;
            }
            Self::Metal(albedo, fuzz) => {
                let mut reflected = Vec3::reflect(r.direction, record.normal);
                reflected = Vec3::unit_vector(reflected)
                    + *fuzz * Vec3::random_unit_vector();
                *scattered = Ray::new(record.point, reflected);
                *attenuation = albedo.clone();
                return (scattered.direction * record.normal) > 0.0;
            }
            Self::Dielectric(refraction_idx) => {
                *attenuation = Vec3::new(1.0, 1.0, 1.0);
                let ri = match record.front_facing {
                    true => 1.0 / refraction_idx,
                    false => *refraction_idx,
                };
                let unit_direction = Vec3::unit_vector(r.direction);
                let cos_theta = (-1.0 * unit_direction * record.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                let cannot_refract = ri * sin_theta > 1.0;
                // schlick approximation for dielectric refraction
                let mut r0 = (1.0 - ri) / (1.0 + ri);
                r0 = r0*r0;
                r0 = r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5);
                let schlick = r0 > random_double();
                let direction = match cannot_refract || schlick {
                    true => Vec3::reflect(unit_direction, record.normal),
                    false => Vec3::refract(unit_direction, record.normal, ri),
                };
                *scattered = Ray::new(record.point, direction);
                return true;
            }
        }
    }
}

pub fn random_double() -> f64 {
    let dist = Uniform::new(0.0, 1.0);
    let mut rng = rand::thread_rng();   
    dist.sample(&mut rng)
}

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
            mat: Material::Diffuse(Vec3::new(0.0, 0.0, 0.0)),
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
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub radius: f64,
    pub center: Vec3,
    pub mat: Material,
}

impl Sphere {
    pub fn new(radius: f64, center: Vec3, mat: Material) -> Self {
        Sphere { radius, center, mat }
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



