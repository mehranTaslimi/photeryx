# Photeryx

> A small, fast Rust + WebAssembly image processor for the browser.

<p align="center">
  <img src="./docs/logo.png" alt="Photeryx logo" width="140" />
</p>

Photeryx lets you load an image in the browser, apply basic transformations in Rust,
and export an optimized image (e.g. JPEG) for upload or download.

---

## Features

- 🦀 Rust core compiled to WebAssembly
- 🖼 Load images from `File` / `ArrayBuffer`
- 🔁 Rotate (0 / 90 / 180 / 270)
- ✂️ Crop (rect-based)
- 📏 Resize (max width, aspect-ratio safe, WIP)
- 🎨 Filters:
  - Grayscale
  - Invert
  - Brighten
  - Contrast
  - Blur
- 📤 Export to encoded bytes (e.g. JPEG) for use as `Blob` / `<img>` / upload

## Status

This library is experimental and under active development.  
APIs may change until the first stable release.
