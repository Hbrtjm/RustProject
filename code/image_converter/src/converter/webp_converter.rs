use crate::converter::errors::ConverterError;
use image::{DynamicImage, ImageFormat as ImgFmt};
use std::io::Cursor;

/// Convert WebP to PNG format
pub fn convert_webp_to_png(input: Vec<u8>) -> Result<Vec<u8>, ConverterError> {
    // Decode WebP using the image crate (supports both lossless and lossy WebP)
    let dyn_img: DynamicImage = image::load_from_memory(&input)
        .map_err(|e| ConverterError::ConversionError(e.to_string()))?;
    let rgba = dyn_img.to_rgba8();
    let (w, h) = (rgba.width(), rgba.height());
    let mut out_buf = Vec::new();
    DynamicImage::ImageRgba8(image::ImageBuffer::from_raw(w, h, rgba.into_raw()).unwrap())
        .write_to(&mut Cursor::new(&mut out_buf), ImgFmt::Png)
        .map_err(|e| ConverterError::WriteError(e.to_string()))?;
    Ok(out_buf)
}

/// Convert WebP to JPEG format
pub fn convert_webp_to_jpeg(input: Vec<u8>) -> Result<Vec<u8>, ConverterError> {
    let dyn_img: DynamicImage = image::load_from_memory(&input)
        .map_err(|e| ConverterError::ConversionError(e.to_string()))?;
    let rgb = dyn_img.to_rgb8(); // JPEG doesn't support alpha
    let (w, h) = (rgb.width(), rgb.height());
    let mut out_buf = Vec::new();
    DynamicImage::ImageRgb8(image::ImageBuffer::from_raw(w, h, rgb.into_raw()).unwrap())
        .write_to(&mut Cursor::new(&mut out_buf), ImgFmt::Jpeg)
        .map_err(|e| ConverterError::WriteError(e.to_string()))?;
    Ok(out_buf)
}

/// Convert WebP to GIF format
pub fn convert_webp_to_gif(input: Vec<u8>) -> Result<Vec<u8>, ConverterError> {
    let dyn_img: DynamicImage = image::load_from_memory(&input)
        .map_err(|e| ConverterError::ConversionError(e.to_string()))?;
    let rgba = dyn_img.to_rgba8();
    let (w, h) = (rgba.width(), rgba.height());
    let mut out_buf = Vec::new();
    DynamicImage::ImageRgba8(image::ImageBuffer::from_raw(w, h, rgba.into_raw()).unwrap())
        .write_to(&mut Cursor::new(&mut out_buf), ImgFmt::Gif)
        .map_err(|e| ConverterError::WriteError(e.to_string()))?;
    Ok(out_buf)
}

/// Convert WebP to ICO format
pub fn convert_webp_to_ico(input: Vec<u8>) -> Result<Vec<u8>, ConverterError> {
    let dyn_img: DynamicImage = image::load_from_memory(&input)
        .map_err(|e| ConverterError::ConversionError(e.to_string()))?;
    let rgba = dyn_img.to_rgba8();
    let (w, h) = (rgba.width(), rgba.height());
    let raw_pixels = rgba.into_raw();
    
    let ico_img = ico::IconImage::from_rgba_data(w as u32, h as u32, raw_pixels);
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    icon_dir.add_entry(
        ico::IconDirEntry::encode(&ico_img)
            .map_err(|e| ConverterError::ConversionError(e.to_string()))?
    );
    
    let mut buf = Vec::new();
    icon_dir.write(&mut buf)
        .map_err(|e| ConverterError::WriteError(e.to_string()))?;
    Ok(buf)
}

/// Convert WebP to BMP format
pub fn convert_webp_to_bmp(input: Vec<u8>) -> Result<Vec<u8>, ConverterError> {
    let dyn_img: DynamicImage = image::load_from_memory(&input)
        .map_err(|e| ConverterError::ConversionError(e.to_string()))?;
    let rgba = dyn_img.to_rgba8();
    let (w, h) = (rgba.width(), rgba.height());
    let mut out_buf = Vec::new();
    DynamicImage::ImageRgba8(image::ImageBuffer::from_raw(w, h, rgba.into_raw()).unwrap())
        .write_to(&mut Cursor::new(&mut out_buf), ImgFmt::Bmp)
        .map_err(|e| ConverterError::WriteError(e.to_string()))?;
    Ok(out_buf)
}

/// Legacy function for compatibility with existing main_converter.rs
/// Converts an RGBA8 pixel buffer to WebP format
pub fn convert_to_webp(rgba_data: Vec<u8>, _format: &crate::converter::formats::ImageFormat) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Assuming the rgba_data comes from a to_rgba8().to_vec() call
    // We need to determine dimensions - this is a limitation of the current API
    // For now, we'll return an error suggesting to use the new API
    Err("Use convert_*_to_webp functions instead for proper dimension handling".into())
}