// Shared utilities for the TS benchmark scripts.

import { performance } from "node:perf_hooks";

export const D = 10_000;
export const RANDOM_HV_BATCH = 1_000;
export const BUNDLE_BATCH = 100;
export const BIND_BATCH = 100;
export const SIMILARITY_BATCH = 10_000;
export const ENCODE_STRING_BATCH = 10_000;
export const RECALL_NS = [100, 1_000, 10_000] as const;
export const RECALL_QUERIES = 1_000;
export const PERSIST_N = 10_000;
export const WAL_APPENDS = 1_000;
export const WARMUP_ITERS = 3;

export interface PrimitiveResult {
  ops_per_sec: number;
  iters: number;
  wall_seconds: number;
}

/**
 * Measure a body function. We run WARMUP_ITERS throw-away iterations first
 * to give V8's inline caches and JIT a chance to settle. The measured loop
 * then runs `iters` times and the wall-clock is computed from `performance.now()`.
 */
export function measure(
  name: string,
  iters: number,
  body: (i: number) => void,
): PrimitiveResult {
  for (let i = 0; i < WARMUP_ITERS; i++) body(0);
  const t0 = performance.now();
  for (let i = 0; i < iters; i++) body(i);
  const wall = (performance.now() - t0) / 1000;
  const ops = iters / wall;
  console.log(
    `  ${name}: ${ops.toLocaleString("en-US", { maximumFractionDigits: 0 })} ops/s ` +
      `(${wall.toFixed(3)}s for ${iters} iters)`,
  );
  return { ops_per_sec: ops, iters, wall_seconds: wall };
}

export function percentile(samples: number[], p: number): number {
  if (samples.length === 0) return 0;
  const s = [...samples].sort((a, b) => a - b);
  const idx = Math.round((s.length - 1) * p);
  return s[Math.min(idx, s.length - 1)];
}
