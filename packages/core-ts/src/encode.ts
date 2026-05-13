// Semantic key encoders per SPEC.md §3.5 / §3.6 / appendix A.1.

import { blake3 } from "@noble/hashes/blake3.js";
import { randomHv, bind, type Hypervector } from "./hypervector.js";

const STRING_DOMAIN = "str:";
const LEVEL_DOMAIN = "lvl:";
const LEVELS = 100;

export function encodeString(s: string, dimension: number): Hypervector {
  const seed = blake3(new TextEncoder().encode(STRING_DOMAIN + s));
  return randomHv(seed, dimension);
}

// Binary thermometer + random projection embedding -> hypervector.
// See SPEC.md appendix A.1.
export function encodeEmbedding(embedding: readonly number[], dimension: number): Hypervector {
  let acc: Hypervector = new Uint8Array(dimension);
  const enc = new TextEncoder();
  for (let i = 0; i < embedding.length; i++) {
    const raw = embedding[i]!;
    const clamped = Math.max(-1, Math.min(1, raw));
    const level = Math.round(((clamped + 1) * (LEVELS - 1)) / 2);
    const seedString = `${LEVEL_DOMAIN}${i}:${level}`;
    const seed = blake3(enc.encode(seedString));
    const hv = randomHv(seed, dimension);
    acc = bind(acc, hv);
  }
  return acc;
}

export function isHypervector(x: unknown, dimension?: number): x is Hypervector {
  if (!(x instanceof Uint8Array)) return false;
  if (dimension !== undefined && x.length !== dimension) return false;
  return true;
}
