import type { StorageAdapter } from "./index.js";

/** Ephemeral in-memory adapter. Useful in tests and as a reference implementation. */
export class MemoryAdapter implements StorageAdapter {
  readonly kind = "memory" as const;
  #buffer: Uint8Array | null;

  constructor(initial?: Uint8Array) {
    this.#buffer = initial ? new Uint8Array(initial) : null;
  }

  async read(): Promise<Uint8Array | null> {
    return this.#buffer ? new Uint8Array(this.#buffer) : null;
  }

  async write(bytes: Uint8Array): Promise<void> {
    this.#buffer = new Uint8Array(bytes);
  }

  async remove(): Promise<void> {
    this.#buffer = null;
  }
}
