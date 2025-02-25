use rand::distributions::{Distribution, Uniform};

use crate::{HitRecord, Ray, Vec3};

pub fn random_double() -> f64 {
    let dist = Uniform::new(0.0, 1.0);
    let mut rng = rand::thread_rng();
    dist.sample(&mut rng)
}

#[derive(Debug, Clone, Copy)]
pub struct Diffuse {
    pub albedo: Vec3,
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    pub refraction: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Diffuse(Diffuse),
    Metal(Metal),
    Dielectric(Dielectric),
}

pub trait Scatter {
    fn scatter(
        &self,
        r: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

impl Scatter for Diffuse {
    fn scatter(
        &self,
        _r: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = record.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() == true {
            scatter_direction = record.normal
        }
        *scattered = Ray::new(record.point, scatter_direction);
        *attenuation = self.albedo.clone();
        return true;
    }
}

impl Scatter for Metal {
    fn scatter(
        &self,
        r: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = Vec3::reflect(r.direction, record.normal);
        reflected = Vec3::unit_vector(reflected)
            + self.fuzz * Vec3::random_unit_vector();
        *scattered = Ray::new(record.point, reflected);
        *attenuation = self.albedo.clone();
        return (scattered.direction * record.normal) > 0.0;
    }
}

impl Scatter for Dielectric {
    fn scatter(
        &self,
        r: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let ri = match record.front_facing {
            true => 1.0 / self.refraction,
            false => self.refraction,
        };
        let unit_direction = Vec3::unit_vector(r.direction);
        let cos_theta = (-1.0 * unit_direction * record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;
        // schlick approximation for dielectric refraction
        let mut r0 = (1.0 - ri) / (1.0 + ri);
        r0 = r0 * r0;
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

impl Scatter for Material {
    fn scatter(
        &self,
        r: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Self::Diffuse(mat) => {
                mat.scatter(r, record, attenuation, scattered)
            }
            Self::Metal(mat) => mat.scatter(r, record, attenuation, scattered),
            Self::Dielectric(mat) => {
                mat.scatter(r, record, attenuation, scattered)
            }
        }
    }
}
