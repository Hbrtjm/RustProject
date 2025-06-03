#[cfg(test)]
pub mod tests {
    use std::path::Path;
    use image_converter::converter::{formats::ImageFormat, main_converter::convert};

    #[test]
    fn png_to_jpeg_roundtrip() {
        let input = Path::new("assets/samples/flowey.png");
        let output = Path::new("assets/samples/flowey.jpg");
        convert(input, &ImageFormat::JPEG).unwrap();
        let img = image::open(output).unwrap();
        assert_eq!(img.color(), image::ColorType::Rgb8);
        convert(output, &ImageFormat::PNG).unwrap();
        let img = image::open(output).unwrap();
        assert_eq!(img.color(), image::ColorType::Rgba8); // No complicated tests, just check if the image is not corrupted 
    }
}
