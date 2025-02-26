use crate::vec3::Vec3;
use std::fmt;

#[derive(Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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

    pub fn print(&self) {
        print!("P3\n{} {}\n255\n", self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols - 1 {
                print!("{} ", self.get(i, j));
            }
            print!("{}\n", self.get(i, self.cols - 1));
        }
    }
}
