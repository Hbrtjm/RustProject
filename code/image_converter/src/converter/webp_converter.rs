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
