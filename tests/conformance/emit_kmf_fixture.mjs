// Emits a deterministic KMF fixture from the TypeScript reference. Run once
// after building `@tanvrit/smritidb`; the resulting `kmf_fixture.bin` and the
// companion `kmf_fixture.json` are consumed by the Rust core's cross-impl
// KMF conformance test.
//
//   cd packages/core-ts && pnpm build
//   node tests/conformance/emit_kmf_fixture.mjs
//
// The fixture is intentionally tiny — a handful of items at D=1024 — so the
// binary lives comfortably in the repo without bloating it. We use literal
// hypervectors and seeded `randomHv` so the bytes are stable across machines.

import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";

import { writeKmf } from "../../packages/core-ts/dist/kmf.js";
import { randomHv } from "../../packages/core-ts/dist/hypervector.js";

const here = dirname(fileURLToPath(import.meta.url));
const DIM = 1024;

const items = [
  {
    id: "alpha",
    seed: "key:alpha",
    value: "alpha-value",
    tags: ["greek", "first"],
    metadata: { rank: 1, kind: "letter", flag: true, note: null },
    createdAt: 1_700_000_000_000,
    accessCount: 0,
    lastAccessedAt: 1_700_000_000_000,
  },
  {
    id: "beta",
    seed: "key:beta",
    value: "beta-value",
    tags: ["greek"],
    metadata: { rank: 2, kind: "letter" },
    createdAt: 1_700_000_001_000,
    accessCount: 3,
    lastAccessedAt: 1_700_000_005_000,
  },
  {
    id: "gamma",
    seed: "key:gamma",
    value: "gamma-value",
    tags: [],
    metadata: {},
    createdAt: 1_700_000_002_000,
    accessCount: 0,
    lastAccessedAt: 1_700_000_002_000,
  },
];

const kmfItems = items.map((it) => ({
  id: it.id,
  key: randomHv(new TextEncoder().encode(it.seed), DIM),
  value: new TextEncoder().encode(it.value),
  tags: it.tags,
  metadata: it.metadata,
  createdAt: it.createdAt,
  accessCount: it.accessCount,
  lastAccessedAt: it.lastAccessedAt,
}));

const bytes = writeKmf({
  dimension: DIM,
  createdAt: 1_700_000_010_000,
  items: kmfItems,
});

writeFileSync(resolve(here, "kmf_fixture.bin"), bytes);
writeFileSync(
  resolve(here, "kmf_fixture.json"),
  JSON.stringify(
    {
      dimension: DIM,
      created_at: 1_700_000_010_000,
      items: items.map((it) => ({
        id: it.id,
        seed: it.seed,
        value: it.value,
        tags: it.tags,
        metadata: it.metadata,
        created_at: it.createdAt,
        access_count: it.accessCount,
        last_accessed_at: it.lastAccessedAt,
      })),
    },
    null,
    2,
  ) + "\n",
);

console.log(`wrote ${bytes.length} byte fixture to kmf_fixture.bin`);
