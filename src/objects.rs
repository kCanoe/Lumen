use crate::Vec3;
use crate::Ray;
use crate::Interval;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Diffuse(Vec3),
    Metal(Vec3, f64),
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
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub mat: Material,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            mat: Material::Diffuse(Vec3::new(0.0, 0.0, 0.0)),
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



