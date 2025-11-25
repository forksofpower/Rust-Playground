// Module declarations
mod algorithms;
mod cli;
mod error;
#[cfg(feature = "gpu")]
mod gpu;
mod image_io;
mod parsers;
mod render;
mod types;

use algorithms::get_algorithm;
use clap::Parser;
use cli::Arguments;
use error::Result;
use render::{render_fractal, RenderConfig};
use types::calculate_region;

fn main() -> Result<()> {
    let args = Arguments::parse();

    if cfg!(feature = "gpu") && args.gpu {
        render_gpu(&args)?;
    } else {
        render_cpu(&args)?;
    }

    Ok(())
}

/// Render using CPU with parallel processing
fn render_cpu(args: &Arguments) -> Result<()> {
    let region = calculate_region(args.zoom, args.center);
    let algorithm = get_algorithm(&args.algorithm);

    let config = RenderConfig::new(args.dimensions, region, args.limit, args.invert);

    let pixels = render_fractal(algorithm.as_ref(), &config);
    image_io::write_png(&args.output, &pixels, args.dimensions)?;

    Ok(())
}

/// Render using GPU acceleration
#[cfg(feature = "gpu")]
fn render_gpu(args: &Arguments) -> Result<()> {
    let img = gpu::gpu_render(args.dimensions, args.limit)?;
    let filename = format!("gpu_{}", args.output);
    img.save(&filename).map_err(|e| error::MandelbrotError::ImageError(e))?;
    Ok(())
}

#[cfg(not(feature = "gpu"))]
fn render_gpu(_args: &Arguments) -> Result<()> {
    eprintln!("GPU rendering not available. Compile with --features gpu");
    std::process::exit(1);
}
