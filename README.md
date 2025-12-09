# Photeryx

> High-performance Rust + WebAssembly image processing for modern web applications.

<p align="center">
  <img src="./docs/logo-small.png" alt="Photeryx logo" width="140" />
</p>

[![npm version](https://img.shields.io/npm/v/photeryx?color=blue)](https://www.npmjs.com/package/photeryx)
[![npm downloads](https://img.shields.io/npm/dm/photeryx)](https://www.npmjs.com/package/photeryx)
[![license](https://img.shields.io/npm/l/photeryx)](https://github.com/mehranTaslimi/photeryx/blob/main/LICENSE)
![GitHub issues](https://img.shields.io/github/issues/mehranTaslimi/photeryx)
![Node version](https://img.shields.io/node/v/photeryx)
![WebAssembly](https://img.shields.io/badge/WebAssembly-Ready-purple)

---

## 🚀 Overview

**Photeryx** is a fast, lightweight image processing pipeline powered by **Rust + WebAssembly**, designed for browsers and modern frontend apps. It supports loading multiple images, transforming them, and exporting them in various formats, all locally without backend services.

This makes it ideal for:

- Image editors
- Upload preprocessors
- Offline-first web apps
- High-performance React / Vue / Svelte applications

---

## ✨ Core Features

- Written in **Rust**, compiled to **WebAssembly**
- Manage multiple images in memory at once
- Load images from:
  - `File`
  - `URL`
  - `ArrayBuffer`
- Transformations:
  - **Rotate**
  - **Crop**
  - **Resize**
  - **Filters** (brightness, contrast, blur, sharpen, etc.)
- Export formats:
  - `JPEG`
  - `PNG`
  - `WebP`
- Export as:
  - `Uint8Array`
  - `Blob`
  - `File`
  - Base64 `data:` URL
- Manual memory control: free images when you’re done
- Zero network dependency

---

## 📦 Installation

```bash
npm install photeryx
```

---

## 🧱 TypeScript API Overview

### Image Configuration

```ts
export interface RotationConfig {
  degrees: number;
}

export interface CropConfig {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface ResizeConfig {
  max_width: number;
  max_height: number;
  mode: "fit" | "exact" | "fill";
}

export interface SharpenConfig {
  radius: number;
  threshold: number;
}

export interface FilterConfig {
  grayscale?: boolean;
  invert?: boolean;
  sharpen?: SharpenConfig | null;
  brightness?: number | null;
  contrast?: number | null;
  blur?: number | null;
}

export type ExportConfig =
  | { format: "jpeg"; quality: number }
  | { format: "png" }
  | { format: "webp" };

export interface ImageConfig {
  rotation?: RotationConfig | null;
  crop?: CropConfig | null;
  resize?: ResizeConfig | null;
  filters?: FilterConfig | null;
  export: ExportConfig;
}
```

---

## 🖼 Using Photeryx

### 1) Import & Initialize

```ts
import Photeryx, { ImageConfig } from "photeryx";

const ph = new Photeryx();
```

### 2) Load Images

```ts
const photo1 = await ph.addFromFile(fileInput.files[0]);
const photo2 = await ph.addFromUrl("https://example.com/image.jpg");
```

### 3) Configure Processing

```ts
const config: ImageConfig = {
  rotation: { degrees: 90 },
  crop: { x: 0, y: 0, width: 800, height: 600 },
  resize: { max_width: 1200, max_height: 1200, mode: "fit" },
  filters: {
    grayscale: false,
    sharpen: { radius: 2, threshold: 1 },
    brightness: 10,
    contrast: 20,
    blur: 1,
  },
  export: { format: "jpeg", quality: 85 },
};
```

### 4) Export Options

```ts
// Uint8Array
const bytes = await photo1.exportAsBytes(config);

// Blob
const blob = await photo1.exportAsBlob(config);

// File (with filename)
const file = await photo1.exportAsFile(config, "output.jpeg");

// Base64 string
const base64 = await photo1.exportAsDataUrl(config);
```

### 5) Export All Loaded Images

```ts
const allBlobs = await ph.exportAllAsBlobs(config);
```

### 6) Memory Management

Photeryx gives you full control over WebAssembly memory:

```ts
photo1.free(); // Free one image
ph.freeAll(); // Free all images
```

**⚠️ After `.free()`, the object can no longer be used.**

---

## 🧪 Browser Requirements

| Feature                     | Support  |
| --------------------------- | -------- |
| WebAssembly                 | Required |
| ES6 Modules                 | Required |
| Offscreen Canvas (optional) | Optional |

---

## 📚 Full API Reference

### Class: `Photeryx`

| Method                        | Description                     |
| ----------------------------- | ------------------------------- |
| `addFromFile(file)`           | Load an image from `File`       |
| `addFromUrl(url)`             | Fetch and load image            |
| `addFromArrayBuffer(buffer)`  | Load raw image data             |
| `photos`                      | Returns list of `Photo` objects |
| `exportAllAsBytes(config)`    | Export all as `Uint8Array[]`    |
| `exportAllAsBlobs(config)`    | Export all as `Blob[]`          |
| `exportAllAsDataUrls(config)` | Export all as Base64 strings    |
| `freeAll()`                   | Free all images in memory       |

### Class: `Photo`

| Method                           | Description               |
| -------------------------------- | ------------------------- |
| `exportAsBytes(config)`          | Export as `Uint8Array`    |
| `exportAsBlob(config)`           | Export as `Blob`          |
| `exportAsFile(config, filename)` | Export as browser `File`  |
| `exportAsDataUrl(config)`        | Export as Base64 string   |
| `free()`                         | Free memory of this image |

---

## 📄 License

Apache-2.0 © [Mehran Taslimi](https://github.com/mehranTaslimi)

---

## 🔗 Repository

[https://github.com/mehranTaslimi/photeryx](https://github.com/mehranTaslimi/photeryx)
