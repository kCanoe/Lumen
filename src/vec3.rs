use std::fmt;
use std::ops::{Add, Sub, Mul, Div};

use crate::Point3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn from_point(p: Point3) -> Self {
        Vec3 { x: p.x, y: p.y, z: p.z }
    }

    pub fn unit_vector(v: Vec3) -> Self {
        let mg = (v.x.powi(2) + v.y.powi(2) + v.z.powi(2)).sqrt();
        Vec3 { x: v.x / mg, y: v.y / mg, z: v.z / mg}
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// operation implementations

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// dot product between two Vec3

impl Mul for Vec3 {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z 
    }
}

// scalar multiplication and division of Vec3

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, k: f64) -> Self::Output {
        Vec3 {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Self::Output {
        Vec3 {
            x: self * vector.x,
            y: self * vector.y,
            z: self * vector.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, k: f64) -> Self::Output {
        Vec3 {
            x: self.x / k,
            y: self.y / k,
            z: self.z / k,
        }
    }
}



