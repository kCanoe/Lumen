use crate::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct CameraSettings {
    pub aspect_ratio: f64,
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub focal_length: f64,
    pub viewport_u: Vec3,
    pub viewport_v: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub pixel_origin: Vec3,
    pub position: Vec3,
    pub image_width: usize,
    pub image_height: usize,
    pub samples: usize,
    pub sample_scale: f64,
    pub max_depth: usize,
}

impl CameraSettings {
    pub fn new() -> Self {
        CameraSettings {
            aspect_ratio: 16.0 / 9.0,
            viewport_width: 2.0,
            viewport_height: 2.0 * 9.0 / 16.0,
            focal_length: 1.0,
            position: Vec3::new(0.0, 0.0, 0.0),
            viewport_u: Vec3::new(0.0, 0.0, 0.0),
            viewport_v: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            pixel_origin: Vec3::new(0.0, 0.0, 0.0),
            image_width: 256,
            image_height: 144,
            samples: 100,
            sample_scale: 0.01,
            max_depth: 10,
        }
    }

    pub fn initialize(&mut self) {
        self.viewport_u = Vec3 {
            x: self.viewport_width,
            y: self.position.y,
            z: self.position.z
        };

        self.viewport_v = Vec3 {
            x: self.position.x,
            y: -self.viewport_height,
            z: self.position.z,
        };

        self.pixel_delta_u = self.viewport_u / self.image_width as f64;
        self.pixel_delta_v = self.viewport_v / self.image_height as f64;

        self.pixel_origin = self.position
            - Vec3::new(0.0, 0.0, self.focal_length)
            - self.viewport_u / 2.0 - self.viewport_v / 2.0
            + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }
}
