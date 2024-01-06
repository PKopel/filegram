use std::{
    error::Error,
    io::{BufRead, Seek},
};

use block_padding::UnpadError;
use image::{ImageFormat, RgbImage};

use crate::padding::unpad_block;

pub fn from_file<R: BufRead + Seek>(input: R) -> Result<Vec<u8>, Box<dyn Error>> {
    let img = image::load(input, ImageFormat::Png)?;
    if let Some(img) = img.as_rgb8() {
        Ok(from_rgb(img)?)
    } else {
        Err("Couldn't read image as RGB")?
    }
}

pub fn from_rgb(input_image: &RgbImage) -> Result<Vec<u8>, UnpadError> {
    let mut rows: Vec<Vec<u8>> = input_image
        .enumerate_rows()
        .map(|(_, row)| row.flat_map(|(_, _, rgb)| rgb.0).collect())
        .collect();
    if let Some(last) = rows.last_mut() {
        *last = unpad_block(last)?;
    }
    let data = rows.into_iter().flatten().collect();
    Ok(data)
}
