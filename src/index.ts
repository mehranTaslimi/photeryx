import { export_image, load_image, free_image } from "./wasm/photeryx.js";

export interface ImageConfig {
  rotation?: RotationConfig | null;
  crop?: CropConfig | null;
  resize?: ResizeConfig | null;
  filters?: FilterConfig | null;
  export: ExportConfig;
}
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
export interface FilterConfig {
  grayscale?: boolean;
  invert?: boolean;
  sharpen?: SharpenConfig | null;
  brightness?: number | null;
  contrast?: number | null;
  blur?: number | null;
}
export interface SharpenConfig {
  radius: number;
  threshold: number;
}

export enum ExportFormat {
  Jpeg = "jpeg",
  Png = "png",
  Webp = "webp",
}

export type ExportConfig =
  | { format: "jpeg"; quality: number }
  | { format: "png" }
  | { format: "webp" };

class Photeryx {
  #id: number | undefined;

  async fromFile(file: File) {
    const buffer = await file.arrayBuffer();
    await this.fromArrayBuffer(buffer);
  }

  async fromUrl(url: string) {
    const response = await fetch(url);
    if (!response.ok)
      throw new Error(`Failed to fetch image: ${response.statusText}`);
    const buffer = await response.arrayBuffer();
    await this.fromArrayBuffer(buffer);
  }

  async fromArrayBuffer(buffer: ArrayBuffer) {
    if (this.#id !== undefined) {
      await this.free();
    }
    const uint8 = new Uint8Array(buffer);
    this.#id = load_image(uint8);
  }

  async free() {
    if (this.#id !== undefined) {
      free_image(this.#id);
      this.#id = undefined;
    }
  }

  async exportAsBytes(config: ImageConfig): Promise<Uint8Array> {
    this.#ensureLoaded();
    return export_image(this.#id!, config);
  }

  async exportAsBlob(config: ImageConfig): Promise<Blob> {
    const bytes = await this.exportAsBytes(config);
    const mime = this.#getMimeType(config.export.format);
    return new Blob([bytes as BlobPart], { type: mime });
  }

  async exportAsFile(config: ImageConfig, filename: string): Promise<File> {
    const blob = await this.exportAsBlob(config);
    return new File([blob], filename, { type: blob.type });
  }

  async exportAsDataUrl(config: ImageConfig): Promise<string> {
    const blob = await this.exportAsBlob(config);
    return new Promise((resolve) => {
      const reader = new FileReader();
      reader.onloadend = () => resolve(reader.result as string);
      reader.readAsDataURL(blob);
    });
  }

  #ensureLoaded() {
    if (this.#id === undefined) {
      throw new Error("Image not loaded. Please load an image first.");
    }
  }

  #getMimeType(format: ExportConfig["format"]) {
    switch (format) {
      case "jpeg":
        return "image/jpeg";
      case "png":
        return "image/png";
      case "webp":
        return "image/webp";
    }
  }
}

export default Photeryx;
