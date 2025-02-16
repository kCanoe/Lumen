use crate::Vec3;
use crate::Point3;

pub struct Camera {
    pub aspect_ratio: f64,
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub focal_length: f64,
    pub position: Point3,
}

impl Camera {
    pub fn default() -> Self {
        Camera {
            aspect_ratio: 1.0,
            viewport_width: 2.0 * 16.0 / 9.0,
            viewport_height: 2.0,
            focal_length: 1.0,
            position: Point3 { x: 0.0, y: 0.0, z: 0.0 },
        }
    }

    pub fn new(
        aspect_ratio: f64,
        viewport_width: f64,
        viewport_height: f64, 
        focal_length: f64,
        position: Point3
    ) -> Self {
        Camera {
            aspect_ratio: aspect_ratio,
            viewport_width: viewport_width,
            viewport_height: viewport_height,
            focal_length: focal_length,
            position: position,
        }
    }

    pub fn set_pos(&mut self, p: Point3) {
        self.position = p;
    }

    pub fn set_aspect(&mut self, width: f64, height: f64) {
        self.aspect_ratio = width / height;
    }

    pub fn set_width(&mut self, new_width: f64) {
        self.viewport_width = new_width;
        self.viewport_height = new_width / self.aspect_ratio;
    }

    pub fn set_height(&mut self, new_height: f64) {
        self.viewport_height = new_height;
        self.viewport_width = new_height * self.aspect_ratio;
    }
}
