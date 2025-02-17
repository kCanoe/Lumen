use crate::types::*;

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
    fn hit(&self, ray: &Ray, ray_t: &Interval, record: &mut HitRecord) -> bool {
        let oc = Vec3::from_point(self.center - ray.origin);
        
        let a = ray.direction * ray.direction;
        let h = ray.direction * oc;
        let c = (oc * oc) - (self.radius * self.radius); 
        let discriminant = h*h - a*c; 

        if discriminant < 0.0 {
            return false;
        }

        let mut root = (h - discriminant.sqrt()) / a;

        if ray_t.surrounds(root) == false {
            root = (h + discriminant.sqrt()) / a;
            if ray_t.surrounds(root) == false {
                return false;
            }
        }

        record.t = root;
        record.point = ray.at(root);

        let outward_normal = Vec3::from_point(
            (ray.at(root) - self.center) / self.radius
        );

        record.set_face_normal(ray, outward_normal);

        return true;
    }
}

pub struct ObjectList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl ObjectList {
    pub fn new() -> Self {
        ObjectList { objects: Vec::new() }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for ObjectList {
    fn hit(&self, ray: &Ray, ray_t: &Interval, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new(); 
        let mut hit_anything = false;
        let mut closest = Interval::new(ray_t.min, ray_t.max);

        for object in &self.objects {
            if object.hit(ray, &closest, &mut temp_record) == true {
                hit_anything = true;
                closest.max = temp_record.t;
                *record = temp_record;
            }
        }
        
        hit_anything
    }
}

