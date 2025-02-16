use crate::Vec3;
use crate::Point3;
use crate::Ray;
use crate::Image;

pub struct Camera {
    pub aspect_ratio: f64,
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub image_width: i32,
    pub image_height: i32,
    pub focal_length: f64,
    pub position: Point3,
}

impl Camera {
    pub fn default() -> Self {
        Camera {
            aspect_ratio: 1.0,
            viewport_width: 2.0 * 16.0 / 9.0,
            viewport_height: 2.0,
            image_width: 512,
            image_height: 512,
            focal_length: 1.0,
            position: Point3 { x: 0.0, y: 0.0, z: 0.0 },
        }
    }

    #[allow(dead_code)]
    pub fn new(
        aspect_ratio: f64,
        viewport_width: f64,
        viewport_height: f64, 
        image_width: i32,
        image_height: i32,
        focal_length: f64,
        position: Point3
    ) -> Self {
        Camera {
            aspect_ratio: aspect_ratio,
            viewport_width: viewport_width,
            viewport_height: viewport_height,
            image_width: image_width,
            image_height: image_height,
            focal_length: focal_length,
            position: position,
        }
    }

    #[allow(dead_code)]
    pub fn set_position(&mut self, x: f64, y: f64, z: f64) {
        self.position = Point3::new(x, y, z);
    }

    pub fn set_aspect(&mut self, width: f64, height: f64) {
        self.aspect_ratio = width / height;
    }

    pub fn set_viewport_width(&mut self, new_width: f64) {
        self.viewport_width = new_width;
        self.viewport_height = new_width / self.aspect_ratio;
    }

    #[allow(dead_code)]
    pub fn set_viewport_height(&mut self, new_height: f64) {
        self.viewport_height = new_height;
        self.viewport_width = new_height * self.aspect_ratio;
    }

    pub fn set_image_width(&mut self, new_width: i32) {
        self.image_width = new_width;
        self.image_height = (new_width as f64 / self.aspect_ratio).round() as i32;
    }

    #[allow(dead_code)]
    pub fn set_image_height(&mut self, new_height: i32) {
        self.image_height = new_height;
        self.image_width = (new_height as f64 * self.aspect_ratio).round() as i32;
    }

    pub fn render(&self) -> Image {
        let mut image = Image::new(self.image_width, self.image_height); 

        let viewport_u = Vec3::new(
            self.viewport_width, 
            self.position.y, 
            self.position.z
        );

        let viewport_v = Vec3::new(
            self.position.x,
            -self.viewport_height,
            self.position.z
        );

        let pixel_delta_u = viewport_u / image.cols as f64;
        let pixel_delta_v = viewport_v / image.rows as f64;

        let viewport_upper_left: Point3 = self.position
            - Vec3::new(0.0, 0.0, self.focal_length) 
            - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel_origin: Point3 = viewport_upper_left
            + 0.5 * (pixel_delta_u + pixel_delta_v);

        for i in 0..image.cols {
            for j in 0..image.rows {
                let pixel_center = pixel_origin
                    + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);

                let ray_direction = Vec3::from_point(pixel_center - self.position);
                let r = Ray::new(self.position, ray_direction);

                let pixel_color = Ray::ray_color(r); 

                image.set(j, i, pixel_color);
            }
        }

        image
    }
}
