mod utils;

use std::io::Cursor;

use filegram::{decode, encode};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn encode(contents: &[u8]) -> Uint8Array {
    let rgb = encode::from_slice(contents);

    let mut bytes: Vec<u8> = Vec::new();
    rgb.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
        .unwrap();

    Uint8Array::from(&bytes[..])
}

#[wasm_bindgen]
pub fn decode(bytes: &[u8]) -> Uint8Array {
    let contents = decode::from_file(Cursor::new(&bytes[..]));

    Uint8Array::from(&contents[..])
}
