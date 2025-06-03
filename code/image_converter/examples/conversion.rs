use std::path::Path;
use image_converter::converter::{convert, convert_bytes, ImageFormat, ConverterError};
use image_conerter::converter::{jpeg_converter, png_converter, webp_converter};

fn main() -> Result<(), ConverterError> {
    // Example 1: File-to-file conversion using the main dispatcher
    let input_path = Path::new("input.jpg");
    let output_path = Path::new("output.png");
    convert(input_path, output_path, &ImageFormat::PNG)?;
    println!("Converted {} to {}", input_path.display(), output_path.display());

    // Example 2: Bytes-to-bytes conversion
    let input_bytes = std::fs::read("input.jpg")?;
    let png_bytes = convert_bytes(input_bytes, ImageFormat::JPEG, ImageFormat::PNG)?;
    std::fs::write("output_bytes.png", png_bytes)?;
    println!("Converted JPEG bytes to PNG bytes");

    // Example 3: Direct converter usage
    let jpeg_bytes = std::fs::read("photo.jpg")?;
    
    // Convert to multiple formats
    let png_bytes = jpeg_converter::convert_jpeg_to_png(jpeg_bytes.clone())?;
    std::fs::write("photo.png", png_bytes)?;
    
    let webp_bytes = jpeg_converter::convert_jpeg_to_webp(jpeg_bytes.clone())?;
    std::fs::write("photo.webp", webp_bytes)?;
    
    let ico_bytes = jpeg_converter::convert_jpeg_to_ico(jpeg_bytes.clone())?;
    std::fs::write("photo.ico", ico_bytes)?;
    
    println!("Converted JPEG to PNG, WebP, and ICO formats");

    // Example 4: Batch conversion
    let formats = [ImageFormat::PNG, ImageFormat::WEBP, ImageFormat::GIF, ImageFormat::BMP];
    let input_jpeg = std::fs::read("batch_input.jpg")?;
    
    for format in &formats {
        let output_bytes = convert_bytes(input_jpeg.clone(), ImageFormat::JPEG, *format)?;
        let filename = format!("batch_output.{}", format.to_extension());
        std::fs::write(&filename, output_bytes)?;
        println!("Created {}", filename);
    }

    Ok(())
}

// Example helper function for quality settings
fn convert_with_quality(
    input_bytes: Vec<u8>,
    source: ImageFormat,
    target: ImageFormat,
    quality: f32,
) -> Result<Vec<u8>, ConverterError> {
    match (source, target) {
        // For WebP conversions, you might want custom quality
        (_, ImageFormat::WEBP) => {
            // Custom WebP encoding with specified quality
            let dyn_img = image::load_from_memory(&input_bytes)
                .map_err(|e| ConverterError::ConversionError(e.to_string()))?;
            let rgba = dyn_img.to_rgba8();
            let (w, h) = (rgba.width(), rgba.height());
            let raw_pixels = rgba.into_raw();
            
            let encoder = webp::Encoder::from_rgba(&raw_pixels, w as u32, h as u32);
            let webp_mem = encoder.encode(quality);
            Ok(webp_mem.into_vec())
        }
        // For other formats, use standard conversion
        _ => convert_bytes(input_bytes, source, target),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_round_trip_conversion() -> Result<(), ConverterError> {
        // Create a simple test image
        let img = image::RgbaImage::new(100, 100);
        let mut buffer = Vec::new();
        image::DynamicImage::ImageRgba8(img)
            .write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Png)
            .unwrap();
        
        // Convert PNG -> JPEG -> PNG
        let jpeg_bytes = png_converter::convert_png_to_jpeg(buffer.clone())?;
        let png_bytes = jpeg_converter::convert_jpeg_to_png(jpeg_bytes)?;
        
        // Both should be valid images
        assert!(image::load_from_memory(&buffer).is_ok());
        assert!(image::load_from_memory(&png_bytes).is_ok());
        
        Ok(())
    }
    
    #[test]
    fn test_format_detection() {
        assert_eq!(
            ImageFormat::from_extension(Some("test.jpg")),
            Some(ImageFormat::JPEG)
        );
        assert_eq!(
            ImageFormat::from_extension(Some("test.PNG")),
            Some(ImageFormat::PNG)
        );
        assert_eq!(
            ImageFormat::from_extension(Some("test.unknown")),
            None
        );
    }
}