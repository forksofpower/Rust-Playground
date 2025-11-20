use crate::error::{MandelbrotError, Result};
use crate::parsers::parse_pair;
use crate::types::Dimensions;
use clap::builder::PossibleValuesParser;
use clap::Parser;

/// Command-line arguments for the mandelbrot renderer
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// Zoom level (magnitude of the viewing window)
    #[arg(short, long)]
    pub zoom: f64,

    /// Output filename for the rendered image
    #[arg(short, long, default_value = "mandelbrot.png")]
    pub output: String,

    /// Fractal algorithm to use
    #[arg(
        short,
        long,
        default_value = "escape_time",
        value_parser = PossibleValuesParser::new(["escape_time", "burning_ship"])
    )]
    pub algorithm: String,

    /// Center point in the complex plane (format: "real,imag")
    #[arg(
        short,
        long,
        allow_hyphen_values = true,
        value_parser = parse_center
    )]
    pub center: (f64, f64),

    /// Image dimensions (format: "widthxheight")
    #[arg(
        short,
        long,
        default_value = "1920x1080",
        value_parser = parse_dimensions
    )]
    pub dimensions: Dimensions,

    /// Maximum number of iterations before giving up
    #[arg(short, long)]
    pub limit: usize,

    /// Invert the color palette
    #[arg(short, long, default_value_t = false)]
    pub invert: bool,

    /// Use GPU rendering (requires 'gpu' feature)
    #[arg(short, long, default_value_t = false)]
    pub gpu: bool,
}

/// Parse center point from string (format: "real,imag")
fn parse_center(arg: &str) -> Result<(f64, f64)> {
    parse_pair::<f64>(arg, ',')
        .ok_or_else(|| MandelbrotError::ParseError("Invalid center point format".to_string()))
}

/// Parse dimensions from string (format: "widthxheight")
fn parse_dimensions(arg: &str) -> Result<Dimensions> {
    parse_pair::<usize>(arg, 'x')
        .map(|(width, height)| Dimensions::new(width, height))
        .ok_or_else(|| MandelbrotError::ParseError("Invalid dimensions format".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_center() {
        assert!(parse_center("0.5,-0.25").is_ok());
        assert_eq!(parse_center("0.5,-0.25").unwrap(), (0.5, -0.25));
        assert!(parse_center("invalid").is_err());
    }

    #[test]
    fn test_parse_dimensions() {
        assert!(parse_dimensions("1920x1080").is_ok());
        let dims = parse_dimensions("1920x1080").unwrap();
        assert_eq!(dims.width, 1920);
        assert_eq!(dims.height, 1080);
        assert!(parse_dimensions("invalid").is_err());
    }
}
