// Persistence adapters per SPEC.md §6.
//
// The reference implementations use the snapshot model only — every write
// re-serialises the full substrate to KMF. The WAL/incremental-update story
// from SPEC §6 is reserved for v0.2.0; until then, callers should snapshot
// at a cadence that matches their durability requirements.
//
// Adapter authors implement the small `StorageAdapter` interface below.
// Bring-your-own adapters are first-class — the Kanerva Memory Format
// (KMF) is the on-the-wire contract; the storage substrate is replaceable.

import { Smritidb, type SmritidbConfig } from "../store.js";
import { restore, snapshot } from "../snapshot.js";

export interface StorageAdapter {
  readonly kind: string;
  read(): Promise<Uint8Array | null>;
  write(bytes: Uint8Array): Promise<void>;
  remove?(): Promise<void>;
}

export type PersistentStoreConfig = SmritidbConfig & { adapter: StorageAdapter };

/**
 * Open a store backed by an adapter. If the adapter has prior contents, the
 * substrate is restored from the KMF snapshot. Otherwise a fresh store is
 * created. Either way the returned Smritidb is in-memory after this call —
 * persistence is opt-in via `persist()` or `withPersistence()`.
 */
export async function openPersistentStore(config: PersistentStoreConfig): Promise<Smritidb> {
  const { adapter, ...storeConfig } = config;
  const bytes = await adapter.read();
  if (bytes) {
    return restore(bytes, storeConfig);
  }
  return new Smritidb(storeConfig);
}

/** Write the current store state through the adapter. */
export async function persistStore(store: Smritidb, adapter: StorageAdapter): Promise<void> {
  const bytes = snapshot(store);
  await adapter.write(bytes);
}

/**
 * Wrap a store so every write (put / delete / consolidate) auto-persists.
 * Reads are unaffected. Returns the same store; the wrapping is in-place.
 *
 * Note: this debounces persists by `debounceMs` (default 100ms) so a burst of
 * writes coalesces into a single snapshot write.
 */
export function withPersistence(
  store: Smritidb,
  adapter: StorageAdapter,
  opts: { debounceMs?: number } = {},
): Smritidb {
  const debounceMs = opts.debounceMs ?? 100;
  let pending: ReturnType<typeof setTimeout> | null = null;

  const schedule = () => {
    if (pending) clearTimeout(pending);
    pending = setTimeout(() => {
      pending = null;
      void persistStore(store, adapter);
    }, debounceMs);
  };

  const originalPut = store.put.bind(store);
  const originalDelete = store.delete.bind(store);
  const originalConsolidate = store.consolidate.bind(store);

  store.put = function (...args: Parameters<typeof originalPut>) {
    const result = originalPut(...args);
    schedule();
    return result;
  } as typeof store.put;

  store.delete = function (...args: Parameters<typeof originalDelete>) {
    const result = originalDelete(...args);
    schedule();
    return result;
  } as typeof store.delete;

  store.consolidate = function (...args: Parameters<typeof originalConsolidate>) {
    const result = originalConsolidate(...args);
    schedule();
    return result;
  } as typeof store.consolidate;

  return store;
}

export { MemoryAdapter } from "./memory.js";
export { fsAdapter } from "./fs.js";
export { indexedDbAdapter } from "./indexeddb.js";
