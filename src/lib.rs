// modules
pub mod materials;
pub mod math;
pub mod objects;
pub mod rendering;
pub mod runtime;

// flatten
pub use materials::Material;
pub use objects::ObjectList;
pub use rendering::Camera;
pub use rendering::CameraBuilder;
pub use rendering::Renderer;
pub use runtime::*;
