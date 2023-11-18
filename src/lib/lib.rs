pub mod decode;
pub mod encode;
pub mod encryption;
mod padding;
mod utils;

const IMAGE_WIDTH: usize = 85;
const BUFFER_SIZE: usize = 255;
