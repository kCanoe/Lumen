// modules
pub mod record;

pub mod objects;

pub mod sphere;
pub mod cube;
pub mod quad;

// flatten
pub use record::HitRecord;

pub use objects::ObjectList;
pub use objects::Physical;

pub use sphere::Sphere;
pub use cube::Cube;
pub use quad::Quad;
