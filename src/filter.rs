use crate::rendering::{Image, Pixel};
use crate::math::Vec3;

fn sample_average(image: &Image, row: usize, col: usize) -> Pixel {
    let top_edge = row == 0;
    let bottom_edge = row == image.rows-1;
    let left_edge = col == 0;
    let right_edge = col == image.cols-1;

    let scale = match (top_edge || bottom_edge, left_edge || right_edge) {
        (true, true) => 4.0,
        (true, false) | (false, true) => 6.0,
        (false, false) => 9.0,
    };

    let (col_start, col_end) = match (left_edge, right_edge) {
        (true, false) => (1, 3),
        (false, true) => (0, 2),
        _ => (0, 3),
    };

    let (row_start, row_end) = match (top_edge, bottom_edge) {
        (true, false) => (1, 3),
        (false, true) => (0, 2),
        _ => (0, 3),
    };

    let mut sum = Vec3::default();

    for i in row_start..row_end {
        for j in col_start..col_end {
            sum += image.get(row + i - 1, col + j - 1).inner();
        }
    }

    let chunk_average = sum / scale;

    Pixel::from(chunk_average)
}

pub fn blur(image: &mut Image, passes: usize) {
    for _ in 0..passes {
        for row in 0..image.rows {
            for col in 0..image.cols {
                let adjusted = sample_average(&image, row, col);
                image.set(row, col, adjusted);
            }
        }
    }
}
