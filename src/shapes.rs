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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let oc = Vec3::from_point(self.center - ray.origin);
        
        let a = ray.direction * ray.direction;
        let h = ray.direction * oc;
        let c = (oc * oc) - (self.radius * self.radius); 
        let discriminant = h*h - a*c; 

        if discriminant < 0.0 {
            return false;
        }

        let mut root = (h - discriminant.sqrt()) / a;

        if root <= t_min || root >= t_max {
            root = (h + discriminant.sqrt()) / a;
            if root <= t_min || root >= t_max {
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

pub struct ObjectList<'a> {
    pub objects: Vec<&'a dyn Hittable>,
}

impl<'b, 'a> ObjectList<'a> {
    pub fn new() -> Self {
        ObjectList { objects: Vec::new() }
    }

    pub fn add(&mut self, object: &'a dyn Hittable) {
        self.objects.push(object);
    }
}

impl<'a>  Hittable for ObjectList<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new(); 
        let mut hit_anything = false;
        let mut closest = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest, &mut temp_record) == true {
                hit_anything = true;
                closest = temp_record.t;
                *record = temp_record;
            }
        }
        
        hit_anything
    }
}




