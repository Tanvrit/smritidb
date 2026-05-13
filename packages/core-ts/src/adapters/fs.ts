import type { StorageAdapter } from "./index.js";

/**
 * Filesystem adapter — writes the substrate as a single KMF file at `path`.
 *
 * Writes go through a sibling `.tmp` file + `rename` for atomicity on POSIX
 * systems. If the temp write fails, the previous snapshot is preserved.
 */
export function fsAdapter(path: string): StorageAdapter {
  const tmpPath = `${path}.tmp`;

  return {
    kind: "fs",

    async read(): Promise<Uint8Array | null> {
      const fs = await import("node:fs/promises");
      try {
        const buf = await fs.readFile(path);
        return new Uint8Array(buf.buffer, buf.byteOffset, buf.byteLength);
      } catch (err) {
        if ((err as NodeJS.ErrnoException).code === "ENOENT") return null;
        throw err;
      }
    },

    async write(bytes: Uint8Array): Promise<void> {
      const fs = await import("node:fs/promises");
      await fs.writeFile(tmpPath, bytes);
      await fs.rename(tmpPath, path);
    },

    async remove(): Promise<void> {
      const fs = await import("node:fs/promises");
      try {
        await fs.unlink(path);
      } catch (err) {
        if ((err as NodeJS.ErrnoException).code !== "ENOENT") throw err;
      }
    },
  };
}
