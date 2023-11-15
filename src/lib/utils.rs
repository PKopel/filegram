use std::io::{self, Read};

pub fn read_exact(file: &mut impl Read, mut buffer: &mut [u8]) -> io::Result<usize> {
    let mut sum = 0;
    while !buffer.is_empty() {
        let n = file.read(buffer)?;
        if n == 0 {
            break;
        }
        buffer = &mut buffer[n..];
        sum += n;
    }
    Ok(sum)
}
