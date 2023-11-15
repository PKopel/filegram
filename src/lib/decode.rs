use std::io::{BufRead, Seek};

use image::{ImageFormat, RgbImage};

use crate::padding::unpad_block;

pub fn decode_to_data<R: BufRead + Seek>(file: R) -> Vec<u8> {
    let img = image::load(file, ImageFormat::Png).unwrap();
    rgb_to_data(img.as_rgb8().unwrap())
}

fn rgb_to_data(input_image: &RgbImage) -> Vec<u8> {
    let mut rows: Vec<Vec<u8>> = input_image
        .enumerate_rows()
        .map(|(_, row)| row.flat_map(|(_, _, rgb)| rgb.0).collect())
        .collect();
    let last = rows.last_mut().unwrap();
    *last = unpad_block(last);
    rows.into_iter().flatten().collect()
}
