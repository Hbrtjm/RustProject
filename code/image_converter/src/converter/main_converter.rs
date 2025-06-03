use std::path::Path;
use std::fs;
use crate::app_utils::file_io::{read_image, write_image};
use crate::converter::formats::ImageFormat;
use crate::converter::errors::ConverterError;
use crate::converter::{
    jpeg_converter, png_converter, webp_converter, 
    gif_converter, ico_converter, bmp_converter
};

/// Main conversion function that dispatches to appropriate converters
pub fn convert(input_path: &Path, output_path: &Path, target_format: &ImageFormat) -> Result<(), ConverterError> {
    // Read input file as bytes
    let input_bytes = fs::read(input_path)
        .map_err(|e| ConverterError::ReadError(e.to_string()))?;
    
    // Detect source format from file extension
    let source_format = ImageFormat::from_extension(input_path.to_str())
        .ok_or_else(|| ConverterError::UnsupportedFormat("Cannot determine input format".to_string()))?;
    
    // If source and target are the same, just copy the file
    if source_format == *target_format {
        fs::copy(input_path, output_path)
            .map_err(|e| ConverterError::WriteError(e.to_string()))?;
        return Ok(());
    }
    
    // Dispatch to appropriate converter based on source and target formats
    let converted_bytes = match (source_format, target_format) {
        // From JPEG
        (ImageFormat::JPEG, ImageFormat::PNG) => jpeg_converter::convert_jpeg_to_png(input_bytes)?,
        (ImageFormat::JPEG, ImageFormat::WEBP) => jpeg_converter::convert_jpeg_to_webp(input_bytes)?,
        (ImageFormat::JPEG, ImageFormat::GIF) => jpeg_converter::convert_jpeg_to_gif(input_bytes)?,
        (ImageFormat::JPEG, ImageFormat::ICO) => jpeg_converter::convert_jpeg_to_ico(input_bytes)?,
        (ImageFormat::JPEG, ImageFormat::BMP) => jpeg_converter::convert_jpeg_to_bmp(input_bytes)?,
        
        // From PNG
        (ImageFormat::PNG, ImageFormat::JPEG) => png_converter::convert_png_to_jpeg(input_bytes)?,
        (ImageFormat::PNG, ImageFormat::WEBP) => png_converter::convert_png_to_webp(input_bytes)?,
        (ImageFormat::PNG, ImageFormat::GIF) => png_converter::convert_png_to_gif(input_bytes)?,
        (ImageFormat::PNG, ImageFormat::ICO) => png_converter::convert_png_to_ico(input_bytes)?,
        (ImageFormat::PNG, ImageFormat::BMP) => png_converter::convert_png_to_bmp(input_bytes)?,
        
        // From WebP
        (ImageFormat::WEBP, ImageFormat::PNG) => webp_converter::convert_webp_to_png(input_bytes)?,
        (ImageFormat::WEBP, ImageFormat::JPEG) => webp_converter::convert_webp_to_jpeg(input_bytes)?,
        (ImageFormat::WEBP, ImageFormat::GIF) => webp_converter::convert_webp_to_gif(input_bytes)?,
        (ImageFormat::WEBP, ImageFormat::ICO) => webp_converter::convert_webp_to_ico(input_bytes)?,
        (ImageFormat::WEBP, ImageFormat::BMP) => webp_converter::convert_webp_to_bmp(input_bytes)?,
        
        // From GIF
        (ImageFormat::GIF, ImageFormat::PNG) => gif_converter::convert_gif_to_png(input_bytes)?,
        (ImageFormat::GIF, ImageFormat::JPEG) => gif_converter::convert_gif_to_jpeg(input_bytes)?,
        (ImageFormat::GIF, ImageFormat::WEBP) => gif_converter::convert_gif_to_webp(input_bytes)?,
        (ImageFormat::GIF, ImageFormat::ICO) => gif_converter::convert_gif_to_ico(input_bytes)?,
        (ImageFormat::GIF, ImageFormat::BMP) => gif_converter::convert_gif_to_bmp(input_bytes)?,
        
        // From ICO
        (ImageFormat::ICO, ImageFormat::PNG) => ico_converter::convert_ico_to_png(input_bytes)?,
        (ImageFormat::ICO, ImageFormat::JPEG) => ico_converter::convert_ico_to_jpeg(input_bytes)?,
        (ImageFormat::ICO, ImageFormat::WEBP) => ico_converter::convert_ico_to_webp(input_bytes)?,
        (ImageFormat::ICO, ImageFormat::GIF) => ico_converter::convert_ico_to_gif(input_bytes)?,
        (ImageFormat::ICO, ImageFormat::BMP) => ico_converter::convert_ico_to_bmp(input_bytes)?,
        
        // From BMP
        (ImageFormat::BMP, ImageFormat::PNG) => bmp_converter::convert_bmp_to_png(input_bytes)?,
        (ImageFormat::BMP, ImageFormat::JPEG) => bmp_converter::convert_bmp_to_jpeg(input_bytes)?,
        (ImageFormat::BMP, ImageFormat::WEBP) => bmp_converter::convert_bmp_to_webp(input_bytes)?,
        (ImageFormat::BMP, ImageFormat::GIF) => bmp_converter::convert_bmp_to_gif(input_bytes)?,
        (ImageFormat::BMP, ImageFormat::ICO) => bmp_converter::convert_bmp_to_ico(input_bytes)?,
        
        // Catch-all for unsupported combinations
        _ => return Err(ConverterError::UnsupportedFormat(
            format!("Conversion from {:?} to {:?} not supported", source_format, target_format)
        )),
    };
    
    // Write the converted bytes to output file
    fs::write(output_path, converted_bytes)
        .map_err(|e| ConverterError::WriteError(e.to_string()))?;
    
    Ok(())
}

