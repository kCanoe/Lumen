use crate::Vec3;
use crate::Point3;
use crate::Ray;
use crate::hit::Hittable;
use crate::HitRecord;

pub struct Sphere {
    pub radius: f64,
    pub center: Point3,
}

impl Sphere {
    pub fn new(radius: f64, cx: f64, cy: f64, cz: f64) -> Self {
        Sphere {
            radius: radius,
            center: Point3::new(cx, cy, cz),
        } 
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, record: &mut HitRecord) -> bool {
        let oc = Vec3::from_point(self.center - ray.origin);
        
        let a = ray.direction * ray.direction;
        let h = ray.direction * oc;
        let c = (oc * oc) - (self.radius * self.radius); 
        let discriminant = h*h - a*c; 

        if discriminant < 0.0 {
            return false;
        }

        let root = (h - discriminant.sqrt()) / a;

        record.t = root;
        record.point = ray.at(root);
        record.normal = Vec3::from_point((ray.at(root) - self.center) / self.radius);

        return true;
    }
}
