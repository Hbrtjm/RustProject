use crate::converter::errors::ConverterError;
use image::{codecs::webp, DynamicImage, ImageFormat as ImgFmt};
use std::io::Cursor;

/// Convert JPEG to PNG format
pub fn convert_jpeg_to_png(input: Vec<u8>) -> Result<Vec<u8>, ConverterError> {
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

/// Convert JPEG to WebP format (lossy)
pub fn convert_jpeg_to_webp(input: Vec<u8>) -> Result<Vec<u8>, ConverterError> {
    let dyn_img: DynamicImage = image::load_from_memory(&input)
        .map_err(|e| ConverterError::ConversionError(e.to_string()))?;
    let rgba = dyn_img.to_rgba8();
    let (w, h) = (rgba.width(), rgba.height());
    let raw_pixels = rgba.into_raw();
    
    // Use the `webp` crate for better compression
    let encoder = webp::Encoder::from_rgba(&raw_pixels, w as u32, h as u32);
    let webp_mem = encoder.encode(80.0); // default quality = 80
    Ok(webp_mem.into_vec())
}

/// Convert JPEG to GIF format
pub fn convert_jpeg_to_gif(input: Vec<u8>) -> Result<Vec<u8>, ConverterError> {
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

/// Convert JPEG to ICO format
pub fn convert_jpeg_to_ico(input: Vec<u8>) -> Result<Vec<u8>, ConverterError> {
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

/// Convert JPEG to BMP format
pub fn convert_jpeg_to_bmp(input: Vec<u8>) -> Result<Vec<u8>, ConverterError> {
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