use filegram::{decode, encode};
use std::{
    fs::File,
    io::{BufReader, Read},
};

#[test]
fn encode_decode_test() {
    let file_path = "tests/data/test.txt";
    let mut original_data = Vec::new();
    {
        let file = File::open(file_path).unwrap();
        let mut file_reader = BufReader::new(&file);
        file_reader.read_to_end(&mut original_data).unwrap();
    }

    let file = File::open(file_path).unwrap();
    let file_size = file.metadata().unwrap().len() as usize;
    let mut file = BufReader::new(file);
    let rgb = encode::from_reader(&mut file, file_size);
    let data = decode::from_rgb(&rgb).unwrap();

    assert_eq!(original_data, data)
}
