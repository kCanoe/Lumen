use std::fs::File;
use std::io::prelude::*;

mod vec3;
use vec3::Vec3;

mod point3;
use point3::Point3;

mod image;
use image::Image;

mod color;
use color::Color;

mod ray;
use ray::Ray;

mod camera;
use camera::Camera;

fn main() {
    let mut cam = Camera::default();

    cam.set_aspect(1.0, 1.0);
    cam.set_width(2.0);

    let img_width: i32 = 1024;
    let img_height: i32 = (img_width as f64 / cam.aspect_ratio).round() as i32;

    let mut img = Image::new(img_width as usize, img_height as usize); 

    let viewport_u = Vec3::new(cam.viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -cam.viewport_width, 0.0);

    let pixel_delta_u = viewport_u / img.cols as f64;
    let pixel_delta_v = viewport_v / img.rows as f64;

    let viewport_upper_left: Point3 = cam.position
        - Vec3::new(0.0, 0.0, cam.focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel_origin: Point3 = viewport_upper_left
        + 0.5 * (pixel_delta_u + pixel_delta_v);


    for i in 0..img.cols {
        for j in 0..img.rows {
            let pixel_center = pixel_origin
                + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);

            let ray_direction = Vec3::from_point(pixel_center - cam.position);
            let r = Ray::new(pixel_center, ray_direction);
            
            let pixel_color = Ray::ray_color(r); 

            img.set(j, i, pixel_color);
        }
    }

    println!("{}", img);
}



