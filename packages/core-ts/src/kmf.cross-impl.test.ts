// KMF cross-implementation smoke test.
//
// The Rust core (`packages/core-rs`) parses the same `kmf_fixture.bin` blob the
// TypeScript reference emits at `tests/conformance/emit_kmf_fixture.mjs`. This
// test runs the symmetric path: reading the canonical TS-emitted fixture back
// in via the TS reader and asserting the invariants documented in
// `kmf_fixture.json`. Combined with the Rust-side parse test, this confirms the
// on-disk format is round-trip stable across both implementations.
//
// When a parallel Rust-emitted fixture (e.g. `rust_kmf_fixture.bin`) lands in
// the conformance corpus, swap the readFileSync target below — the rest of the
// assertions are byte-identical contracts pulled from the spec.

import { strict as assert } from "node:assert";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, it } from "node:test";

import { readKmf, KMF_SPEC_VERSION } from "./kmf.js";
import { randomHv } from "./hypervector.js";

const here = dirname(fileURLToPath(import.meta.url));
const fixtureBinPath = resolve(here, "../../../tests/conformance/kmf_fixture.bin");
const fixtureJsonPath = resolve(here, "../../../tests/conformance/kmf_fixture.json");

interface FixtureItem {
  id: string;
  seed: string;
  value: string;
  tags: string[];
  metadata: Record<string, string | number | boolean | null>;
  created_at: number;
  access_count: number;
  last_accessed_at: number;
}

interface Fixture {
  dimension: number;
  created_at: number;
  items: FixtureItem[];
}

const fixture: Fixture = JSON.parse(readFileSync(fixtureJsonPath, "utf8"));
const fixtureBytes = readFileSync(fixtureBinPath);

describe("KMF cross-impl", () => {
  it("declares the spec version this reader understands", () => {
    assert.equal(KMF_SPEC_VERSION, "0.1.0");
  });

  it("reads the canonical KMF fixture without error", () => {
    const parsed = readKmf(new Uint8Array(fixtureBytes));
    assert.equal(parsed.dimension, fixture.dimension);
    assert.equal(parsed.createdAt, fixture.created_at);
    assert.equal(parsed.items.length, fixture.items.length);
  });

  it("preserves fixture invariants byte-for-byte", () => {
    const parsed = readKmf(new Uint8Array(fixtureBytes));
    assert.equal(parsed.dimension, 1024);
    assert.equal(parsed.items.length, 3);

    const decoder = new TextDecoder();
    for (let i = 0; i < fixture.items.length; i++) {
      const expected = fixture.items[i]!;
      const got = parsed.items[i]!;

      assert.equal(got.id, expected.id, `item[${i}].id`);
      assert.deepEqual(got.tags, expected.tags, `item[${i}].tags`);
      assert.deepEqual(got.metadata, expected.metadata, `item[${i}].metadata`);
      assert.equal(got.createdAt, expected.created_at, `item[${i}].createdAt`);
      assert.equal(got.accessCount, expected.access_count, `item[${i}].accessCount`);
      assert.equal(got.lastAccessedAt, expected.last_accessed_at, `item[${i}].lastAccessedAt`);
      assert.equal(decoder.decode(got.value), expected.value, `item[${i}].value`);

      // The seed-derived hypervector must match a freshly-generated one from the
      // same UTF-8 seed bytes — this is the canonical determinism contract that
      // both core-ts and core-rs implement.
      const expectedHv = randomHv(new TextEncoder().encode(expected.seed), fixture.dimension);
      assert.equal(got.key.length, expectedHv.length, `item[${i}].key.length`);
      for (let bit = 0; bit < got.key.length; bit++) {
        assert.equal(got.key[bit], expectedHv[bit], `item[${i}].key[${bit}]`);
      }
    }
  });

  it("rejects a corrupted fixture (mutation detected by BLAKE3)", () => {
    const tampered = new Uint8Array(fixtureBytes);
    // Flip a byte deep inside the hv_block — the first ~100 bytes are header
    // material, so mutating offset 200 is guaranteed to land in a data block.
    tampered[200] = (tampered[200]! ^ 0xff) & 0xff;
    assert.throws(() => readKmf(tampered), /KMF: BLAKE3 mismatch/);
  });
});
