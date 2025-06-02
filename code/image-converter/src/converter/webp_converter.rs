use image::DynamicImage;
use image::ImageFormat as ImgFormat;
use crate::converter::formats::ImageFormat;
use crate::converter::errors::ConverterError;

pub fn convert_to_webp(image: Vec<u8>, format: &ImageFormat) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

    // Decode the input image
    let img = image::load_from_memory(&image)?;
    
    // Convert to WEBP format
    let mut output = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut output), ImgFormat::WebP)?;
    
    Ok(output)

}