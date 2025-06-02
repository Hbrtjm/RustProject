use std::fs::File;
use std::path::Path;
use image::{DynamicImage, ImageError};

use crate::converter::formats::ImageFormat;
use crate::converter::errors::ConverterError;

pub fn read_image(path: &Path)  -> Result<DynamicImage, ImageError> {
    image::open(path)
}

pub fn write_image(img: &DynamicImage, path: &Path, fmt: &ImageFormat) -> Result<(), Box<dyn std::error::Error>> {
    let fout = File::create(path)?;
    Ok(img.write_to(&mut std::io::BufWriter::new(fout), fmt.to_image_output_format())?)
}
