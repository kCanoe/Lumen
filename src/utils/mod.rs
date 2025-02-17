pub mod hit;
pub use hit::{HitRecord, Hittable};

pub mod shapes;
pub use shapes::{Sphere, ObjectList};

pub mod utility;
pub use utility::{random, random_range, degrees_to_radians};


