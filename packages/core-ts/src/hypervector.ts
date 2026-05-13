// Binary hypervector primitives per SPEC.md §1.
//
// In-memory representation: Uint8Array of length D, each byte 0 or 1 (unpacked).
// The packed bit-array form lives in the KMF wire format only.
// Phase 2 (Rust) will use packed bits + SIMD popcount; the reference impl
// optimizes for clarity.

import { blake3 } from "@noble/hashes/blake3.js";
import { dimensionMismatch } from "./errors.js";

export type Hypervector = Uint8Array;

const TIEBREAKER_DOMAIN = "smritidb/tiebreak";

export function randomHv(seed: Uint8Array, dimension: number): Hypervector {
  const bits = blake3
    .create({ dkLen: Math.ceil(dimension / 8) })
    .update(seed)
    .digest();
  const out = new Uint8Array(dimension);
  for (let i = 0; i < dimension; i++) {
    // MSB-first within each byte, per SPEC §1.1.
    const byte = bits[i >>> 3]!;
    const bit = (byte >>> (7 - (i & 7))) & 1;
    out[i] = bit;
  }
  return out;
}

export function similarity(a: Hypervector, b: Hypervector): number {
  if (a.length !== b.length) throw dimensionMismatch(a.length, b.length);
  let mismatches = 0;
  for (let i = 0; i < a.length; i++) {
    if (a[i] !== b[i]) mismatches++;
  }
  return 1 - mismatches / a.length;
}

export function bind(a: Hypervector, b: Hypervector): Hypervector {
  if (a.length !== b.length) throw dimensionMismatch(a.length, b.length);
  const out = new Uint8Array(a.length);
  for (let i = 0; i < a.length; i++) out[i] = a[i]! ^ b[i]!;
  return out;
}

// Self-inverse alias — see SPEC.md §3.3. Same op, different intent.
export const unbind = bind;

export function bundle(hvs: readonly Hypervector[]): Hypervector {
  if (hvs.length === 0) {
    throw new Error("bundle requires at least one hypervector");
  }
  const D = hvs[0]!.length;
  for (const hv of hvs) {
    if (hv.length !== D) throw dimensionMismatch(D, hv.length);
  }
  const sums = new Uint32Array(D);
  for (const hv of hvs) {
    for (let i = 0; i < D; i++) sums[i]! += hv[i]!;
  }
  const out = new Uint8Array(D);
  const n = hvs.length;
  const half = n / 2;
  for (let i = 0; i < D; i++) {
    const s = sums[i]!;
    if (s > half) {
      out[i] = 1;
    } else if (s < half) {
      out[i] = 0;
    } else {
      // Tie. Resolve deterministically per SPEC §1.3.
      out[i] = deterministicTiebreak(D, i, n);
    }
  }
  return out;
}

export function permute(hv: Hypervector, k: number): Hypervector {
  const D = hv.length;
  // Normalize k into [0, D).
  let shift = k % D;
  if (shift < 0) shift += D;
  const out = new Uint8Array(D);
  for (let i = 0; i < D; i++) out[(i + shift) % D] = hv[i]!;
  return out;
}

function deterministicTiebreak(D: number, index: number, count: number): 0 | 1 {
  const seed = new Uint8Array(TIEBREAKER_DOMAIN.length + 12);
  for (let i = 0; i < TIEBREAKER_DOMAIN.length; i++) {
    seed[i] = TIEBREAKER_DOMAIN.charCodeAt(i);
  }
  const view = new DataView(seed.buffer, TIEBREAKER_DOMAIN.length);
  view.setUint32(0, D, true);
  view.setUint32(4, index, true);
  view.setUint32(8, count, true);
  const digest = blake3(seed);
  return (digest[0]! & 1) as 0 | 1;
}
