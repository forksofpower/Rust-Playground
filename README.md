# Rust Playground

## Mandelbrot
### Build & Setup
Navigate to the mandelbrot example and build a release. Then copy the release to your local bin.
```
$ cd mandelbrot/
$ cargo build --release
$ cp target/release/mandelbrot ~/.local/bin
```
### Usage
Examples:

**CPU Rendering**
```shell
$ mandelbrot \
    --output mandel-3.png \
    --dimensions 1920x1080 \
    --zoom 0.000940816 \
    --center "0.2613577,-0.002018128" \
    --limit 512
```

**GPU Rendering**
```shell
$ mandelbrot \
    --output mandel-3.png \
    --dimensions 38200x21600 \
    --zoom 0.000940816 \
    --center "0.2613577,-0.002018128" \
    --limit 512
    --gpu
```