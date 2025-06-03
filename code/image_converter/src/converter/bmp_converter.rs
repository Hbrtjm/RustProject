use crate::converter::errors::ConverterError;
use image::{codecs::webp::WebPEncoder, DynamicImage, ExtendedColorType, ImageFormat as ImgFmt};
use std::io::Cursor;

/// Convert BMP to PNG format
pub fn convert_bmp_to_png(input: Vec<u8>) -> Result<Vec<u8>, ConverterError> {
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

/// Convert BMP to JPEG format
pub fn convert_bmp_to_jpeg(input: Vec<u8>) -> Result<Vec<u8>, ConverterError> {
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

/// Convert BMP to WebP format (lossy)
pub fn convert_bmp_to_webp(input: Vec<u8>) -> Result<Vec<u8>, ConverterError> {
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

/// Convert BMP to GIF format
pub fn convert_bmp_to_gif(input: Vec<u8>) -> Result<Vec<u8>, ConverterError> {
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
