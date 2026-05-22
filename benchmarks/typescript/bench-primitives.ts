// Primitive throughput benchmark — TypeScript (pure-JS reference) binding.

import { bind, bundle, encodeString, randomHv, similarity } from "@tanvrit/smritidb";
import {
  BIND_BATCH,
  BUNDLE_BATCH,
  D,
  ENCODE_STRING_BATCH,
  RANDOM_HV_BATCH,
  SIMILARITY_BATCH,
  measure,
  type PrimitiveResult,
} from "./bench-utils.js";

function seedBytes(s: string): Uint8Array {
  return new TextEncoder().encode(s);
}

export interface PrimitivesOut {
  random_hv: PrimitiveResult;
  bundle: PrimitiveResult;
  bind: PrimitiveResult;
  similarity: PrimitiveResult;
  encode_string: PrimitiveResult;
}

export function runPrimitives(): PrimitivesOut {
  console.log("[1/4] primitives (TypeScript)");

  // random_hv — varied seed per iter.
  const seeds = Array.from({ length: RANDOM_HV_BATCH }, (_, i) => seedBytes(`hv_${i}`));
  const r_hv = measure("random_hv", RANDOM_HV_BATCH, (i) => {
    randomHv(seeds[i % seeds.length], D);
  });

  // Pre-generate pairs for bundle / bind / similarity. The setup time is
  // significant for SIMILARITY_BATCH=10000 and we want to exclude it.
  const a_pool: Uint8Array[] = new Array(SIMILARITY_BATCH);
  const b_pool: Uint8Array[] = new Array(SIMILARITY_BATCH);
  for (let i = 0; i < SIMILARITY_BATCH; i++) {
    a_pool[i] = randomHv(seedBytes(`a${i}`), D);
    b_pool[i] = randomHv(seedBytes(`b${i}`), D);
  }

  const r_bundle = measure("bundle", BUNDLE_BATCH, (i) => {
    bundle([a_pool[i % a_pool.length], b_pool[i % b_pool.length]]);
  });

  const r_bind = measure("bind", BIND_BATCH, (i) => {
    bind(a_pool[i % a_pool.length], b_pool[i % b_pool.length]);
  });

  const r_sim = measure("similarity", SIMILARITY_BATCH, (i) => {
    similarity(a_pool[i % a_pool.length], b_pool[i % b_pool.length]);
  });

  const inputs = Array.from({ length: ENCODE_STRING_BATCH }, (_, i) => `item_${i}`);
  const r_enc = measure("encode_string", ENCODE_STRING_BATCH, (i) => {
    encodeString(inputs[i], D);
  });

  return {
    random_hv: r_hv,
    bundle: r_bundle,
    bind: r_bind,
    similarity: r_sim,
    encode_string: r_enc,
  };
}

if (import.meta.url === `file://${process.argv[1]}`) {
  const out = runPrimitives();
  console.log(JSON.stringify(out, null, 2));
}
