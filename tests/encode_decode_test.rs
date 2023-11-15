use filegram::{decode::decode_to_data, encode::encode_to_rgb};
use image::ImageFormat;
use std::{
    fs::File,
    io::{BufReader, Cursor, Read},
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
    let rgb = encode_to_rgb(&mut file, file_size);

    let inner = Vec::new();
    let mut buf = Cursor::new(inner);
    rgb.write_to(&mut buf, ImageFormat::Png).unwrap();
    let data = decode_to_data(buf);

    assert_eq!(original_data, data)
}
