import { export_image, load_image } from "./wasm/photeryx.js";

export interface ImageConfig {
  rotation?: RotationConfig;
  crop?: CropConfig;
  resize?: ResizeConfig;
  filters?: FilterConfig;
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
}

export interface FilterConfig {
  grayscale?: boolean;
  invert?: boolean;
  sharpen?: SharpenConfig;
  brightness?: number;
  contrast?: number;
  blur?: number;
}

export interface SharpenConfig {
  radius: number;
  threshold: number;
}

export interface ExportConfig {
  format: "jpeg" | "png" | "webp";
  quality: number;
}

class Photeryx {
  #id: number | undefined;

  async fromFile(image: File): Promise<void> {
    if (this.#id) throw new Error("Image already loaded.");

    const buffer = await image.arrayBuffer();
    const uint8 = new Uint8Array(buffer);

    this.#id = load_image(uint8);
  }

  async exportImage(config: ImageConfig): Promise<Uint8Array> {
    if (typeof this.#id === "undefined") {
      throw new Error("Image not loaded. Please load an image first.");
    }

    return export_image(this.#id, config);
  }
}

export default Photeryx;
