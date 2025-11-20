use crate::error::Result;
use crate::types::Dimensions;
use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;
use std::path::Path;

/// Write grayscale pixel data to a PNG file
///
/// # Arguments
///
/// * `path` - Path where the PNG file will be written
/// * `pixels` - Grayscale pixel data (one byte per pixel)
/// * `dimensions` - Image dimensions (width x height)
///
/// # Returns
///
/// `Ok(())` on success, or an error if writing fails
pub fn write_png<P: AsRef<Path>>(
    path: P,
    pixels: &[u8],
    dimensions: Dimensions,
) -> Result<()> {
    // Verify that the pixel buffer size matches the dimensions
    assert_eq!(
        pixels.len(),
        dimensions.total_pixels(),
        "Pixel buffer size must match dimensions"
    );

    let output = File::create(path)?;
    let encoder = PNGEncoder::new(output);
    
    encoder.encode(
        pixels,
        dimensions.width as u32,
        dimensions.height as u32,
        ColorType::Gray(8),
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn test_write_png() {
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_mandelbrot.png");

        let dimensions = Dimensions::new(2, 2);
        let pixels = vec![0, 128, 255, 64];

        let result = write_png(&temp_file, &pixels, dimensions);
        assert!(result.is_ok());

        // Verify file was created
        assert!(temp_file.exists());

        // Verify it's a valid PNG by checking magic bytes
        let mut file = File::open(&temp_file).unwrap();
        let mut magic = [0u8; 8];
        file.read_exact(&mut magic).unwrap();
        assert_eq!(&magic, b"\x89PNG\r\n\x1a\n");

        // Clean up
        std::fs::remove_file(temp_file).ok();
    }

    #[test]
    #[should_panic(expected = "Pixel buffer size must match dimensions")]
    fn test_write_png_size_mismatch() {
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_mismatch.png");

        let dimensions = Dimensions::new(2, 2);
        let pixels = vec![0, 128]; // Wrong size!

        let _ = write_png(&temp_file, &pixels, dimensions);
    }
}
