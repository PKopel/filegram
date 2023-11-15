use std::{
    fs::{self, File},
    io::{BufReader, Error},
    path::Path,
};

use filegram::{decode, encode};
use image::ImageFormat;

fn main() -> Result<(), Error> {
    {
        let file = File::open(".ignore/file_large.txt")?;
        let file_size = file.metadata().unwrap().len() as usize;
        let mut file = BufReader::new(file);
        let rgb = encode::to_rgb(&mut file, file_size);
        let path = Path::new("file_large.txt.png");
        rgb.save(path).unwrap();
    }

    {
        let file = File::open("file_large.txt.png")?;
        let data = decode::from_file(BufReader::new(file), ImageFormat::Png);
        fs::write("file_large_decoded.txt", data)?;
    }

    Ok(())
}
