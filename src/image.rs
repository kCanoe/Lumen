use std::fmt::Formatter;

use crate::vec3::Vec3;
use std::fmt;

#[derive(Clone)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

pub struct Image {
    pub data: Vec<Pixel>,
    pub rows: usize,
    pub cols: usize,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Pixel { r, g, b }
    }

    pub fn to_byte(value: f64) -> u8 {
        (255.999 * value.max(0.0).min(1.0)) as u8
    }

    pub fn from_vec(v: Vec3) -> Self {
        Pixel {
            r: Self::to_byte(v.x),
            g: Self::to_byte(v.y),
            b: Self::to_byte(v.z),
        }
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Image {
            data: vec![Pixel::new(0, 0, 0); width * height],
            rows: height,
            cols: width,
        }
    }

    pub fn get(&self, i: usize, j: usize) -> &Pixel {
        &self.data[i * self.cols + j]
    }

    pub fn set(&mut self, i: usize, j: usize, p: Pixel) {
        self.data[i * self.cols + j] = p;
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let data = (0..self.rows).map(|r| {
            (0..self.cols)
                .map(|c| format!("{}", self.get(r, c)))
                .collect::<Vec<String>>().join(" ")
        })
        .collect::<Vec<String>>().join("\n");
        write!(f, "P3\n{} {}\n255\n{}", self.cols, self.rows, data)
    } 
}


