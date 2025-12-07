# Photeryx

> A small, fast Rust + WebAssembly image processor for the browser.

<p align="center">
  <img src="./docs/logo-small.png" alt="Photeryx logo" width="140" />
</p>

[![npm version](https://img.shields.io/npm/v/photeryx?color=blue)](https://www.npmjs.com/package/photeryx)
[![npm downloads](https://img.shields.io/npm/dm/photeryx)](https://www.npmjs.com/package/photeryx)
[![license](https://img.shields.io/npm/l/photeryx)](https://github.com/mehranTaslimi/photeryx/blob/main/LICENSE)
![GitHub issues](https://img.shields.io/github/issues/mehranTaslimi/photeryx)
![Node version](https://img.shields.io/node/v/photeryx)
![WebAssembly](https://img.shields.io/badge/wasm-supported-brightgreen)

**Photeryx** is a fast **Rust + WebAssembly** image processing pipeline for the browser. It’s designed for **high-performance image manipulation** before upload or display, supporting rotation, crop, resize, filters, and multiple export formats.

---

## Features

- 🦀 Rust core compiled to WebAssembly
- 🖼 Load images from `File`, `URL`, or `ArrayBuffer`
- 🔁 Rotate (0 / 90 / 180 / 270)
- ✂️ Crop (rect-based)
- 📏 Resize (max width, aspect-ratio safe)
- 🎨 Filters:

  - Grayscale
  - Invert
  - Brighten
  - Contrast
  - Blur
  - Sharpen (with radius and threshold)

- 📤 Export images as:

  - `Uint8Array` (`exportAsBytes`)
  - `Blob` (`exportAsBlob`)
  - `File` (`exportAsFile`)
  - Base64 `data:` URL (`exportAsDataUrl`)

- Supports **JPEG**, **PNG**, and **WebP**
- Built for modern frontend applications with **TypeScript + WASM**

---

## Status

This library is experimental and under active development. APIs may change until the first stable release.

---

## Installation

```bash
npm install photeryx
```

---

## Usage

```ts
import Photeryx, { ImageConfig } from "photeryx";

const config: ImageConfig = {
  rotation: { degrees: 90 },
  resize: { max_width: 800 },
  filters: { grayscale: true },
  export: { format: "png", quality: 90 },
};

async function processImage() {
  const photeryx = new Photeryx();

  // Load image from a File
  const file =
    document.querySelector<HTMLInputElement>("#fileInput")!.files![0];
  await photeryx.fromFile(file);

  // Or load image from URL
  // await photeryx.fromUrl("https://example.com/image.jpg");

  // Export as File
  const outputFile = await photeryx.exportAsFile(config, "output.png");

  // Export as Blob
  const blob = await photeryx.exportAsBlob(config);

  // Export as Uint8Array
  const bytes = await photeryx.exportAsBytes(config);

  // Export as Data URL
  const dataUrl = await photeryx.exportAsDataUrl(config);

  console.log({ outputFile, blob, bytes, dataUrl });
}

processImage();
```

---

## API

### Loading Images

| Method                                 | Description                             |
| -------------------------------------- | --------------------------------------- |
| `fromFile(file: File)`                 | Load image from a File object           |
| `fromUrl(url: string)`                 | Load image from a remote URL            |
| `fromArrayBuffer(buffer: ArrayBuffer)` | Load image from an ArrayBuffer directly |

### Exporting Images

| Method                                                | Description                         |
| ----------------------------------------------------- | ----------------------------------- |
| `exportAsBytes(config: ImageConfig)`                  | Returns a `Uint8Array` of the image |
| `exportAsBlob(config: ImageConfig)`                   | Returns a `Blob`                    |
| `exportAsFile(config: ImageConfig, filename: string)` | Returns a `File`                    |
| `exportAsDataUrl(config: ImageConfig)`                | Returns a Base64 `data:` URL        |

---

## Example Filters

```ts
const filters = {
  grayscale: true,
  invert: false,
  sharpen: { radius: 2, threshold: 1 },
  brightness: 10,
  contrast: 15,
  blur: 1.5,
};
```

---

## License

Apache-2.0 © [Mehran Taslimi](https://github.com/mehranTaslimi)

---

## Repository

[https://github.com/mehranTaslimi/photeryx](https://github.com/mehranTaslimi/photeryx)
