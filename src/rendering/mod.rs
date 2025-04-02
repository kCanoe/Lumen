// modules
pub mod camera;
pub mod image;
pub mod renderer;

// flatten
pub use image::Image;
pub use image::Pixel;

pub use camera::Camera;
pub use camera::CameraBuilder;

pub use renderer::Renderer;
