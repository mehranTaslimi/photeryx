/// <reference lib="webworker" />

import type { ImageConfig } from "./index.js";
import {
  export_image,
  load_image,
  free_image,
  find_duplicates,
} from "./wasm/photeryx.js";

export type WorkerRequest =
  | {
      id: number;
      type: "loadImage";
      payload: { buf: Uint8Array };
    }
  | {
      id: number;
      type: "exportImage";
      payload: { id: number; config: ImageConfig };
    }
  | {
      id: number;
      type: "freeImage";
      payload: { id: number };
    }
  | {
      id: number;
      type: "findDuplicates";
      payload: { ids: Uint32Array<ArrayBuffer>; threshold: number };
    };

export type WorkerRequestType = WorkerRequest["type"];

export type WorkerResponse =
  | {
      id: number;
      ok: true;
      result?: unknown;
    }
  | {
      id: number;
      ok: false;
      error: string;
    };

const ctx = self as unknown as DedicatedWorkerGlobalScope;

ctx.onmessage = async (event: MessageEvent<WorkerRequest>) => {
  const reply = (msg: WorkerResponse) => ctx.postMessage(msg);

  const {
    data: { id, payload, type },
  } = event;

  try {
    switch (type) {
      case "loadImage": {
        const result = await load_image(payload.buf);
        reply({ id, ok: true, result });
        break;
      }
      case "exportImage": {
        const result = await export_image(payload.id, payload.config);
        reply({ id, ok: true, result });
        break;
      }
      case "freeImage": {
        await free_image(payload.id);
        reply({ id, ok: true, result: null });
        break;
      }
      case "findDuplicates": {
        const result = await find_duplicates(payload.ids, payload.threshold);
        reply({ id, ok: true, result });
        break;
      }
    }
  } catch (error) {
    reply({
      id: event.data.id,
      ok: false,
      error: (error as Error).message,
    });
  }
};
