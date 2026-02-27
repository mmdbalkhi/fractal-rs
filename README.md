# Mandelbrot Renderer

A Mandelbrot set generator implemented in Rust, optimized for high-resolution output.

## Algorithm Overview

### Fractal Generation logic
The renderer generates the Mandelbrot set by iterating the complex quadratic polynomial:
$z_{n+1} = {z_n}^2 + c$

For each pixel $(x, y)$ in the output image:
1. **Coordinate Mapping**: The pixel coordinates are mapped to a point $c$ in the complex plane ($c = a + bi$).
2. **Iteration**: Starting with $z_0 = 0$, the function is applied repeatedly.
3. **Escape Condition**: If the magnitude $|z|$ exceeds a threshold (256) within the maximum number of iterations, the point is considered to "escape" to infinity.
4. **Coloring**: The number of iterations taken to escape, combined with a renormalization factor, determines the pixel color. Points that do not escape are considered part of the Mandelbrot set.

### Technical Implementation
- **Memory Management**: Images are rendered and written line-by-line to allow massive scale renders (e.g., 35,000px+) on systems with limited RAM.
- **Optimizations**: Geometric checks are used to skip interior points (Cardioid/Bulb), and binary P6 PPM output is used for fast I/O.

## Dependencies
- `rayon`: Used for parallelizing the computation of each row across all CPU cores.

## Execution

To run with full optimizations:
```bash
cargo run --release
```

## LICENSE

+ [MIT](./LICENSE)
