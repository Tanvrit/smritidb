// Emits the byte-parity fixture consumed by
// `packages/core-ts/src/kmf.write-parity.test.ts`.
//
// The fixture is a small deterministic KMF blob (10 items, D=128) produced by
// the TS reference writer. The companion Rust test
// `packages/core-rs/tests/kmf_cross_impl.rs` proves the Rust core's writer
// produces byte-identical output for the same logical input, so this fixture
// is effectively canonical for both implementations.
//
// Regenerate after any change to the KMF wire format:
//
//   cd packages/core-ts && pnpm build
//   node tests/conformance/emit_kmf_write_parity_fixture.mjs
//
// The fixture lives at tests/conformance/kmf_write_parity_fixture.bin and is
// checked into the repo. Any future drift in the TS writer causes
// kmf.write-parity.test.ts to fail with a precise byte-offset diff.

import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";

import { writeKmf } from "../../packages/core-ts/dist/kmf.js";
import { randomHv } from "../../packages/core-ts/dist/hypervector.js";

const here = dirname(fileURLToPath(import.meta.url));
const DIM = 128;
const N = 10;
const CREATED_AT = 1_700_000_000_000;

const items = [];
for (let i = 0; i < N; i++) {
  const ii = i.toString().padStart(2, "0");
  items.push({
    id: `item_${ii}`,
    key: randomHv(new TextEncoder().encode(`seed-${ii}`), DIM),
    value: new TextEncoder().encode(`value-${ii}`),
    tags: ["bench", `g${i % 3}`],
    metadata: { rank: i, kind: "letter" },
    createdAt: CREATED_AT + i * 1_000,
    accessCount: i,
    lastAccessedAt: CREATED_AT + i * 1_000 + 500,
  });
}

const bytes = writeKmf({ dimension: DIM, createdAt: CREATED_AT, items });
const outPath = resolve(here, "kmf_write_parity_fixture.bin");
writeFileSync(outPath, bytes);
console.log(`wrote ${bytes.length}-byte parity fixture to ${outPath}`);
