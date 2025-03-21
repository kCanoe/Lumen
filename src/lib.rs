// modules
pub mod materials;
pub mod math;
pub mod objects;
pub mod rendering;

// flatten
pub use materials::Material;
pub use objects::ObjectList;
pub use rendering::Renderer;
pub use rendering::Camera;
pub use rendering::CameraBuilder;

