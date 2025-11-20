use crate::error::{MandelbrotError, Result};
use crate::types::Dimensions;
use ocl::ProQue;

/// Map iteration count to an RGB grayscale value
fn map_iterations_to_color(iterations: u32, max_iterations: u32) -> image::Rgb<u8> {
    if iterations > max_iterations {
        return image::Rgb([255, 255, 255]);
    }
    
    let normalized = if max_iterations == 255 {
        iterations as u8
    } else {
        ((iterations as f32 / max_iterations as f32) * 255.0).round() as u8
    };
    
    image::Rgb([normalized, normalized, normalized])
}

/// Render a fractal using GPU acceleration with OpenCL
///
/// # Arguments
///
/// * `dimensions` - Image dimensions (width x height)
/// * `limit` - Maximum number of iterations
///
/// # Returns
///
/// An RGB image on success, or an error if GPU rendering fails
pub fn gpu_render(dimensions: Dimensions, limit: usize) -> Result<image::RgbImage> {
    let w = dimensions.width as u32;
    let h = dimensions.height as u32;
    let mut img = image::RgbImage::new(w, h);
    
    // Build OpenCL context and load the kernel from mandelbrot.cl
    let pro_que = ProQue::builder()
        .src(include_str!("shaders/mandelbrot.cl"))
        .dims((w, h))
        .build()
        .map_err(|e| MandelbrotError::GpuError(format!("Failed to build OpenCL context: {}", e)))?;
    
    // Create output buffer on the GPU
    let buffer = pro_que
        .create_buffer::<u32>()
        .map_err(|e| MandelbrotError::GpuError(format!("Failed to create buffer: {}", e)))?;
    
    // Build and configure the kernel
    let kernel = pro_que
        .kernel_builder("mandelbrot")
        .arg(&buffer)
        .arg(w)
        .arg(h)
        .arg(limit as u32)
        .build()
        .map_err(|e| MandelbrotError::GpuError(format!("Failed to build kernel: {}", e)))?;

    // Execute the kernel
    unsafe {
        kernel.enq()
            .map_err(|e| MandelbrotError::GpuError(format!("Failed to execute kernel: {}", e)))?;
    }
    
    // Read results back from GPU
    let mut results = vec![0u32; buffer.len()];
    buffer.read(&mut results).enq()
        .map_err(|e| MandelbrotError::GpuError(format!("Failed to read buffer: {}", e)))?;
    
    // Convert iteration counts to RGB pixels
    for (idx, &iterations) in results.iter().enumerate() {
        let rgb = map_iterations_to_color(iterations, limit as u32);
        let x = idx as u32 % w;
        let y = idx as u32 / w;
        img.put_pixel(x, y, rgb);
    }
    
    Ok(img)
}
