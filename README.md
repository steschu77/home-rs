# Home Assist

[![Rust](https://github.com/steschu77/home-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/steschu77/home-rs/actions/workflows/ci.yml)

Home Assist is a minimal, high-performance Rust project that provides a lightweight operating system layer and user interface for a Raspberry Pi–based home information display.

It is designed to run on a dedicated device (e.g. a Raspberry Pi) and present photos and contextual information such as weather, time, news, calendar reminders, shopping lists, and more - without the overhead of a full desktop environment or web stack.

The project prioritizes simplicity, performance, and full control over the rendering pipeline, making it suitable for always-on devices with limited resources.

## Goals

* Minimal dependencies and overhead
* High performance and predictable behavior
* Rust-first, low-level control
* Development on Windows, Linux, or Raspberry Pi

## Features

* Photo display and story-driven slideshows
* Weather, time, and date
* News and notifications
* Calendar reminders and shopping lists
* Designed for touch, keyboard, or external input devices

## Architecture

### OS abstraction layer
Unified access to user input, messaging, and OpenGL across platforms.

### Scene Manager
Manages different scenes that describe how to layout UI elements in high-level terms (images, text, elements).

#### Photo Format

All photos are stored as WEBP images:

* Chosen as a pragmatic trade-off between:
  * JPEG (inefficient compression)
  * AVIF (excellent compression but high decode cost)
* Uses [miniwebp-rs](https://github.com/steschu77/miniwebp-rs)
  
### Layouter
Converts high-level UI descriptions into low-level rendering primitives.

### Renderer
OpenGL renderer that consumes the canvas and issues draw calls.

#### Text Rendering

Text rendering is based on multi-channel signed distance fields (MTSDF):

* Fonts are baked into an MTSDF atlas using [msdf-atlas-gen](https://github.com/Chlumsky/msdf-atlas-gen)
* Allows high-quality, resolution-independent text rendering
* Efficient GPU-side rendering with minimal CPU overhead
* Consistent visual quality across different display sizes

## Running

Running Home Assist without parameters uses the included photo album at `./assets/photos`.

```
cargo run --release
```

To use a custom photo directory, provide the path as an argument:

```
cargo run --release -- --photo-dir /path/to/photo/directory
```

## Status

🚧 Work in progress
The project is under active development. APIs and internal structure may change as the design is refined.

## License
This project is licensed under:

* MIT License