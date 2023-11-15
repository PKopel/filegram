use std::{
    fs::{self, File},
    io::{BufReader, Error},
    path::Path,
};

use filegram::{decode::decode_to_data, encode::encode_to_rgb};

fn main() -> Result<(), Error> {
    {
        let file = File::open(".ignore/file_large.txt")?;
        let file_size = file.metadata().unwrap().len() as usize;
        let mut file = BufReader::new(file);
        let rgb = encode_to_rgb(&mut file, file_size);
        let path = Path::new("file_large.txt.png");
        rgb.save(path).unwrap();
    }

    {
        let file = File::open("file_large.txt.png")?;
        let data = decode_to_data(BufReader::new(file));
        fs::write("file_large_decoded.txt", data)?;
    }

    Ok(())
}
