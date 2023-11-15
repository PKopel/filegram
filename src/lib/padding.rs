use block_padding::{
    generic_array::{typenum::U255, GenericArray},
    AnsiX923, Padding,
};

use crate::BUFFER_SIZE;

pub fn pad_block(data: Vec<u8>) -> Vec<u8> {
    let mut block: GenericArray<u8, U255> = GenericArray::clone_from_slice(&[0u8; BUFFER_SIZE]);
    let data_len = data.len();
    block[..data_len].copy_from_slice(&data);
    AnsiX923::pad(&mut block, data_len);
    block.to_vec()
}

pub fn unpad_block(data: &Vec<u8>) -> Vec<u8> {
    let mut block: GenericArray<u8, U255> = GenericArray::clone_from_slice(&[0u8; BUFFER_SIZE]);
    let data_len = data.len();
    block[..data_len].copy_from_slice(data);
    let data = AnsiX923::unpad(&block).unwrap();
    data.to_vec()
}
