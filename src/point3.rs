use std::fmt;
use std::ops::{Add, Sub, Mul, Div};

use crate::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3 { x: x, y: y, z: z }
    }
}

impl fmt::Display for Point3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add for Point3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<Vec3> for Point3 {
    type Output = Self;

    fn add(self, vector: Vec3) -> Self::Output {
        Point3 {
            x: self.x + vector.x,
            y: self.y + vector.y,
            z: self.z + vector.z,
        } 
    }
}


impl Sub for Point3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Self;

    fn sub(self, vector: Vec3) -> Self::Output {
        Point3 {
            x: self.x - vector.x,
            y: self.y - vector.y,
            z: self.z - vector.z,
        } 
    }
}

impl Mul<f64> for Point3 {
    type Output = Self;

    fn mul(self, k: f64) -> Self::Output {
        Point3 {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }
}

impl Mul<Point3>  for f64 {
    type Output = Point3;

    fn mul(self, point: Point3) -> Self::Output {
        Point3 {
            x: self * point.x,
            y: self * point.y,
            z: self * point.z,
        }
    }
}

impl Div<f64> for Point3 {
    type Output = Self;

    fn div(self, k: f64) -> Self::Output {
        Point3 {
            x: self.x / k,
            y: self.y / k,
            z: self.z / k,
        }
    }
}
