use crate::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct CameraSettings {
    #[allow(dead_code)]
    pub aspect_ratio: f64,
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub vertical_fov: f64,
    pub focal_length: f64,
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub up: Vec3,
    pub v: Vec3,
    pub u: Vec3,
    pub w: Vec3,
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
    pub fn new(width: usize, height: usize) -> Self {
        CameraSettings {
            aspect_ratio: 16.0 / 9.0,
            viewport_width: 2.0,
            viewport_height: 2.0 * 9.0 / 16.0,
            vertical_fov: 90.0,
            focal_length: 1.0,
            look_from: Vec3::new(-2.0, 2.0, 1.0),
            look_at: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            u: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
            position: Vec3::new(0.0, 0.0, 0.0),
            viewport_u: Vec3::new(0.0, 0.0, 0.0),
            viewport_v: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            pixel_origin: Vec3::new(0.0, 0.0, 0.0),
            image_width: width,
            image_height: height,
            samples: 100,
            sample_scale: 0.01,
            max_depth: 10,
        }
    }

    pub fn initialize(&mut self) {
        self.position = self.look_from;
        self.focal_length = (self.look_from - self.look_at).length();

        let theta = self.vertical_fov * std::f64::consts::PI / 180.0;
        let h = (theta / 2.0).tan();
        self.viewport_height = 2.0 * h * self.focal_length;
        self.viewport_width = self.viewport_height * 16.0 / 9.0;

        self.w = Vec3::unit_vector(self.look_from - self.look_at);
        self.u = Vec3::unit_vector(Vec3::cross(self.up, self.w));
        self.v = Vec3::cross(self.w,  self.u);

        self.viewport_u = self.viewport_width * self.u;
        self.viewport_v = -self.viewport_height * self.v;

        self.pixel_delta_u = self.viewport_u / self.image_width as f64;
        self.pixel_delta_v = self.viewport_v / self.image_height as f64;

        self.pixel_origin = self.position - (self.focal_length * self.w)
            - self.viewport_u / 2.0 - self.viewport_v / 2.0
            + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }
}
