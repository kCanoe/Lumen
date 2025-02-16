use std::fmt;

use crate::Point3;
use crate::Vec3;
use crate::Color;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
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

    pub fn at(&self, t: f64) -> Vec3 {
        Vec3::from_point(self.origin + t * self.direction)
    }


    pub fn hit_sphere(&self, center: Point3, radius: f64) -> f64 {
        let oc = Vec3::from_point(center - self.origin);
        
        let a = self.direction * self.direction;
        let h = self.direction * oc;
        let c = (oc * oc) - (radius * radius); 
        let discriminant = h*h - a*c; 
        
        if discriminant < 0.0 {
            return -1.0;
        } else {
            let t: f64 = (h - discriminant.sqrt()) /  a;
            return t;
        }
    } 

    pub fn ray_color(r: Ray) -> Color {
        let origin = Point3::new(0.0, 0.0, -1.0);

        let t = r.hit_sphere(origin, 0.45);

        if t > 0.0 {
            let N = Vec3::unit_vector(r.at(t) - Vec3::from_point(origin));
            let normal_shade = Color::new(N.x+1.0, N.y+1.0, N.z+1.0);
            return 0.5 * normal_shade;
        }

        let unit_direction = Vec3::unit_vector(r.direction);
        let a: f64 = 0.5 * (unit_direction.y + 1.0);

        let start_value = Color::new(1.0, 1.0, 1.0);
        let end_value = Color::new(0.5, 0.7, 1.0); 

        (1.0-a) * start_value + a * end_value
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Origin: {} Direction: {}", self.origin, self.direction)
    }
}
