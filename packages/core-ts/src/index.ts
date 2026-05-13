// Kanerva — TypeScript reference implementation against SPEC.md v0.1.0-draft.

export const SPEC_VERSION = "0.1.0-draft" as const;

export { Kanerva } from "./store.js";
export type {
  KanervaConfig,
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

export { KanervaError, type KanervaErrorKind } from "./errors.js";
