use std::path::Path;
use crate::utils::file_io::{read_image, write_image};
use crate::converter::webp_converter::*;
use crate::converter::formats::ImageFormat;
use crate::converter::errors::ConverterError;

pub fn convert(input_path: &Path, output_path: &Path, format: &ImageFormat) -> Result<(), ConverterError> {
    let image = read_image(input_path)
        .map_err(|e| ConverterError::ReadError(e.to_string()))?;
    let converted_image = match &format {
        ImageFormat::WEBP => convert_to_webp(image.to_rgba8().to_vec(), format)
            .map_err(|e| ConverterError::ConversionError(e.to_string()))?,
        _ => {
            let mut output = Vec::new();
            image.write_to(&mut std::io::Cursor::new(&mut output), format.to_image_output_format())
                .map_err(|e| ConverterError::WriteError(e.to_string()))?;
            output
        }
    };
    let final_image = image::load_from_memory(&converted_image)
        .map_err(|e| ConverterError::ConversionError(e.to_string()))?;
    write_image(&final_image, output_path, format)
        .map_err(|e| ConverterError::WriteError(e.to_string()))?;
    Ok(())
}