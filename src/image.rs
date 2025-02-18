use std::fmt;

use crate::Vec3;

#[derive(Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Image {
    pub data: Vec<Vec<Pixel>>,
    pub rows: usize,
    pub cols: usize,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Pixel { r, g, b }
    }

    pub fn to_byte(value: f64) -> u8 {
        (255.999 * value.min(0.0).max(1.0)) as u8
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

impl Image {
    pub fn new(rows: usize, cols: usize) -> Self {
        Image {
            data: vec![vec![Pixel::new(0, 0, 0); cols]; rows],
            rows: rows,
            cols: cols,
        }
    }

    pub fn get(&self, i: usize, j: usize) -> &Pixel {
        &self.data[i][j]
    }

    pub fn set(&mut self, i: usize, j: usize, color: &Vec3) {
        self.data[i][j].r = Pixel::to_byte(color.x);
        self.data[i][j].g = Pixel::to_byte(color.y);
        self.data[i][j].b = Pixel::to_byte(color.z);
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
