use std::fmt;

use crate::types::{Point3, Vec3, Color, Interval};

use crate::utils::{HitRecord, Hittable, ObjectList};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    const IDENTITY: Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
    const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0 };
    const SKY: Color = Color { r: 0.5, g: 0.7, b: 1.0 };

    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn ray_color(ray: Ray, depth: i32, objects: &ObjectList) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        

        let mut record = HitRecord::new();
        
        let ray_t = Interval::new(0.001, f64::INFINITY);

        let hit: bool = objects.hit(&ray, &ray_t, &mut record);

        match hit {
            true => {
                let direction = record.normal + Vec3::random_unit_vector();
                let bounce = Ray::new(record.point, direction);
                0.5 * Self::ray_color(bounce, depth - 1, objects)
            }
            false => {
                let unit_direction = Vec3::unit_vector(ray.direction);
                let a: f64 = 0.5 * (unit_direction.y + 1.0);
                (1.0-a) * Self::SKY + a * Self::WHITE
            }
        }
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Origin: {} Direction: {}", self.origin, self.direction)
    }
}
