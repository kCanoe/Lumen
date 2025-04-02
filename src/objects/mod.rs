// modules
pub mod record;

pub mod objects;

pub mod cube;
pub mod quad;
pub mod sphere;

// flatten
pub use record::HitRecord;

pub use objects::ObjectList;
pub use objects::Physical;

pub use cube::Cube;
pub use quad::Quad;
pub use sphere::Sphere;
