use std::io::Read;

use image::{Rgb, RgbImage};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};

use crate::{padding::pad_block, utils::read_exact, BUFFER_SIZE, IMAGE_WIDTH};

fn update_frame(image: &mut RgbImage, data: Vec<u8>, shift: usize) {
    data.chunks(3)
        .map(|chunk| {
            if chunk.len() == 3 {
                chunk.try_into().unwrap()
            } else {
                let mut triple = [0u8; 3];
                triple[..chunk.len()].copy_from_slice(chunk);
                triple
            }
        })
        .map(Rgb)
        .enumerate()
        .for_each(|(i, color)| {
            let i = (i + shift) as i32;
            let (x, y) = (i % IMAGE_WIDTH as i32, i / IMAGE_WIDTH as i32);
            draw_filled_rect_mut(image, Rect::at(x, y).of_size(1, 1), color);
        });
}

pub fn from_reader(mut input: &mut impl Read, file_size: usize) -> RgbImage {
    let height = (file_size / BUFFER_SIZE) + 1;
    let mut image = RgbImage::new(IMAGE_WIDTH as u32, height as u32);

    let mut bytes = [0u8; BUFFER_SIZE];
    let mut shift = 0;

    while let Ok(n) = read_exact(&mut input, &mut bytes) {
        if n == 0 {
            break;
        };
        let block = if n < BUFFER_SIZE {
            pad_block(bytes[..n].to_vec())
        } else {
            bytes.to_vec()
        };
        update_frame(&mut image, block, shift);
        shift += IMAGE_WIDTH;
    }
    image
}

pub fn from_slice(input: &[u8]) -> RgbImage {
    let height = (input.len() / BUFFER_SIZE) + 1;
    let mut image = RgbImage::new(IMAGE_WIDTH as u32, height as u32);

    let mut shift = 0;

    input.chunks(BUFFER_SIZE).for_each(|bytes| {
        let n = bytes.len();
        let block = if n < BUFFER_SIZE {
            pad_block(bytes[..n].to_vec())
        } else {
            bytes.to_vec()
        };
        update_frame(&mut image, block, shift);
        shift += IMAGE_WIDTH;
    });
    image
}
