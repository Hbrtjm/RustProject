use crate::converter::errors::ConverterError;
use image::{DynamicImage, ImageFormat as ImgFmt, RgbaImage, RgbImage};
use std::io::Cursor;

/// Convert ICO to PNG format
/// Note: This extracts the first (largest) icon from the ICO file
pub fn convert_ico_to_png(input: Vec<u8>) -> Result<Vec<u8>, ConverterError> {
    let cursor = Cursor::new(input);
    let icon_dir = ico::IconDir::read(cursor)
        .map_err(|e| ConverterError::ConversionError(format!("Failed to read ICO file: {}", e)))?;
    
    if icon_dir.entries().is_empty() {
        return Err(ConverterError::ConversionError("ICO file contains no icons".to_string()));
    }
    
    // Get the first (usually largest) icon
    let entry = &icon_dir.entries()[0];
    let ico_image = entry.decode()
        .map_err(|e| ConverterError::ConversionError(format!("Failed to decode icon: {}", e)))?;
    
    let raw_pixels = ico_image.rgba_data().to_vec();
    let (w, h) = (ico_image.width(), ico_image.height());
    
    // Validate dimensions
    if w == 0 || h == 0 {
        return Err(ConverterError::ConversionError("Invalid icon dimensions".to_string()));
    }
    
    // Validate pixel data length
    let expected_len = (w * h * 4) as usize;
    if raw_pixels.len() != expected_len {
        return Err(ConverterError::ConversionError(
            format!("Invalid pixel data length: expected {}, got {}", expected_len, raw_pixels.len())
        ));
    }
    
    let rgba_image = RgbaImage::from_raw(w, h, raw_pixels)
        .ok_or_else(|| ConverterError::ConversionError("Failed to create RGBA image buffer".to_string()))?;
    
    let mut out_buf = Vec::new();
    DynamicImage::ImageRgba8(rgba_image)
        .write_to(&mut Cursor::new(&mut out_buf), ImgFmt::Png)
        .map_err(|e| ConverterError::WriteError(format!("Failed to write PNG: {}", e)))?;
    
    Ok(out_buf)
}

/// Convert ICO to JPEG format
/// Note: This extracts the first icon and removes transparency
pub fn convert_ico_to_jpeg(input: Vec<u8>) -> Result<Vec<u8>, ConverterError> {
    let cursor = Cursor::new(input);
    let icon_dir = ico::IconDir::read(cursor)
        .map_err(|e| ConverterError::ConversionError(format!("Failed to read ICO file: {}", e)))?;
    
    if icon_dir.entries().is_empty() {
        return Err(ConverterError::ConversionError("ICO file contains no icons".to_string()));
    }
    
    let entry = &icon_dir.entries()[0];
    let ico_image = entry.decode()
        .map_err(|e| ConverterError::ConversionError(format!("Failed to decode icon: {}", e)))?;
    
    let raw_pixels = ico_image.rgba_data().to_vec();
    let (w, h) = (ico_image.width(), ico_image.height());
    
    // Validate dimensions
    if w == 0 || h == 0 {
        return Err(ConverterError::ConversionError("Invalid icon dimensions".to_string()));
    }
    
    // Convert RGBA to RGB (JPEG doesn't support alpha)
    // Handle transparency by blending with white background
    let rgb_pixels: Vec<u8> = raw_pixels
        .chunks_exact(4)
        .flat_map(|rgba| {
            let alpha = rgba[3] as f32 / 255.0;
            let r = ((rgba[0] as f32 * alpha) + (255.0 * (1.0 - alpha))) as u8;
            let g = ((rgba[1] as f32 * alpha) + (255.0 * (1.0 - alpha))) as u8;
            let b = ((rgba[2] as f32 * alpha) + (255.0 * (1.0 - alpha))) as u8;
            [r, g, b]
        })
        .collect();
    
    let rgb_image = RgbImage::from_raw(w, h, rgb_pixels)
        .ok_or_else(|| ConverterError::ConversionError("Failed to create RGB image buffer".to_string()))?;
    
    let mut out_buf = Vec::new();
    DynamicImage::ImageRgb8(rgb_image)
        .write_to(&mut Cursor::new(&mut out_buf), ImgFmt::Jpeg)
        .map_err(|e| ConverterError::WriteError(format!("Failed to write JPEG: {}", e)))?;
    
    Ok(out_buf)
}

