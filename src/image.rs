use std::fmt::Formatter;

use crate::vec3::Vec3;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Pixel(Vec3);

pub struct Image {
    pub data: Vec<Pixel>,
    pub rows: usize,
    pub cols: usize,
}

impl Pixel {
    pub fn default() -> Self {
        Self(Vec3::default())
    }

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        let r = (255.999 * self.0.x.clamp(0.0, 1.0)) as u8;
        let g = (255.999 * self.0.y.clamp(0.0, 1.0)) as u8;
        let b = (255.999 * self.0.z.clamp(0.0, 1.0)) as u8;
        (r, g, b)
    }
}

impl From<Vec3> for Pixel {
    fn from(v: Vec3) -> Self {
        Self(v)
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, g, b) = self.to_rgb();
        write!(f, "{} {} {}", r, g, b)
    }
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Image {
            data: vec![Pixel::default(); width * height],
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
        let data = (0..self.rows)
            .map(|r| {
                (0..self.cols)
                    .map(|c| format!("{}", self.get(r, c)))
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "P3\n{} {}\n255\n{}", self.cols, self.rows, data)
    }
}
