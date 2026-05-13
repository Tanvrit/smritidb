// Cleanup memory per SPEC.md §4. Phase 1 uses brute-force linear scan; this
// remains the reference for cross-binding conformance. Phase 2 (Rust) layers
// LSH / learned indices on top for hot-path performance.

import { similarity, type Hypervector } from "./hypervector.js";
import { dimensionMismatch } from "./errors.js";

export interface CleanupEntry {
  readonly id: string;
  readonly key: Hypervector;
}

export interface CleanupHit {
  readonly id: string;
  readonly similarity: number;
}

export function cleanupSearch(
  entries: readonly CleanupEntry[],
  cue: Hypervector,
  topK: number,
  minSimilarity: number,
): CleanupHit[] {
  const out: CleanupHit[] = [];
  for (const entry of entries) {
    if (entry.key.length !== cue.length) {
      throw dimensionMismatch(entry.key.length, cue.length);
    }
    const sim = similarity(entry.key, cue);
    if (sim >= minSimilarity) {
      out.push({ id: entry.id, similarity: sim });
    }
  }
  out.sort((a, b) => {
    if (b.similarity !== a.similarity) return b.similarity - a.similarity;
    return a.id < b.id ? -1 : a.id > b.id ? 1 : 0;
  });
  return out.slice(0, topK);
}
