use std::io::{BufRead, Seek};

use image::{ImageFormat, RgbImage};

use crate::padding::unpad_block;

pub fn from_file<R: BufRead + Seek>(input: R, format: ImageFormat) -> Vec<u8> {
    let img = image::load(input, format).unwrap();
    from_rgb(img.as_rgb8().unwrap())
}

pub fn from_rgb(input_image: &RgbImage) -> Vec<u8> {
    let mut rows: Vec<Vec<u8>> = input_image
        .enumerate_rows()
        .map(|(_, row)| row.flat_map(|(_, _, rgb)| rgb.0).collect())
        .collect();
    let last = rows.last_mut().unwrap();
    *last = unpad_block(last);
    rows.into_iter().flatten().collect()
}
