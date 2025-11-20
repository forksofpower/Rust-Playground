use crate::algorithms::FractalAlgorithm;
use crate::types::{ComplexRegion, Dimensions, Pixel};
use rayon::prelude::*;

/// Configuration for rendering a fractal
#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub dimensions: Dimensions,
    pub region: ComplexRegion,
    pub limit: usize,
    pub invert: bool,
}

impl RenderConfig {
    pub fn new(dimensions: Dimensions, region: ComplexRegion, limit: usize, invert: bool) -> Self {
        Self {
            dimensions,
            region,
            limit,
            invert,
        }
    }
}

/// Renders a fractal using CPU parallelization with Rayon
///
/// This function divides the image into horizontal bands and processes them
/// in parallel using Rayon's parallel iterators.
pub fn render_fractal(
    algorithm: &dyn FractalAlgorithm,
    config: &RenderConfig,
) -> Vec<u8> {
    let mut pixels = vec![0u8; config.dimensions.total_pixels()];

    // Split pixels into bands (one per row) and process in parallel
    let bands: Vec<(usize, &mut [u8])> = pixels
        .chunks_mut(config.dimensions.width)
        .enumerate()
        .collect();

    bands.into_par_iter().for_each(|(row, band)| {
        render_band(algorithm, config, row, band);
    });

    pixels
}

/// Render a single horizontal band of the fractal
fn render_band(
    algorithm: &dyn FractalAlgorithm,
    config: &RenderConfig,
    row: usize,
    band: &mut [u8],
) {
    let band_dims = Dimensions::new(config.dimensions.width, 1);

    // Calculate the complex region for this band
    let band_upper_left = config.region.pixel_to_point(
        config.dimensions,
        Pixel::new(0, row),
    );
    let band_lower_right = config.region.pixel_to_point(
        config.dimensions,
        Pixel::new(config.dimensions.width, row + 1),
    );
    let band_region = ComplexRegion::new(band_upper_left, band_lower_right);

    // Render each pixel in the band
    for col in 0..config.dimensions.width {
        let pixel = Pixel::new(col, 0);
        let point = band_region.pixel_to_point(band_dims, pixel);
        
        band[col] = match algorithm.calculate(point, config.limit) {
            None => 0,
            Some(count) => {
                let adjusted_count = if config.invert {
                    config.limit - count
                } else {
                    count
                };
                map_to_grayscale(adjusted_count, config.limit)
            }
        };
    }
}

/// Map iteration count to a grayscale value (0-255)
fn map_to_grayscale(value: usize, limit: usize) -> u8 {
    if limit == 0 {
        return 0;
    }
    let normalized = (value as f64 / limit as f64) * 255.0;
    normalized.min(255.0) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::EscapeTime;
    use num::Complex;

    #[test]
    fn test_map_to_grayscale() {
        assert_eq!(map_to_grayscale(0, 100), 0);
        assert_eq!(map_to_grayscale(50, 100), 127);
        assert_eq!(map_to_grayscale(100, 100), 255);
    }

    #[test]
    fn test_render_small_fractal() {
        let algorithm = EscapeTime;
        let config = RenderConfig::new(
            Dimensions::new(4, 4),
            ComplexRegion::new(Complex::new(-1.0, 1.0), Complex::new(1.0, -1.0)),
            4,
            false,
        );

        let pixels = render_fractal(&algorithm, &config);
        assert_eq!(pixels.len(), 16);
    }
}
