use std::path::Path;

const DEF_IMG_PATH: &str = "images/emblem-documents.png";
const ZIP_EXTENSIONS: [&str; 6] = ["zip", "tar", "gz", "xz", "jar", "7z"];
const ZIP_IMG_PATH: &str = "images/application-x-zip.png";
const IMG_EXTENSIONS: [&str; 6] = ["png", "jpg", "jpeg", "gif", "svg", "webp"];
const IMG_IMG_PATH: &str = "images/image-x-generic.png";
const PDF_EXTENSION: [&str; 1] = ["pdf"];
const PDF_IMG_PATH: &str = "images/application-pdf.png";

pub fn get_image_for_file(file_name: String) -> &'static str {
    let extension = Path::new(&file_name).extension().unwrap().to_str().unwrap();
    if ZIP_EXTENSIONS.contains(&extension) {
        ZIP_IMG_PATH
    } else if IMG_EXTENSIONS.contains(&extension) {
        IMG_IMG_PATH
    } else if PDF_EXTENSION.contains(&extension) {
        PDF_IMG_PATH
    } else {
        DEF_IMG_PATH
    }
}
