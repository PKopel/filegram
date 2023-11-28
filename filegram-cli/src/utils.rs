use std::io::{BufReader, Error, Read};

pub fn read_to_end<R: Read>(reader: R) -> Result<Vec<u8>, Error> {
    let mut buffer = BufReader::new(reader);
    let mut data = Vec::new();
    buffer.read_to_end(&mut data)?;
    Ok(data)
}
