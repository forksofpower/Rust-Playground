use num::Complex;

/// Trait for fractal rendering algorithms
pub trait FractalAlgorithm: Send + Sync {
    /// Calculate the escape time for a given complex number
    ///
    /// # Arguments
    ///
    /// * `c` - The complex number to test
    /// * `limit` - Maximum number of iterations before giving up
    ///
    /// # Returns
    ///
    /// * `Some(iterations)` - Number of iterations before escape
    /// * `None` - Point did not escape within the limit
    fn calculate(&self, c: Complex<f64>, limit: usize) -> Option<usize>;
}

/// Standard Mandelbrot set algorithm using escape time
///
/// The Mandelbrot set is defined as the set of complex numbers c for which
/// the function f(z) = z² + c does not diverge when iterated from z = 0.
pub struct EscapeTime;

impl FractalAlgorithm for EscapeTime {
    fn calculate(&self, c: Complex<f64>, limit: usize) -> Option<usize> {
        let mut z = Complex::new(0.0, 0.0);
        
        for i in 0..limit {
            // Check if z has escaped (using 32 instead of 4 for smoother rendering)
            if z.norm_sqr() > 32.0 {
                return Some(i);
            }
            // Mandelbrot iteration: z = z² + c
            z = z * z + c;
        }
        
        // Point is in the set (or close enough)
        None
    }
}

/// Burning Ship fractal algorithm
///
/// Similar to the Mandelbrot set, but uses absolute values of the
/// real and imaginary components before squaring.
pub struct BurningShip;

impl FractalAlgorithm for BurningShip {
    fn calculate(&self, c: Complex<f64>, limit: usize) -> Option<usize> {
        let mut z = Complex::new(0.0, 0.0);
        
        for i in 0..limit {
            if z.norm_sqr() > 4.0 {
                return Some(i);
            }
            
            // Burning Ship iteration: take absolute value of components before squaring
            z = Complex::new((z.re as f64).abs(), (z.im as f64).abs());
            z = z * z + c;
        }
        
        None
    }
}

/// Get a fractal algorithm by name
pub fn get_algorithm(name: &str) -> Box<dyn FractalAlgorithm> {
    match name {
        "escape_time" => Box::new(EscapeTime),
        "burning_ship" => Box::new(BurningShip),
        _ => Box::new(EscapeTime), // Default to EscapeTime
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_time_origin() {
        let c = Complex::new(0.0, 0.0);
        assert_eq!(EscapeTime.calculate(c, 10), None);
    }

    #[test]
    fn test_escape_time_diverges() {
        let c = Complex::new(1.0, 2.0);
        assert_eq!(EscapeTime.calculate(c, 100), Some(1));
    }

    #[test]
    fn test_escape_time_periodic() {
        let c = Complex::new(-0.4, 0.6);
        assert_eq!(EscapeTime.calculate(c, 1000), Some(26));
    }

    #[test]
    fn test_escape_time_outside_main_cardioid() {
        let c = Complex::new(-1.75, -0.02);
        assert_eq!(EscapeTime.calculate(c, 1000), Some(13));
    }

    #[test]
    fn test_escape_time_inside_set() {
        let c = Complex::new(0.32, -0.04);
        assert_eq!(EscapeTime.calculate(c, 1000), None);
    }

    #[test]
    fn test_burning_ship() {
        // Basic test for burning ship
        let c = Complex::new(0.0, 0.0);
        assert_eq!(BurningShip.calculate(c, 10), None);
    }
}
