use image_converter::converter::{formats::ImageFormat, main_converter::convert};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Path::new("assets/samples/example.png");
    let output = Path::new("output/example.jpg");
    // Consumption would occur, so I have to borrow the format here, even if it looks odd
    convert(input, output, &ImageFormat::JPEG)?;
    println!("Converted successfully!");
    Ok(())
}

