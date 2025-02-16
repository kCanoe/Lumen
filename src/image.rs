use std::fmt;

use crate::Color;

pub struct Image {
    pixels: Vec<Vec<Color>>,
    pub rows: usize,
    pub cols: usize,
}

impl Image {
    pub fn new(width: i32, height: i32) -> Self {
        let (rows, cols) = (height as usize, width as usize);
        Image {
            pixels: vec![vec![Color::new(0.0, 0.0, 0.0); cols]; rows],
            rows: rows,
            cols: cols,
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: Color) {
        self.pixels[row][col] = value;
    }

    #[allow(dead_code)]
    pub fn get(&self, row: usize, col: usize) -> Option<&Color> {
        self.pixels.get(row).and_then(|r| r.get(col))
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Color> {
        self.pixels.get_mut(row).and_then(|r| r.get_mut(col))
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self.pixels.iter()
            .map(|row| {
                row.iter().map(|c| {
                    c.to_string() 
                })
                .collect::<Vec<String>>()
                .join(" ") 
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "P3\n{} {}\n255\n{}", self.cols, self.rows, s)
    }
}
