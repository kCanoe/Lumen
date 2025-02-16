use std::fmt;

use crate::Point3;
use crate::Vec3;
use crate::Color;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub struct Sphere {
    pub radius: f64,
    pub center: Point3,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            point: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(1.0, 1.0, 1.0),
            t: 0.0,
        }
    }
}

impl Sphere {
    pub fn new(radius: f64, cx: f64, cy: f64, cz: f64) -> Self {
        Sphere {
            radius: radius,
            center: Point3::new(cx, cy, cz),
        } 
    }
}

trait Hittable {
    fn hit(&self, ray: &Ray, record: &mut HitRecord) -> bool;
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

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn from_vals(ox: f64, oy: f64, oz: f64, dx: f64, dy: f64, dz: f64) -> Self {
        Ray {
            origin: Point3 { x: ox, y: oy, z: oz },
            direction: Vec3 { x: dx, y: dy, z: dz },
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn ray_color(ray: Ray) -> Color {
        let sphere = Sphere::new(0.4, 0.0, 0.0, -1.0);

        let mut record = HitRecord::new();
        
        let hit: bool = sphere.hit(&ray, &mut record);

        match hit {
            true => {
                let N: Vec3 = 0.5 * (record.normal + Vec3::new(1.0, 1.0, 1.0));
                Color::new(N.x, N.y, N.z)
            }
            false => {
                let unit_direction = Vec3::unit_vector(ray.direction);
                let a: f64 = 0.5 * (unit_direction.y + 1.0);
                let start_value = Color::new(1.0, 1.0, 1.0);
                let end_value = Color::new(0.5, 0.7, 1.0); 
                (1.0-a) * start_value + a * end_value
            }
        }
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Origin: {} Direction: {}", self.origin, self.direction)
    }
}
