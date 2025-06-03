use crate::converter::errors::ConverterError;
use image::{codecs::webp::WebPEncoder, ExtendedColorType, DynamicImage, ImageFormat as ImgFmt};
use std::io::Cursor;

/// Convert PNG to JPEG format
pub fn convert_png_to_jpeg(input: Vec<u8>) -> Result<Vec<u8>, ConverterError> {
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

/// Convert PNG to WebP format (lossy)
pub fn convert_png_to_webp(input: Vec<u8>) -> Result<Vec<u8>, ConverterError> {
    let dyn_img: DynamicImage = image::load_from_memory(&input)
        .map_err(|e| ConverterError::ConversionError(e.to_string()))?;
    let rgba = dyn_img.to_rgba8();
    let (w, h) = (rgba.width(), rgba.height());
    let raw_pixels = rgba.into_raw();
    
    // Use the `webp` crate for better compression
    let mut webp_mem = Cursor::new(Vec::new());
    let encoder = WebPEncoder::new_lossless(&mut webp_mem);
    encoder.encode(&raw_pixels, w as u32, h as u32, ExtendedColorType::Rgba8)
        .map_err(|e| ConverterError::ConversionError(e.to_string()))?;
    let result = webp_mem.clone().into_inner();
    Ok(result)
}

/// Convert PNG to GIF format
pub fn convert_png_to_gif(input: Vec<u8>) -> Result<Vec<u8>, ConverterError> {
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


/// Convert PNG to BMP format
pub fn convert_png_to_bmp(input: Vec<u8>) -> Result<Vec<u8>, ConverterError> {
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