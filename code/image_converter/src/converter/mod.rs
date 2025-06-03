pub mod errors;
pub mod formats;
pub mod main_converter;

// Individual format converters
pub mod jpeg_converter;
pub mod png_converter;
pub mod webp_converter;
pub mod gif_converter;
pub mod ico_converter;
pub mod bmp_converter;

// Re-export commonly used items
pub use errors::ConverterError;
pub use formats::ImageFormat;
pub use main_converter::{convert, convert_bytes};