# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

This is a Rust learning playground containing four independent example projects demonstrating different Rust capabilities:

- **gcd**: Library and CLI for computing greatest common divisor using Euclidean algorithm
- **gcd-web**: Web service wrapper around the gcd library using Actix-web
- **mandelbrot**: Parallel Mandelbrot set renderer with optional GPU support via OpenCL
- **quickreplace**: Command-line text replacement tool using regex

Each project is a separate Cargo workspace with its own `Cargo.toml` and can be built independently.

## Building and Testing

### Building Individual Projects

Navigate to the project directory and build:
```bash
cd <project-name>/
cargo build
```

For release builds (recommended for mandelbrot):
```bash
cargo build --release
```

### Running Tests

Each project has its own test suite:
```bash
cd <project-name>/
cargo test
```

Tests are embedded in source files using `#[cfg(test)]` modules.

### Running Individual Tests

To run a specific test:
```bash
cargo test <test_name>
```

To run tests with output visible:
```bash
cargo test -- --nocapture
```

## Project-Specific Commands

### mandelbrot

Build with GPU support (requires OpenCL):
```bash
cargo build --release --features gpu
```

Run CPU rendering:
```bash
./target/release/mandelbrot \
    --output mandel.png \
    --dimensions 1920x1080 \
    --zoom 0.000940816 \
    --center "0.2613577,-0.002018128" \
    --limit 512
```

Run GPU rendering:
```bash
./target/release/mandelbrot \
    --output mandel.png \
    --dimensions 38200x21600 \
    --zoom 0.000940816 \
    --center "0.2613577,-0.002018128" \
    --limit 512 \
    --gpu
```

### gcd-web

Run the web server:
```bash
cd gcd-web/
cargo run
```

Server runs on `http://localhost:3000` with a simple HTML form interface.

### quickreplace

Run with arguments:
```bash
cargo run -- --filename <input> --output <output> <target> <replacement>
```

## Architecture Notes

### Module Structure

- **gcd**: Exports a single `gcd()` function as a library (`lib.rs`) with a separate CLI binary (`cli.rs`)
- **gcd-web**: Depends on the local gcd library via path dependency, demonstrates workspace linking
- **mandelbrot**: Modular design with separate concerns:
  - `mandelbrot.rs`: Core algorithm (escape time, coordinate mapping, rendering)
  - `parsers.rs`: Generic parsing utilities for CLI arguments
  - `gpu.rs`: OpenCL GPU rendering (feature-gated with `#[cfg(feature = "gpu")]`)
  - `mandelbrot.cl`: OpenCL kernel for GPU computation
  - `main.rs`: CLI orchestration using clap derive macros
- **quickreplace**: Single-file CLI using clap, regex, and colorized error output

### Key Patterns

**Parallel Rendering (mandelbrot)**:
Uses `crossbeam::scope` to spawn worker threads that pull bands of pixels from a shared mutex-guarded iterator. Work is distributed dynamically rather than pre-assigned.

**Feature Gates**:
The mandelbrot project uses Cargo features to conditionally compile GPU support. GPU-related code is wrapped in `#[cfg(feature = "gpu")]`.

**Error Handling**:
- gcd: Uses `assert!` for precondition validation (panics on invalid input)
- mandelbrot: Uses `Result<>` for I/O operations
- quickreplace: Custom error handling with `print_error_and_halt()` for user-friendly colored error messages

## Development Notes

- All projects use Rust 2021 edition
- The codebase uses Clap v4 with derive macros for CLI argument parsing
- Tests include both unit tests and should_panic tests for error conditions
- The mandelbrot renderer uses `crossbeam` for thread management and `num` for complex number arithmetic
