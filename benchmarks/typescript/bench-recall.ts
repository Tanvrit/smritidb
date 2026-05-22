// Recall latency benchmark — TypeScript binding.

import { performance } from "node:perf_hooks";
import { Smritidb, encodeString } from "@tanvrit/smritidb";

import {
  D,
  RECALL_NS,
  RECALL_QUERIES,
  WARMUP_ITERS,
  percentile,
} from "./bench-utils.js";

export interface RecallResult {
  n: number;
  queries: number;
  p50_ms: number;
  p99_ms: number;
  p999_ms: number;
  insert_seconds: number;
}

export function runRecall(): RecallResult[] {
  console.log("[2/4] recall (TypeScript)");
  const out: RecallResult[] = [];

  for (const n of RECALL_NS) {
    const store = new Smritidb({ dimension: D });
    const t0 = performance.now();
    for (let i = 0; i < n; i++) {
      store.put(`item_${i}`, `value for item ${i}`);
    }
    const insertSeconds = (performance.now() - t0) / 1000;

    // Pre-encode cues so the timed region measures recall in isolation.
    const cues: Uint8Array[] = new Array(RECALL_QUERIES);
    for (let i = 0; i < RECALL_QUERIES; i++) {
      cues[i] = encodeString(`item_${i % n}`, D);
    }

    for (let i = 0; i < WARMUP_ITERS; i++) {
      store.recall(cues[0], { topK: 10, minSimilarity: 0.5 });
    }

    const samples_ms: number[] = new Array(cues.length);
    for (let i = 0; i < cues.length; i++) {
      const t = performance.now();
      store.recall(cues[i], { topK: 10, minSimilarity: 0.5 });
      samples_ms[i] = performance.now() - t;
    }

    const p50 = percentile(samples_ms, 0.5);
    const p99 = percentile(samples_ms, 0.99);
    const p999 = percentile(samples_ms, 0.999);
    console.log(
      `  N=${String(n).padStart(5)} insert=${insertSeconds.toFixed(3)}s ` +
        `p50=${p50.toFixed(3)}ms p99=${p99.toFixed(3)}ms p999=${p999.toFixed(3)}ms`,
    );
    out.push({
      n,
      queries: cues.length,
      p50_ms: p50,
      p99_ms: p99,
      p999_ms: p999,
      insert_seconds: insertSeconds,
    });
  }
  return out;
}

if (import.meta.url === `file://${process.argv[1]}`) {
  const out = runRecall();
  console.log(JSON.stringify(out, null, 2));
}
