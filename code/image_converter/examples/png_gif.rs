use image_converter::converter::{formats::ImageFormat, main_converter::convert};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Path::new("assets/samples/algebra.png");
    convert(input, &ImageFormat::GIF)?;
    println!("Converted successfully!");
    Ok(())
}