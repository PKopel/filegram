#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use filegram_web::{decode, encode};
use getrandom;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn encode_decode_test() {
    let mut original_data = [0u8; 1000];
    getrandom::getrandom(&mut original_data).unwrap();
    let original_data = original_data.to_vec();

    let rgb = encode(&original_data).to_vec();
    let data = decode(&rgb).to_vec();

    assert_eq!(original_data, data)
}
