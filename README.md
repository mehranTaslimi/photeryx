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
  - Base64 `data:` URL
- Duplicate detection across loaded images (returns `Photo[][]`)
- Manual memory control: free images when you’re done
- Zero network dependency

---

## 📦 Installation

```bash
npm install photeryx
```

---

## 🧱 TypeScript API Overview

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
  | {
      format: "jpeg";
      quality: number;
    }
  | {
      format: "png";
    }
  | {
      format: "webp";
    };

export interface ImageConfig {
  rotation?: RotationConfig | null;
  crop?: CropConfig | null;
  resize?: ResizeConfig | null;
  filters?: FilterConfig | null;
  export: ExportConfig;
}

export declare class Photo {
  #private;
  constructor(manager: Photeryx, id: number);
  get id(): number;
  exportAsBytes(config: ImageConfig): Promise<Uint8Array>;
  exportAsBlob(config: ImageConfig): Promise<Blob>;
  exportAsDataUrl(config: ImageConfig): Promise<string>;
  free(): void;
  _unsafeFreeWithoutDetach(): void;
}

export declare class Photeryx {
  #private;
  get photos(): readonly Photo[];
  addFromFile(file: File): Promise<Photo>;
  addFromUrl(url: string): Promise<Photo>;
  addFromArrayBuffer(buffer: ArrayBuffer): Promise<Photo>;
  exportAllAsBytes(config: ImageConfig): Promise<Uint8Array[]>;
  exportAllAsBlobs(config: ImageConfig): Promise<Blob[]>;
  exportAllAsDataUrls(config: ImageConfig): Promise<string[]>;
  findDuplicates(threshold?: number): Promise<Photo[][]>;
  freeAll(): void;
  _detach(photo: Photo): void;
}

export default Photeryx;
```

---

## 🖼 Using Photeryx

### 1) Import & Initialize

```ts
import Photeryx, { type ImageConfig } from "photeryx";

const ph = new Photeryx();
```

### 2) Load Images

```ts
const photo1 = await ph.addFromFile(fileInput.files[0]);
const photo2 = await ph.addFromUrl("https://example.com/image.jpg");

// Or from ArrayBuffer
const buffer = await someFetchOrFileApi();
const photo3 = await ph.addFromArrayBuffer(buffer);
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

### 4) Export Options (single image)

```ts
// Uint8Array
const bytes = await photo1.exportAsBytes(config);

// Blob
const blob = await photo1.exportAsBlob(config);

// Base64 string (data URL)
const base64 = await photo1.exportAsDataUrl(config);

// If you need a File instance:
const file = new File([blob], "output.jpeg", { type: "image/jpeg" });
```

### 5) Export All Loaded Images

```ts
// As Uint8Array[]
const allBytes = await ph.exportAllAsBytes(config);

// As Blob[]
const allBlobs = await ph.exportAllAsBlobs(config);

// As data URLs
const allDataUrls = await ph.exportAllAsDataUrls(config);
```

### 6) Duplicate Detection

`findDuplicates` compares all loaded photos and returns groups of `Photo` instances that are considered duplicates or very similar.

```ts
// Optionally pass a threshold (implementation-defined, e.g. 0–100)
const groups = await ph.findDuplicates(90);

// Example shape:
// [
//   [Photo, Photo],       // first duplicate group
//   [Photo, Photo, Photo] // second duplicate group
// ]

for (const group of groups) {
  console.log("Duplicate group:");
  for (const photo of group) {
    console.log("  Photo id:", photo.id);
  }
}
```

You can still access the flat list of currently loaded photos through `ph.photos`.

### 7) Memory Management

Photeryx gives you full control over WebAssembly memory:

```ts
photo1.free(); // Free one image
ph.freeAll(); // Free all images
```

**⚠️ After `.free()` or `.freeAll()`, freed objects can no longer be used.**

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

| Member / Method               | Description                                                  |
| ----------------------------- | ------------------------------------------------------------ |
| `photos: readonly Photo[]`    | Readonly list of currently loaded photos                     |
| `addFromFile(file)`           | Load an image from a `File`                                  |
| `addFromUrl(url)`             | Fetch and load image from a URL                              |
| `addFromArrayBuffer(buffer)`  | Load image from raw `ArrayBuffer`                            |
| `exportAllAsBytes(config)`    | Export all loaded images as `Uint8Array[]`                   |
| `exportAllAsBlobs(config)`    | Export all loaded images as `Blob[]`                         |
| `exportAllAsDataUrls(config)` | Export all loaded images as Base64 data URLs (`string[]`)    |
| `findDuplicates(threshold?)`  | Find duplicate/similar images, returns groups of `Photo[][]` |
| `freeAll()`                   | Free all images from WebAssembly memory                      |

### Class: `Photo`

| Member / Method           | Description                        |
| ------------------------- | ---------------------------------- |
| `id: number`              | Stable numeric ID for this photo   |
| `exportAsBytes(config)`   | Export as `Uint8Array`             |
| `exportAsBlob(config)`    | Export as `Blob`                   |
| `exportAsDataUrl(config)` | Export as Base64 data URL `string` |
| `free()`                  | Free memory of this image          |

---

## 📄 License

Apache-2.0 © [Mehran Taslimi](https://github.com/mehranTaslimi)

---

## 🔗 Repository

[https://github.com/mehranTaslimi/photeryx](https://github.com/mehranTaslimi/photeryx)
