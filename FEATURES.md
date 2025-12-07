# Photeryx · Features

Photeryx is a fast, Rust + WebAssembly powered image pipeline for modern web apps.
Built for preparing images **before upload** and **before display**.

---

## Core Image Processing

- **Resize & scale**
  - Resize by `max_width` / `max_height`
  - Fit modes: `cover`, `contain`, `inside`, `outside`

- **Crop**
  - Center crop
  - Manual crop: `x`, `y`, `width`, `height`
  - Aspect-ratio crop: `1:1`, `4:3`, `16:9`, etc.

- **Rotate & flip**
  - Rotate 90 / 180 / 270 degrees
  - Horizontal & vertical flip

- **Auto orientation (EXIF aware)**
  - Fix sideways images based on EXIF orientation

- **Format conversion**
  - Convert between `jpeg`, `png`, `webp` (more formats planned)

- **Compression & quality**
  - Control quality (`0–100`)
  - Reasonable defaults per format

---

## Pipeline API

Chainable, async, and frontend-friendly:

```ts
const result = await photeryx
  .fromFile(file)
  .resize({ maxWidth: 1600 })
  .crop({ aspectRatio: 16 / 9, mode: "center" })
  .toFormat("webp", { quality: 80 })
  .toBlob();
```

- Pure, composable steps
- No in-place mutation
- Designed for UI apps and upload flows

---

## High-level Presets

Ready-made helpers for common use cases:

- `prepareAvatar(file)`
  - Square center crop, reasonable size & quality
- `prepareThumbnail(file)`
  - Small previews for lists / cards
- `prepareGalleryImage(file)`
  - Large but optimized images for galleries

All presets are built on top of the same pipeline primitives.

---

## Input & Output Types

**Inputs**

- `File`
- `Blob`
- `ArrayBuffer`
- Remote URL (via internal `fetch`)

**Outputs**

- `Blob`
- `Uint8Array`
- `ImageData` (optional, for canvas)
- Convenience helpers:
  - `toDataUrl()`
  - `toObjectUrl()` for direct `<img src={...} />` usage

---

## React Integration (Optional Layer)

Ergonomic hooks for React apps (optional package):

- `usePhoteryx(file, options)`
  - Returns `{ loading, error, result }`
- `usePreparedImage(file, preset)`
  - E.g. `preset: "avatar" | "thumbnail" | "gallery"`

Designed to plug directly into file inputs and previews.

---

## Utilities & Metadata (Planned)

Future additions:

- EXIF reading & stripping
- Dominant color detection
- Basic analysis (brightness, aspect info)
- Validation helpers:
  - `validateImage(file, { maxSizeMB, allowedTypes, maxWidth, maxHeight })`

---

## Design Goals

- Run heavy image work **in the browser** with WebAssembly
- Make image processing **predictable**, **declarative**, and **reusable**
- Focus on:
  - Pre-upload transformations
  - Responsive image preparation
  - Consistent behavior across projects
