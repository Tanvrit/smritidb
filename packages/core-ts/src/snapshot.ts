// Phase 1 in-memory snapshot/restore via KMF.
// Higher-level wrapper around kmf.ts; bridges the Store API to the wire format.

import { Smritidb, type Item, type SmritidbConfig } from "./store.js";
import { readKmf, writeKmf, type KmfItem, type KmfSnapshot } from "./kmf.js";

export function snapshot(store: Smritidb): Uint8Array {
  const items = store.itemsSnapshot();
  const kmfItems: KmfItem[] = items.map((it) => ({
    id: it.id,
    key: it.key,
    value: it.value,
    tags: it.tags,
    metadata: it.metadata as Record<string, string | number | boolean | null>,
    createdAt: it.createdAt,
    accessCount: it.accessCount,
    lastAccessedAt: it.lastAccessedAt,
  }));
  const snap: KmfSnapshot = {
    dimension: store.dimension,
    createdAt: Date.now(),
    items: kmfItems,
  };
  return writeKmf(snap);
}

export function restore(
  bytes: Uint8Array,
  config: Omit<SmritidbConfig, "dimension"> = {},
): Smritidb {
  const snap = readKmf(bytes);
  const items: Item[] = snap.items.map((it) => ({
    id: it.id,
    key: it.key,
    value: it.value,
    tags: it.tags,
    metadata: it.metadata,
    createdAt: it.createdAt,
    accessCount: it.accessCount,
    lastAccessedAt: it.lastAccessedAt,
    cold: false,
  }));
  return Smritidb.fromItems({ ...config, dimension: snap.dimension }, items);
}
