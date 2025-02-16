use std::fmt;

use crate::Point3;
use crate::Vec3;
use crate::Color;

use crate::Sphere;
use crate::HitRecord;
use crate::hit::Hittable;

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

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn ray_color(ray: Ray) -> Color {
        let sphere = Sphere::new(0.4, 0.0, 0.0, -1.0);

        let mut record = HitRecord::new();
        
        let hit: bool = sphere.hit(&ray, &mut record);

        match hit {
            true => {
                Color::from_vec3(0.5 * (record.normal + Vec3::new(1.0, 1.0, 1.0)))
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
