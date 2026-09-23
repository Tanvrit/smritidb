// Smritidb — TypeScript reference implementation against SPEC.md v0.1.0-draft.
//
// This is the BROWSER entry (the `browser` condition in package.json
// `exports`). It is the whole public surface minus the two Node-only storage
// adapters, so a web bundler never has to resolve `node:fs/promises` or
// `better-sqlite3`. `./index.ts` is the Node entry: it re-exports everything
// here and adds `fsAdapter` + `sqliteAdapter`.

export const SPEC_VERSION = "0.1.0-draft" as const;

export { Smritidb } from "./store.js";
export type {
  SmritidbConfig,
  Item,
  Match,
  CueLike,
  PutOptions,
  RecallOptions,
  Metadata,
  Scalar,
} from "./store.js";

export {
  randomHv,
  bind,
  unbind,
  bundle,
  permute,
  similarity,
  type Hypervector,
} from "./hypervector.js";

export { encodeString, encodeEmbedding, isHypervector } from "./encode.js";

export { encodeBagOfWords, encodeWordNgrams, encodeCharNgrams } from "./text.js";
export type { TextEncodingOptions } from "./text.js";

export { SmritidbError, type SmritidbErrorKind } from "./errors.js";

export type {
  ConsolidationConfig,
  ConsolidationReport,
} from "./consolidate.js";
export { DEFAULT_CONSOLIDATION } from "./consolidate.js";

export { snapshot, restore } from "./snapshot.js";
export { KMF_SPEC_VERSION, readKmf, writeKmf } from "./kmf.js";
export type { KmfSnapshot, KmfItem, KmfHeader, KmfBlockRef, KmfBlockKind } from "./kmf.js";

export {
  openPersistentStore,
  persistStore,
  withPersistence,
  MemoryAdapter,
  indexedDbAdapter,
} from "./adapters/core.js";
export type { StorageAdapter, PersistentStoreConfig } from "./adapters/core.js";
