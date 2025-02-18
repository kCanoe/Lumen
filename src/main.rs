mod image;
use image::Image;
use image::Pixel;

mod objects;
mod render;
mod camera;

use std::thread;

fn render_pixel(
    i: usize,
    j: usize,
    camera: CameraSettings,
    render: RenderSettings,
    objects: ObjectList,
) -> Pixel {

}

fn render_column(
    col: usize,
    camera: CameraSettings, 
    render: RenderSettings, 
    objects: ObjectList
) -> Vec<Pixel> {
     
}

fn render_image(img: &mut Image) {
    let num_threads = 8;
    let mut handles = Vec::with_capacity(num_threads);

    for _ in 0..num_threads {
        let handle = thread::spawn(move || {
            render_column()
        });
    }

    handles.push(handle);

    let mut results: Vec<Vec<Pixel>> = Vec::with_capacity(num_threads);

    for handle in handles {
        let result = handle.join().unwrap();
        results.push(result);
    }
}

fn main() {
    let mut img = Image::new(512, 512);

    render_image(&mut img);

    img.print();
}

