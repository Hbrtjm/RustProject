#[cfg(test)]
pub mod tests {
    use std::path::Path;
    use image_converter::converter::{formats::ImageFormat, main_converter::convert};

    #[test]
    fn png_to_jpeg_roundtrip() {
        let input = Path::new("assets/samples/example.png");
        let output = Path::new("tests/outputs/test.jpg");
        let format = image_converter::converter::formats::ImageFormat::from_extension(input.to_str());
        // Of course I have to borrow here, because it would be consumed twice...
        convert(input, output, &ImageFormat::JPEG).unwrap();
        // Optionally reopen `tests/outputs/test.jpg` and verify basic properties
        let img = image::open(output).unwrap();
        assert_eq!(img.color(), image::ColorType::Rgb8);
    }
}
