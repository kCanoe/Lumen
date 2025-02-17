use std::fmt;

use crate::Point3;
use crate::Vec3;
use crate::Color;
use crate::Interval;

use crate::HitRecord;
use crate::ObjectList;
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

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn ray_color(ray: Ray, objects: &ObjectList) -> Color {
        let mut record = HitRecord::new();
        
        let ray_t = Interval::new(0.0, f64::INFINITY);

        let hit: bool = objects.hit(&ray, &ray_t, &mut record);

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
