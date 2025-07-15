# vis-rs

A WebAssembly-based visualization library built with Rust and Bevy.

Adapted from https://github.com/cxreiff/vite_nannou_template

## Features

- **Parallel Processing**: Uses rayon with wasm-bindgen-rayon for parallel processing of boids
- **Spatial Grid**: Efficient neighbor lookup using spatial partitioning
- **Flocking Simulation**: Implements boid flocking behavior (separation, alignment, cohesion)

## Building

```bash
npm run build    # Production build
npm run dev      # Development build with hot reload
```

## Browser Requirements

For parallel processing to work, your browser must support:
- SharedArrayBuffer
- Atomics
- Cross-origin isolation (COOP/COEP headers)

The development server is configured with the necessary headers.

## Architecture

- **Boid**: Individual agent with position, velocity, and acceleration
- **SpatialGrid**: Spatial partitioning for efficient neighbor queries
- **FlockingParams**: Configuration for boid behavior
- **Parallel Processing**: Boid updates are processed in parallel using rayon when available

## Performance

With rayon enabled by default, the simulation can handle significantly more boids while maintaining smooth performance, especially on multi-core systems.
