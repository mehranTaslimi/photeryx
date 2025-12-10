import type { WorkerRequestType, WorkerResponse } from "./photeryx.worker";

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
    default: {
      const _exhaustive: never = format;
      return "application/octet-stream";
    }
  }
}

const createWorker = () => {
  return new Worker(new URL("./photeryx.worker.js", import.meta.url), {
    type: "module",
  });
};

export class Photo {
  #id: number;
  #manager: Photeryx;
  #freed = false;

  constructor(manager: Photeryx, id: number) {
    this.#manager = manager;
    this.#id = id;
  }

  get id(): number {
    return this.#id;
  }

  async exportAsBytes(config: ImageConfig): Promise<Uint8Array> {
    this.#ensureAlive();
    return this.#manager.call<Uint8Array>("exportImage", {
      id: this.#id,
      config,
    });
  }

  async exportAsBlob(config: ImageConfig): Promise<Blob> {
    this.#ensureAlive();
    const bytes = await this.exportAsBytes(config);
    const mime = getMimeType(config.export.format);
    return new Blob([bytes as BlobPart], { type: mime });
  }

  async exportAsDataUrl(config: ImageConfig): Promise<string> {
    this.#ensureAlive();
    const blob = await this.exportAsBlob(config);
    return new Promise((resolve) => {
      const reader = new FileReader();
      reader.onloadend = () => resolve(reader.result as string);
      reader.readAsDataURL(blob);
    });
  }

  async free(): Promise<void> {
    if (this.#freed) return;
    await this.#manager.call("freeImage", { id: this.#id });
    this.#freed = true;
    this.#manager._detach(this);
  }

  async _unsafeFreeWithoutDetach(): Promise<void> {
    if (this.#freed) return;
    await this.#manager.call("freeImage", { id: this.#id });
    this.#freed = true;
  }

  #ensureAlive(): void {
    if (this.#freed) {
      throw new Error("This Photo has been freed and cannot be used.");
    }
  }
}

export class Photeryx {
  #photos: Set<Photo> = new Set();
  #messageId = 0;
  #worker = createWorker();

  get photos(): readonly Photo[] {
    return Array.from(this.#photos);
  }

  call<T>(type: WorkerRequestType, payload: any): Promise<T> {
    const id = ++this.#messageId;

    return new Promise((resolve, reject) => {
      const onMessage = (event: MessageEvent<WorkerResponse>) => {
        if (event.data.id !== id) return;

        this.#worker.removeEventListener("message", onMessage);

        if (event.data.ok) {
          resolve(event.data.result as T);
        } else {
          reject(new Error(event.data.error));
        }
      };

      this.#worker.addEventListener("message", onMessage);
      this.#worker.postMessage({ id, type, payload });
    });
  }

  async addFromFile(file: File): Promise<Photo> {
    const buffer = await file.arrayBuffer();
    return this.addFromArrayBuffer(buffer);
  }

  async addFromUrl(url: string): Promise<Photo> {
    const response = await fetch(url);
    if (!response.ok) {
      throw new Error(
        `Failed to fetch image: ${response.status} ${response.statusText}`
      );
    }
    const buffer = await response.arrayBuffer();
    return this.addFromArrayBuffer(buffer);
  }

  async addFromArrayBuffer(buffer: ArrayBuffer): Promise<Photo> {
    const buf = new Uint8Array(buffer);
    const result = await this.call<number>("loadImage", { buf });
    const photo = new Photo(this, result);
    this.#photos.add(photo);
    return photo;
  }

  async exportAllAsBytes(config: ImageConfig): Promise<Uint8Array[]> {
    const photos = this.photos;
    if (!photos.length) return [];
    return Promise.all(photos.map((p) => p.exportAsBytes(config)));
  }

  async exportAllAsBlobs(config: ImageConfig): Promise<Blob[]> {
    const photos = this.photos;
    if (!photos.length) return [];
    return Promise.all(photos.map((p) => p.exportAsBlob(config)));
  }

  async exportAllAsDataUrls(config: ImageConfig): Promise<string[]> {
    const photos = this.photos;
    if (!photos.length) return [];
    return Promise.all(photos.map((p) => p.exportAsDataUrl(config)));
  }

  /**
   * Find duplicate / similar images.
   * @param threshold similarity threshold (implementation-defined, e.g. 0–100)
   * @returns groups of Photo objects that are considered duplicates
   */
  async findDuplicates(threshold: number = 90): Promise<Photo[][]> {
    const photos = this.photos;
    if (!photos.length) return [];

    const idToPhoto = new Map<number, Photo>(
      photos.map((photo) => [photo.id, photo])
    );

    const ids = new Uint32Array(photos.map((p) => p.id));

    const groups = await this.call<number[][]>("findDuplicates", {
      ids,
      threshold,
    });

    if (!groups?.length) return [];

    const photoGroups: Photo[][] = [];
    for (const group of groups) {
      if (!group.length) continue;
      const mapped: Photo[] = [];
      for (const id of group) {
        const photo = idToPhoto.get(id);
        if (!photo) {
          throw new Error(`Photo with ID ${id} not found.`);
        }
        mapped.push(photo);
      }
      if (mapped.length) {
        photoGroups.push(mapped);
      }
    }

    return photoGroups;
  }

  freeAll(): void {
    for (const photo of this.#photos) {
      photo._unsafeFreeWithoutDetach();
    }
    this.#photos.clear();
  }

  _detach(photo: Photo): void {
    this.#photos.delete(photo);
  }
}

export default Photeryx;
