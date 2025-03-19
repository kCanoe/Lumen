use std::fmt;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign,
};

use rand::distributions::{Distribution, Uniform};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn default() -> Self {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn unit_vector(v: Vec3) -> Self {
        let mg = (v.x.powi(2) + v.y.powi(2) + v.z.powi(2)).sqrt();
        Vec3 {
            x: v.x / mg,
            y: v.y / mg,
            z: v.z / mg,
        }
    }

    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    pub fn clamp(v: Vec3) -> Self {
        Vec3 {
            x: v.x.max(0.0).min(1.0),
            y: v.y.max(0.0).min(1.0),
            z: v.z.max(0.0).min(1.0),
        }
    }

    pub fn random_vector() -> Self {
        let dist = Uniform::new(0.0, 1.0);
        let mut rng = rand::thread_rng();
        Vec3 {
            x: dist.sample(&mut rng),
            y: dist.sample(&mut rng),
            z: dist.sample(&mut rng),
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::unit_vector(Self::random_vector())
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Self {
        let on_unit_sphere = Self::random_unit_vector();

        match on_unit_sphere * normal > 0.0 {
            true => on_unit_sphere,
            false => -1.0 * on_unit_sphere,
        }
    }

    pub fn reflect(v: Vec3, u: Vec3) -> Vec3 {
        v - (2.0 * v * u * u)
    }

    pub fn len_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn refract(uv: Vec3, n: Vec3, eta_etap: f64) -> Vec3 {
        let cos_theta = (-1.0 * uv * n).min(1.0);
        let r_out_perp = eta_etap * (uv + cos_theta * n);
        let r_out_parallel =
            -1.0 * n * (1.0 - r_out_perp.len_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
    }

    pub fn near_zero(&self) -> bool {
        let s = 0.00000001;
        match ((self.x.abs() < s), (self.y.abs() < s), (self.z.abs() < s)) {
            (true, true, true) => true,
            _ => false,
        }
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

// assignment operators

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