/// Alternative conversion function that works with bytes directly
pub fn convert_bytes(
    input_bytes: Vec<u8>, 
    source_format: ImageFormat, 
    target_format: ImageFormat
) -> Result<Vec<u8>, ConverterError> {
    // If source and target are the same, return unchanged
    if source_format == target_format {
        return Ok(input_bytes);
    }
    
    // Dispatch to appropriate converter
    match (source_format, target_format) {
        // From JPEG
        (ImageFormat::JPEG, ImageFormat::PNG) => jpeg_converter::convert_jpeg_to_png(input_bytes),
        (ImageFormat::JPEG, ImageFormat::WEBP) => jpeg_converter::convert_jpeg_to_webp(input_bytes),
        (ImageFormat::JPEG, ImageFormat::GIF) => jpeg_converter::convert_jpeg_to_gif(input_bytes),
        (ImageFormat::JPEG, ImageFormat::ICO) => jpeg_converter::convert_jpeg_to_ico(input_bytes),
        (ImageFormat::JPEG, ImageFormat::BMP) => jpeg_converter::convert_jpeg_to_bmp(input_bytes),
        
        // From PNG
        (ImageFormat::PNG, ImageFormat::JPEG) => png_converter::convert_png_to_jpeg(input_bytes),
        (ImageFormat::PNG, ImageFormat::WEBP) => png_converter::convert_png_to_webp(input_bytes),
        (ImageFormat::PNG, ImageFormat::GIF) => png_converter::convert_png_to_gif(input_bytes),
        (ImageFormat::PNG, ImageFormat::ICO) => png_converter::convert_png_to_ico(input_bytes),
        (ImageFormat::PNG, ImageFormat::BMP) => png_converter::convert_png_to_bmp(input_bytes),
        
        // From WebP
        (ImageFormat::WEBP, ImageFormat::PNG) => webp_converter::convert_webp_to_png(input_bytes),
        (ImageFormat::WEBP, ImageFormat::JPEG) => webp_converter::convert_webp_to_jpeg(input_bytes),
        (ImageFormat::WEBP, ImageFormat::GIF) => webp_converter::convert_webp_to_gif(input_bytes),
        (ImageFormat::WEBP, ImageFormat::ICO) => webp_converter::convert_webp_to_ico(input_bytes),
        (ImageFormat::WEBP, ImageFormat::BMP) => webp_converter::convert_webp_to_bmp(input_bytes),
        
        // From GIF
        (ImageFormat::GIF, ImageFormat::PNG) => gif_converter::convert_gif_to_png(input_bytes),
        (ImageFormat::GIF, ImageFormat::JPEG) => gif_converter::convert_gif_to_jpeg(input_bytes),
        (ImageFormat::GIF, ImageFormat::WEBP) => gif_converter::convert_gif_to_webp(input_bytes),
        (ImageFormat::GIF, ImageFormat::ICO) => gif_converter::convert_gif_to_ico(input_bytes),
        (ImageFormat::GIF, ImageFormat::BMP) => gif_converter::convert_gif_to_bmp(input_bytes),
        
        // From ICO
        (ImageFormat::ICO, ImageFormat::PNG) => ico_converter::convert_ico_to_png(input_bytes),
        (ImageFormat::ICO, ImageFormat::JPEG) => ico_converter::convert_ico_to_jpeg(input_bytes),
        (ImageFormat::ICO, ImageFormat::WEBP) => ico_converter::convert_ico_to_webp(input_bytes),
        (ImageFormat::ICO, ImageFormat::GIF) => ico_converter::convert_ico_to_gif(input_bytes),
        (ImageFormat::ICO, ImageFormat::BMP) => ico_converter::convert_ico_to_bmp(input_bytes),
        
        // From BMP
        (ImageFormat::BMP, ImageFormat::PNG) => bmp_converter::convert_bmp_to_png(input_bytes),
        (ImageFormat::BMP, ImageFormat::JPEG) => bmp_converter::convert_bmp_to_jpeg(input_bytes),
        (ImageFormat::BMP, ImageFormat::WEBP) => bmp_converter::convert_bmp_to_webp(input_bytes),
        (ImageFormat::BMP, ImageFormat::GIF) => bmp_converter::convert_bmp_to_gif(input_bytes),
        (ImageFormat::BMP, ImageFormat::ICO) => bmp_converter::convert_bmp_to_ico(input_bytes),
        
        // Unsupported combinations
        _ => Err(ConverterError::UnsupportedFormat(
            format!("Conversion from {:?} to {:?} not supported", source_format, target_format)
        )),
    }
}