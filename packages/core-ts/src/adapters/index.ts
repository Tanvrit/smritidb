// Persistence adapters per SPEC.md §6 — full (Node) surface.
//
// `./core.ts` holds the bundler-safe half: the `StorageAdapter` contract, the
// store-wiring helpers, and the adapters that run anywhere (memory,
// IndexedDB). This module re-exports all of it and adds the Node-only
// adapters (`fsAdapter`, `sqliteAdapter`). Importing this file pulls in
// `node:fs/promises` and `better-sqlite3`.

export * from "./core.js";

export { fsAdapter } from "./fs.js";
export { sqliteAdapter, migrateLegacySchema } from "./sqlite.js";
export type {
  SqliteAdapterOptions,
  SqliteDatabase,
  SqliteStatement,
  SqliteStorageAdapter,
} from "./sqlite.js";
