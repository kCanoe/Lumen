use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub pixel_origin: Vec3,
    pub image_width: usize,
    pub image_height: usize,
    pub samples: usize,
    pub max_depth: usize,
}

#[derive(Default)]
pub struct CameraBuilder {
    image_width: Option<usize>,
    image_height: Option<usize>,
    vfov: Option<f64>,
    position: Option<Vec3>,
    target: Option<Vec3>,
    up: Option<Vec3>,
    samples: Option<usize>,
    max_depth: Option<usize>,
}

impl CameraBuilder {
    pub fn new() -> Self {
        CameraBuilder::default()
    }

    pub fn resolution(&mut self, width: usize, height: usize) -> &mut Self {
        self.image_width = Some(width);
        self.image_height = Some(height);
        self
    }
    pub fn vfov(&mut self, fov: f64) -> &mut Self {
        self.vfov = Some(fov);
        self
    }
    pub fn position(&mut self, position: Vec3) -> &mut Self {
        self.position = Some(position);
        self
    }
    pub fn target(&mut self, target: Vec3) -> &mut Self {
        self.target = Some(target);
        self
    }
    pub fn upward(&mut self, upward: Vec3) -> &mut Self {
        self.up = Some(upward);
        self
    }
    pub fn samples(&mut self, samples: usize) -> &mut Self {
        self.samples = Some(samples);
        self
    }
    pub fn max_depth(&mut self, depth: usize) -> &mut Self {
        self.max_depth = Some(depth);
        self
    }
    pub fn build(&self) -> Camera {
        let focal_length =
            (self.target.unwrap() - self.position.unwrap()).length();

        let theta = self.vfov.unwrap() * std::f64::consts::PI / 180.0;
        let h = (theta / 2.0).tan();
        let vp_height = 2.0 * h * focal_length;
        let vp_width = vp_height * 16.0 / 9.0;
        let w =
            Vec3::unit_vector(self.position.unwrap() - self.target.unwrap());
        let u = Vec3::unit_vector(Vec3::cross(self.up.unwrap(), w));
        let v = Vec3::cross(w, u);

        let (viewport_u, viewport_v) = (vp_width * u, -vp_height * v);

        let pixel_delta_u = viewport_u / self.image_width.unwrap() as f64;
        let pixel_delta_v = viewport_v / self.image_height.unwrap() as f64;

        let pixel_origin = self.position.unwrap()
            - (focal_length * w)
            - viewport_u / 2.0
            - viewport_v / 2.0
            + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            position: self.position.unwrap(),
            target: self.target.unwrap(),
            image_width: self.image_width.unwrap(),
            image_height: self.image_height.unwrap(),
            pixel_delta_u,
            pixel_delta_v,
            pixel_origin,
            samples: self.samples.unwrap(),
            max_depth: self.max_depth.unwrap(),
        }
    }
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Vec3::new(0.0, 0.0, 0.0),
            target: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            pixel_origin: Vec3::new(0.0, 0.0, 0.0),
            image_width: 0,
            image_height: 0,
            samples: 0,
            max_depth: 0,
        }
    }
}
