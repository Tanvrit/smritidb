// Hebbian consolidation per SPEC.md §5.
//
// Phase 1 implementation:
//   - Tracks pairwise co-activation across recall calls in a bounded
//     sliding window.
//   - Pulls co-activated keys closer by flipping a deterministically-chosen
//     subset of disagreeing bits (bounded so similarity moves by at most
//     `maxSimDelta` per pass).
//   - Marks cold items (old + rarely accessed). Bundled cold-attic
//     summarization is deferred to v0.2.0; for now cold items are simply
//     flagged.
//
// All operations are deterministic given identical inputs, which is what
// makes the substrate replayable from a KMF snapshot + access log.

import { blake3 } from "@noble/hashes/blake3.js";
import type { Hypervector } from "./hypervector.js";

export interface ConsolidationConfig {
  readonly windowSize: number;
  readonly pullThreshold: number;
  readonly maxSimDelta: number;
  readonly coldDays: number;
  readonly coldMinAccess: number;
}

export const DEFAULT_CONSOLIDATION: ConsolidationConfig = {
  windowSize: 1000,
  pullThreshold: 32,
  maxSimDelta: 0.02,
  coldDays: 30,
  coldMinAccess: 3,
};

export interface ConsolidationReport {
  readonly pairsPulled: number;
  readonly coldItemsFlagged: number;
  readonly bitsFlipped: number;
}

export class CoactivationTracker {
  readonly #window: Array<readonly string[]> = [];
  readonly #pairs = new Map<string, Map<string, number>>();
  readonly #size: number;

  constructor(size: number) {
    this.#size = size;
  }

  record(idsInBatch: readonly string[]): void {
    if (idsInBatch.length < 2) {
      this.#window.push(idsInBatch);
      this.#trim();
      return;
    }
    const sorted = [...idsInBatch].sort();
    for (let i = 0; i < sorted.length; i++) {
      for (let j = i + 1; j < sorted.length; j++) {
        const a = sorted[i]!;
        const b = sorted[j]!;
        const row = this.#pairs.get(a) ?? new Map<string, number>();
        row.set(b, (row.get(b) ?? 0) + 1);
        this.#pairs.set(a, row);
      }
    }
    this.#window.push(sorted);
    this.#trim();
  }

  #trim(): void {
    while (this.#window.length > this.#size) {
      const expired = this.#window.shift()!;
      if (expired.length < 2) continue;
      for (let i = 0; i < expired.length; i++) {
        for (let j = i + 1; j < expired.length; j++) {
          const a = expired[i]!;
          const b = expired[j]!;
          const row = this.#pairs.get(a);
          if (!row) continue;
          const next = (row.get(b) ?? 0) - 1;
          if (next <= 0) row.delete(b);
          else row.set(b, next);
          if (row.size === 0) this.#pairs.delete(a);
        }
      }
    }
  }

  pairsAtOrAbove(threshold: number): Array<{ a: string; b: string; count: number }> {
    const out: Array<{ a: string; b: string; count: number }> = [];
    for (const [a, row] of this.#pairs) {
      for (const [b, count] of row) {
        if (count >= threshold) out.push({ a, b, count });
      }
    }
    out.sort((x, y) => y.count - x.count || x.a.localeCompare(y.a) || x.b.localeCompare(y.b));
    return out;
  }

  reset(): void {
    this.#window.length = 0;
    this.#pairs.clear();
  }
}

// Move two hypervectors closer in Hamming space by flipping a bounded number
// of disagreeing bits. The choice of which bits to flip is fully
// deterministic — derived from BLAKE3 over (a, b, salt) — so consolidation is
// replayable from snapshot.
export function pullCloser(
  a: Hypervector,
  b: Hypervector,
  maxSimDelta: number,
  salt: number,
): { a: Hypervector; b: Hypervector; bitsFlipped: number } {
  const D = a.length;
  if (b.length !== D) throw new Error("dimension mismatch in pullCloser");

  const disagree: number[] = [];
  for (let i = 0; i < D; i++) if (a[i] !== b[i]) disagree.push(i);

  const maxFlips = Math.max(1, Math.floor(maxSimDelta * D));
  const toFlip = Math.min(disagree.length, maxFlips);

  // Derive a deterministic ordering over disagreeing bits.
  const seed = new Uint8Array(8);
  new DataView(seed.buffer).setBigUint64(0, BigInt(salt), true);
  const digest = blake3(seed, { dkLen: Math.max(64, toFlip * 4) });

  disagree.sort((x, y) => {
    const dx = digest[x % digest.length]!;
    const dy = digest[y % digest.length]!;
    return dx - dy || x - y;
  });

  const aOut = new Uint8Array(a);
  const bOut = new Uint8Array(b);

  // Flip alternately: half toward b, half toward a. Net effect: both move ~equally.
  for (let i = 0; i < toFlip; i++) {
    const bit = disagree[i]!;
    if (i % 2 === 0) {
      aOut[bit] = b[bit]!;
    } else {
      bOut[bit] = a[bit]!;
    }
  }

  return { a: aOut, b: bOut, bitsFlipped: toFlip };
}

export interface ColdCandidate {
  readonly id: string;
  readonly accessCount: number;
  readonly lastAccessedAt: number;
}

export function flagColdItems(
  items: readonly ColdCandidate[],
  config: Pick<ConsolidationConfig, "coldDays" | "coldMinAccess">,
  now: number,
): string[] {
  const cutoff = now - config.coldDays * 24 * 60 * 60 * 1000;
  return items
    .filter((it) => it.accessCount < config.coldMinAccess && it.lastAccessedAt < cutoff)
    .map((it) => it.id);
}
