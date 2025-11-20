use std::fmt;
use std::io;

/// Custom error type for the mandelbrot application
#[derive(Debug)]
pub enum MandelbrotError {
    /// Error parsing command-line arguments
    ParseError(String),
    /// I/O error when reading or writing files
    IoError(io::Error),
    /// Image encoding/decoding error
    ImageError(image::ImageError),
    /// GPU rendering error
    #[cfg(feature = "gpu")]
    GpuError(String),
}

impl fmt::Display for MandelbrotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MandelbrotError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            MandelbrotError::IoError(err) => write!(f, "I/O error: {}", err),
            MandelbrotError::ImageError(err) => write!(f, "Image error: {}", err),
            #[cfg(feature = "gpu")]
            MandelbrotError::GpuError(msg) => write!(f, "GPU error: {}", msg),
        }
    }
}

impl std::error::Error for MandelbrotError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MandelbrotError::IoError(err) => Some(err),
            MandelbrotError::ImageError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for MandelbrotError {
    fn from(err: io::Error) -> Self {
        MandelbrotError::IoError(err)
    }
}

impl From<image::ImageError> for MandelbrotError {
    fn from(err: image::ImageError) -> Self {
        MandelbrotError::ImageError(err)
    }
}

/// Type alias for Results using MandelbrotError
pub type Result<T> = std::result::Result<T, MandelbrotError>;
