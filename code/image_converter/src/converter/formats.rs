#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    PNG,
    JPEG,
    WEBP,
    GIF,
    ICO,
    BMP,
}

impl ImageFormat {
    fn get_extension(filename: &str) -> &str {
        filename.rsplit('.').next().unwrap_or("")
    }

    pub fn from_extension(filename_opt: Option<&str>) -> Option<Self> {
        let mut result = None;
        if let Some(filename) = filename_opt { 
        let ext = Self::get_extension(&filename);
        let capitalized = ext.to_uppercase();
        result = match capitalized.as_str() {
            "PNG" => Some(ImageFormat::PNG),
            "JPEG" => Some(ImageFormat::JPEG),
            "WEBP" => Some(ImageFormat::WEBP),
            "GIF" => Some(ImageFormat::GIF),
            "ICO" => Some(ImageFormat::ICO),
            "BMP" => Some(ImageFormat::BMP),
            _ => None,
        };
    }
    result
    }

    pub fn to_image_output_format(&self) -> image::ImageFormat {
        match self {
            ImageFormat::PNG => image::ImageFormat::Png,
            ImageFormat::JPEG => image::ImageFormat::Jpeg,
            ImageFormat::WEBP => image::ImageFormat::WebP,
            ImageFormat::GIF => image::ImageFormat::Gif,
            ImageFormat::ICO => image::ImageFormat::Ico,
            ImageFormat::BMP => image::ImageFormat::Bmp,
        }
    }
    
    pub fn to_extension(&self) -> &str {
        match self {
            ImageFormat::PNG => "png",
            ImageFormat::JPEG => "jpg",
            ImageFormat::WEBP => "webp",
            ImageFormat::GIF => "gif",
            ImageFormat::ICO => "ico",
            ImageFormat::BMP => "bmp",
        }
    }
}
