#[derive(thiserror::Error, Debug)]
pub enum ConverterError {
    #[error("unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Image processing error: {0}")]
    ImageError(#[from] image::ImageError),
    #[error("Read error: {0}")]
    ReadError(String),
    #[error("Conversion error: {0}")]
    ConversionError(String),
    #[error("Write error: {0}")]
    WriteError(String),
    #[error("Unknown error: {0}")]
    UnknownError(String),
}
