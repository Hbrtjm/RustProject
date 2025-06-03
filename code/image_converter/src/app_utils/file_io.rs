use std::fs::File;
use std::path::Path;
use image::{DynamicImage, ImageError, ImageFormat};

pub fn read_image(path: &Path)  -> Result<DynamicImage, ImageError> {
    image::open(path)
}

pub fn write_image(image: &DynamicImage, path: &Path, format: ImageFormat) -> Result<(), Box<dyn std::error::Error>> {
    let fout = File::create(path)?;
    Ok(image.write_to(&mut std::io::BufWriter::new(fout), format)?)
}
