import { export_image, load_image, free_image } from "./wasm/photeryx.js";
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

function getMimeType(format: ExportConfig["format"]): string {
  switch (format) {
    case "jpeg":
      return "image/jpeg";
    case "png":
      return "image/png";
    case "webp":
      return "image/webp";
  }
}

export class Photo {
  #id: number;
  #manager: Photeryx;
  #freed = false;

  constructor(manager: Photeryx, id: number) {
    this.#manager = manager;
    this.#id = id;
  }

  async exportAsBytes(config: ImageConfig): Promise<Uint8Array> {
    this.#ensureAlive();
    return export_image(this.#id, config);
  }

  async exportAsBlob(config: ImageConfig): Promise<Blob> {
    const bytes = await this.exportAsBytes(config);
    const mime = getMimeType(config.export.format);
    return new Blob([bytes as BlobPart], { type: mime });
  }

  async exportAsDataUrl(config: ImageConfig): Promise<string> {
    const blob = await this.exportAsBlob(config);
    return new Promise((resolve) => {
      const reader = new FileReader();
      reader.onloadend = () => resolve(reader.result as string);
      reader.readAsDataURL(blob);
    });
  }

  free() {
    if (this.#freed) return;
    free_image(this.#id);
    this.#freed = true;
    this.#manager._detach(this);
  }

  _unsafeFreeWithoutDetach() {
    if (this.#freed) return;
    free_image(this.#id);
    this.#freed = true;
  }

  #ensureAlive() {
    if (this.#freed) {
      throw new Error("This Photo has been freed and cannot be used.");
    }
  }
}

export class Photeryx {
  #photos: Set<Photo> = new Set();

  get photos(): readonly Photo[] {
    return Array.from(this.#photos);
  }

  async addFromFile(file: File): Promise<Photo> {
    const buffer = await file.arrayBuffer();
    return this.addFromArrayBuffer(buffer);
  }

  async addFromUrl(url: string): Promise<Photo> {
    const response = await fetch(url);
    if (!response.ok) {
      throw new Error(`Failed to fetch image: ${response.statusText}`);
    }
    const buffer = await response.arrayBuffer();
    return this.addFromArrayBuffer(buffer);
  }

  async addFromArrayBuffer(buffer: ArrayBuffer): Promise<Photo> {
    const bytes = new Uint8Array(buffer);
    const id = load_image(bytes);

    const photo = new Photo(this, id);
    this.#photos.add(photo);
    return photo;
  }

  async exportAllAsBytes(config: ImageConfig): Promise<Uint8Array[]> {
    const tasks = Array.from(this.#photos).map((p) => p.exportAsBytes(config));
    return Promise.all(tasks);
  }

  async exportAllAsBlobs(config: ImageConfig): Promise<Blob[]> {
    const tasks = Array.from(this.#photos).map((p) => p.exportAsBlob(config));
    return Promise.all(tasks);
  }

  async exportAllAsDataUrls(config: ImageConfig): Promise<string[]> {
    const tasks = Array.from(this.#photos).map((p) =>
      p.exportAsDataUrl(config)
    );
    return Promise.all(tasks);
  }

  freeAll() {
    for (const photo of this.#photos) {
      photo._unsafeFreeWithoutDetach();
    }
    this.#photos.clear();
  }

  findDuplicates() {}

  _detach(photo: Photo) {
    this.#photos.delete(photo);
  }
}

export default Photeryx;
