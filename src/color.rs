use std::fmt;
use std::ops::{Add, Mul};


#[derive(Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    fn clamp(value: f64) -> f64 {
        value.max(0.0).min(1.0)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rbyte = (255.999 * self.r) as u8;
        let gbyte = (255.999 * self.g) as u8;
        let bbyte = (255.999 * self.b) as u8;
        write!(f, "{} {} {}", rbyte, gbyte, bbyte)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Color {
            r: Color::clamp(self.r + other.r),
            g: Color::clamp(self.g + other.g),
            b: Color::clamp(self.b + other.b),
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, k: f64) -> Self::Output {
        Color {
            r: Color::clamp(self.r * k),
            g: Color::clamp(self.g * k),
            b: Color::clamp(self.b * k),
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, color: Color) -> Self::Output {
        Color {
            r: Color::clamp(color.r * self),
            g: Color::clamp(color.g * self),
            b: Color::clamp(color.b * self),
        }
    }
}


