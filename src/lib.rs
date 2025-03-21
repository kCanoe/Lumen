// modules
pub mod materials;
pub mod math;
pub mod objects;

pub mod camera;
pub mod image;
pub mod render;

// flatten
pub use camera::Camera;
pub use camera::CameraBuilder;
pub use materials::Material;
pub use objects::ObjectList;
pub use render::Renderer;
