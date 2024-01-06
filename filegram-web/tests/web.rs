#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use filegram::{decode, encode};
use getrandom;

use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn encode_decode_test() {
    let mut original_data = [0u8; 1000];
    getrandom::getrandom(&mut original_data).unwrap();
    let original_data = original_data.to_vec();
    let rgb = encode::from_slice(&original_data);
    let data = decode::from_rgb(&rgb).unwrap();
    assert_eq!(original_data, data)
}
