// Kanerva store — public API per SPEC.md §3.
//
// Phase 1: in-memory only. Persistence adapters land in Phase 4.

import { encodeEmbedding, encodeString, isHypervector } from "./encode.js";
import { cleanupSearch, type CleanupEntry } from "./cleanup.js";
import { invalidConfig, notFound, valueTooLarge } from "./errors.js";
import type { Hypervector } from "./hypervector.js";
import { uuidv7 } from "./uuid.js";

export type Scalar = string | number | boolean | null;
export type Metadata = Record<string, Scalar>;

export interface KanervaConfig {
  dimension?: number;
  valueCapBytes?: number;
  backend?: "memory";
  recall?: {
    defaultTopK?: number;
    defaultMinSimilarity?: number;
  };
}

export interface Item {
  readonly id: string;
  readonly key: Hypervector;
  readonly value: Uint8Array;
  readonly tags: readonly string[];
  readonly metadata: Readonly<Metadata>;
  readonly createdAt: number;
  accessCount: number;
  lastAccessedAt: number;
}

export interface Match {
  readonly item: Item;
  readonly similarity: number;
}

export type CueLike = string | readonly number[] | Hypervector;

export interface PutOptions {
  readonly id?: string;
  readonly tags?: readonly string[];
  readonly metadata?: Metadata;
}

export interface RecallOptions {
  readonly topK?: number;
  readonly minSimilarity?: number;
  readonly filter?: (item: Item) => boolean;
}

const DEFAULT_DIMENSION = 10_000;
const DEFAULT_VALUE_CAP = 16 * 1024 * 1024;
const DEFAULT_TOP_K = 10;
const DEFAULT_MIN_SIM = 0.5;

export class Kanerva {
  readonly dimension: number;
  readonly valueCapBytes: number;
  readonly backend: "memory";

  readonly #items = new Map<string, Item>();
  readonly #defaultTopK: number;
  readonly #defaultMinSim: number;
  readonly #valueEncoder = new TextEncoder();

  constructor(config: KanervaConfig = {}) {
    this.dimension = config.dimension ?? DEFAULT_DIMENSION;
    this.valueCapBytes = config.valueCapBytes ?? DEFAULT_VALUE_CAP;
    this.backend = config.backend ?? "memory";
    this.#defaultTopK = config.recall?.defaultTopK ?? DEFAULT_TOP_K;
    this.#defaultMinSim = config.recall?.defaultMinSimilarity ?? DEFAULT_MIN_SIM;

    if (this.dimension < 1024) {
      throw invalidConfig(`dimension must be >= 1024 (got ${this.dimension})`);
    }
    if (this.backend !== "memory") {
      throw invalidConfig(`unsupported backend ${this.backend}; only 'memory' in Phase 1`);
    }
  }

  put(key: CueLike, value: string | Uint8Array, opts: PutOptions = {}): Item {
    const bytes = typeof value === "string" ? this.#valueEncoder.encode(value) : value;
    if (bytes.byteLength > this.valueCapBytes) {
      throw valueTooLarge(bytes.byteLength, this.valueCapBytes);
    }
    const hv = this.#toHypervector(key);
    const now = Date.now();
    const existing = opts.id ? this.#items.get(opts.id) : undefined;
    const item: Item = {
      id: opts.id ?? uuidv7(now),
      key: hv,
      value: bytes,
      tags: opts.tags ?? [],
      metadata: opts.metadata ?? {},
      createdAt: existing?.createdAt ?? now,
      accessCount: existing?.accessCount ?? 0,
      lastAccessedAt: now,
    };
    this.#items.set(item.id, item);
    return item;
  }

  recall(cue: CueLike, opts: RecallOptions = {}): Match[] {
    const topK = opts.topK ?? this.#defaultTopK;
    const minSim = opts.minSimilarity ?? this.#defaultMinSim;
    const hv = this.#toHypervector(cue);
    const filtered: CleanupEntry[] = [];
    for (const item of this.#items.values()) {
      if (opts.filter && !opts.filter(item)) continue;
      filtered.push({ id: item.id, key: item.key });
    }
    const hits = cleanupSearch(filtered, hv, topK, minSim);
    const now = Date.now();
    const out: Match[] = [];
    for (const hit of hits) {
      const item = this.#items.get(hit.id)!;
      item.accessCount += 1;
      item.lastAccessedAt = now;
      out.push({ item, similarity: hit.similarity });
    }
    return out;
  }

  delete(id: string): boolean {
    return this.#items.delete(id);
  }

  get(id: string): Item {
    const item = this.#items.get(id);
    if (!item) throw notFound(id);
    return item;
  }

  size(): number {
    return this.#items.size;
  }

  #toHypervector(input: CueLike): Hypervector {
    if (typeof input === "string") {
      return encodeString(input, this.dimension);
    }
    if (isHypervector(input)) {
      if (input.length !== this.dimension) {
        throw invalidConfig(
          `hypervector dimension ${input.length} does not match store dimension ${this.dimension}`,
        );
      }
      return input;
    }
    if (Array.isArray(input)) {
      return encodeEmbedding(input, this.dimension);
    }
    throw invalidConfig("cue must be a string, number[], or Hypervector");
  }
}