/// Convert ICO to WebP format (lossy)
/// Note: This extracts the first icon
pub fn convert_ico_to_webp(input: Vec<u8>) -> Result<Vec<u8>, ConverterError> {
    let cursor = Cursor::new(input);
    let icon_dir = ico::IconDir::read(cursor)
        .map_err(|e| ConverterError::ConversionError(format!("Failed to read ICO file: {}", e)))?;
    
    if icon_dir.entries().is_empty() {
        return Err(ConverterError::ConversionError("ICO file contains no icons".to_string()));
    }
    
    let entry = &icon_dir.entries()[0];
    let ico_image = entry.decode()
        .map_err(|e| ConverterError::ConversionError(format!("Failed to decode icon: {}", e)))?;
    
    let raw_pixels = ico_image.rgba_data().to_vec();
    let (w, h) = (ico_image.width(), ico_image.height());
    
    // Validate dimensions
    if w == 0 || h == 0 {
        return Err(ConverterError::ConversionError("Invalid icon dimensions".to_string()));
    }
    
    // Use the `webp` crate for better compression
    let encoder = webp::Encoder::from_rgba(&raw_pixels, w, h);
    let webp_mem = encoder.encode(80.0); // quality = 80
    Ok(webp_mem.into_vec())
}

/// Convert ICO to GIF format
/// Note: This extracts the first icon
pub fn convert_ico_to_gif(input: Vec<u8>) -> Result<Vec<u8>, ConverterError> {
    let cursor = Cursor::new(input);
    let icon_dir = ico::IconDir::read(cursor)
        .map_err(|e| ConverterError::ConversionError(format!("Failed to read ICO file: {}", e)))?;
    
    if icon_dir.entries().is_empty() {
        return Err(ConverterError::ConversionError("ICO file contains no icons".to_string()));
    }
    
    let entry = &icon_dir.entries()[0];
    let ico_image = entry.decode()
        .map_err(|e| ConverterError::ConversionError(format!("Failed to decode icon: {}", e)))?;
    
    let raw_pixels = ico_image.rgba_data().to_vec();
    let (w, h) = (ico_image.width(), ico_image.height());
    
    // Validate dimensions
    if w == 0 || h == 0 {
        return Err(ConverterError::ConversionError("Invalid icon dimensions".to_string()));
    }
    
    // Validate pixel data length
    let expected_len = (w * h * 4) as usize;
    if raw_pixels.len() != expected_len {
        return Err(ConverterError::ConversionError(
            format!("Invalid pixel data length: expected {}, got {}", expected_len, raw_pixels.len())
        ));
    }
    
    let rgba_image = RgbaImage::from_raw(w, h, raw_pixels)
        .ok_or_else(|| ConverterError::ConversionError("Failed to create RGBA image buffer".to_string()))?;
    
    let mut out_buf = Vec::new();
    DynamicImage::ImageRgba8(rgba_image)
        .write_to(&mut Cursor::new(&mut out_buf), ImgFmt::Gif)
        .map_err(|e| ConverterError::WriteError(format!("Failed to write GIF: {}", e)))?;
    
    Ok(out_buf)
}

/// Convert ICO to BMP format
/// Note: This extracts the first icon
pub fn convert_ico_to_bmp(input: Vec<u8>) -> Result<Vec<u8>, ConverterError> {
    let cursor = Cursor::new(input);
    let icon_dir = ico::IconDir::read(cursor)
        .map_err(|e| ConverterError::ConversionError(format!("Failed to read ICO file: {}", e)))?;
    
    if icon_dir.entries().is_empty() {
        return Err(ConverterError::ConversionError("ICO file contains no icons".to_string()));
    }
    
    let entry = &icon_dir.entries()[0];
    let ico_image = entry.decode()
        .map_err(|e| ConverterError::ConversionError(format!("Failed to decode icon: {}", e)))?;
    
    let raw_pixels = ico_image.rgba_data().to_vec();
    let (w, h) = (ico_image.width(), ico_image.height());
    
    // Validate dimensions
    if w == 0 || h == 0 {
        return Err(ConverterError::ConversionError("Invalid icon dimensions".to_string()));
    }
    
    // Validate pixel data length
    let expected_len = (w * h * 4) as usize;
    if raw_pixels.len() != expected_len {
        return Err(ConverterError::ConversionError(
            format!("Invalid pixel data length: expected {}, got {}", expected_len, raw_pixels.len())
        ));
    }
    
    let rgba_image = RgbaImage::from_raw(w, h, raw_pixels)
        .ok_or_else(|| ConverterError::ConversionError("Failed to create RGBA image buffer".to_string()))?;
    
    let mut out_buf = Vec::new();
    DynamicImage::ImageRgba8(rgba_image)
        .write_to(&mut Cursor::new(&mut out_buf), ImgFmt::Bmp)
        .map_err(|e| ConverterError::WriteError(format!("Failed to write BMP: {}", e)))?;
    
    Ok(out_buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_empty_ico_file() {
        let empty_data = Vec::new();
        let result = convert_ico_to_png(empty_data);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_invalid_ico_data() {
        let invalid_data = vec![0x00, 0x01, 0x02, 0x03]; // Not a valid ICO
        let result = convert_ico_to_png(invalid_data);
        assert!(result.is_err());
    }
}