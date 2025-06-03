use std::path::{Path, PathBuf};
use std::fs;
use crate::converter::formats::ImageFormat;
use crate::converter::errors::ConverterError;
use crate::converter::{
    jpeg_converter, png_converter, webp_converter, 
    gif_converter, bmp_converter
};

// Alternative version that returns PathBuf for better path handling
fn convert_path_extension_pathbuf(path: &Path, new_extension: &str) -> PathBuf {
    let clean_extension = new_extension.strip_prefix('.').unwrap_or(new_extension);
    path.with_extension(clean_extension)
}

/// Main conversion function that dispatches to appropriate converters
pub fn convert(input_path: &Path, target_format: &ImageFormat) -> Result<(), ConverterError> {
    let output_path = convert_path_extension_pathbuf(input_path, target_format.to_extension());
    if input_path == output_path {
        return Err(ConverterError::UnsupportedFormat(
            "Input and output paths are the same".to_string()
        ));
    }
    let input_bytes = fs::read(input_path)
        .map_err(|e| ConverterError::ReadError(e.to_string()))?;
    
    let source_format = ImageFormat::from_extension(input_path.to_str())
        .ok_or_else(|| ConverterError::UnsupportedFormat("Cannot determine input format".to_string()))?;
    
    if source_format == *target_format {
        fs::copy(input_path, output_path)
            .map_err(|e| ConverterError::WriteError(e.to_string()))?;
        return Ok(());
    }
    
    let converted_bytes = match (source_format, target_format) {
        // JPEG
        (ImageFormat::JPEG, ImageFormat::PNG) => jpeg_converter::convert_jpeg_to_png(input_bytes)?,
        (ImageFormat::JPEG, ImageFormat::WEBP) => jpeg_converter::convert_jpeg_to_webp(input_bytes)?,
        (ImageFormat::JPEG, ImageFormat::GIF) => jpeg_converter::convert_jpeg_to_gif(input_bytes)?,
        (ImageFormat::JPEG, ImageFormat::BMP) => jpeg_converter::convert_jpeg_to_bmp(input_bytes)?,
        
        // PNG
        (ImageFormat::PNG, ImageFormat::JPEG) => png_converter::convert_png_to_jpeg(input_bytes)?,
        (ImageFormat::PNG, ImageFormat::WEBP) => png_converter::convert_png_to_webp(input_bytes)?,
        (ImageFormat::PNG, ImageFormat::GIF) => png_converter::convert_png_to_gif(input_bytes)?,
        (ImageFormat::PNG, ImageFormat::BMP) => png_converter::convert_png_to_bmp(input_bytes)?,
        
        // WebP
        (ImageFormat::WEBP, ImageFormat::PNG) => webp_converter::convert_webp_to_png(input_bytes)?,
        (ImageFormat::WEBP, ImageFormat::JPEG) => webp_converter::convert_webp_to_jpeg(input_bytes)?,
        (ImageFormat::WEBP, ImageFormat::GIF) => webp_converter::convert_webp_to_gif(input_bytes)?,
        (ImageFormat::WEBP, ImageFormat::BMP) => webp_converter::convert_webp_to_bmp(input_bytes)?,
        
        // GIF
        (ImageFormat::GIF, ImageFormat::PNG) => gif_converter::convert_gif_to_png(input_bytes)?,
        (ImageFormat::GIF, ImageFormat::JPEG) => gif_converter::convert_gif_to_jpeg(input_bytes)?,
        (ImageFormat::GIF, ImageFormat::WEBP) => gif_converter::convert_gif_to_webp(input_bytes)?,
        (ImageFormat::GIF, ImageFormat::BMP) => gif_converter::convert_gif_to_bmp(input_bytes)?,
                
        // BMP
        (ImageFormat::BMP, ImageFormat::PNG) => bmp_converter::convert_bmp_to_png(input_bytes)?,
        (ImageFormat::BMP, ImageFormat::JPEG) => bmp_converter::convert_bmp_to_jpeg(input_bytes)?,
        (ImageFormat::BMP, ImageFormat::WEBP) => bmp_converter::convert_bmp_to_webp(input_bytes)?,
        (ImageFormat::BMP, ImageFormat::GIF) => bmp_converter::convert_bmp_to_gif(input_bytes)?,
        
        _ => return Err(ConverterError::UnsupportedFormat(
            format!("Conversion from {:?} to {:?} not supported", source_format, target_format)
        )),
    };
    
    fs::write(output_path, converted_bytes)
        .map_err(|e| ConverterError::WriteError(e.to_string()))?;
    
    Ok(())
}
