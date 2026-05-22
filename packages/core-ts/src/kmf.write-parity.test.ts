// KMF write byte-parity regression test.
//
// Locks down the TS writer against `tests/conformance/kmf_write_parity_fixture.bin`,
// a checked-in canonical KMF blob built from a deterministic 10-item / D=128
// workload. The fixture is emitted by `tests/conformance/emit_kmf_write_parity_fixture.mjs`.
//
// Why this exists: the patent's "byte-identical cross-implementation
// reproducibility" claim demands that any logically-equivalent snapshot
// serialises to the same bytes everywhere. The Rust side proves this via
// `packages/core-rs/tests/kmf_cross_impl.rs` — that test loads a TS-emitted
// fixture, parses it, and re-emits it via `write_kmf`, asserting byte
// identity. This test is the symmetric lock for the TS writer: change KMF
// serialisation (whitespace, key order, length-prefix width, …) and this
// test fails with a precise byte-offset diff.
//
// Regenerate the fixture (only if the wire format intentionally changes):
//
//   cd packages/core-ts && pnpm build
//   node tests/conformance/emit_kmf_write_parity_fixture.mjs
//
// then update `kmf_cross_impl.rs` if needed, run both test suites, and
// verify byte length + first-diverging-byte assertions still hold.

import { strict as assert } from "node:assert";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, it } from "node:test";

import { writeKmf, type KmfItem } from "./kmf.js";
import { randomHv } from "./hypervector.js";

const here = dirname(fileURLToPath(import.meta.url));
const fixturePath = resolve(here, "../../../tests/conformance/kmf_write_parity_fixture.bin");

// Workload parameters must match `tests/conformance/emit_kmf_write_parity_fixture.mjs`.
const DIM = 128;
const N = 10;
const CREATED_AT = 1_700_000_000_000;

function buildWorkload(): KmfItem[] {
  const enc = new TextEncoder();
  const items: KmfItem[] = [];
  for (let i = 0; i < N; i++) {
    const ii = i.toString().padStart(2, "0");
    items.push({
      id: `item_${ii}`,
      key: randomHv(enc.encode(`seed-${ii}`), DIM),
      value: enc.encode(`value-${ii}`),
      tags: ["bench", `g${i % 3}`],
      metadata: { rank: i, kind: "letter" },
      createdAt: CREATED_AT + i * 1_000,
      accessCount: i,
      lastAccessedAt: CREATED_AT + i * 1_000 + 500,
    });
  }
  return items;
}

describe("KMF write byte-parity", () => {
  it("produces bytes byte-identical to the canonical fixture", () => {
    const fresh = writeKmf({
      dimension: DIM,
      createdAt: CREATED_AT,
      items: buildWorkload(),
    });
    const golden = new Uint8Array(readFileSync(fixturePath));

    if (fresh.length !== golden.length) {
      throw new Error(
        `KMF length drift: fresh=${fresh.length} golden=${golden.length} ` +
          `(delta ${fresh.length - golden.length} bytes). ` +
          `Regenerate the fixture or fix the writer.`,
      );
    }

    // Locate the first diverging byte for a tractable failure mode.
    for (let i = 0; i < fresh.length; i++) {
      if (fresh[i] !== golden[i]) {
        const lo = Math.max(0, i - 8);
        const hi = Math.min(fresh.length, i + 8);
        throw new Error(
          `byte mismatch at offset ${i}: fresh=0x${fresh[i]!.toString(16).padStart(2, "0")} ` +
            `golden=0x${golden[i]!.toString(16).padStart(2, "0")}\n` +
            `context fresh:  [${Array.from(fresh.subarray(lo, hi))
              .map((b) => b.toString(16).padStart(2, "0"))
              .join(" ")}]\n` +
            `context golden: [${Array.from(golden.subarray(lo, hi))
              .map((b) => b.toString(16).padStart(2, "0"))
              .join(" ")}]`,
        );
      }
    }

    assert.equal(fresh.length, golden.length);
  });

  it("the canonical fixture has the expected length", () => {
    // Hard-coded so an accidental fixture rewrite shows up here too. If you
    // intentionally bumped the wire format, update both numbers and the
    // companion Rust test in `kmf_cross_impl.rs`.
    const golden = readFileSync(fixturePath);
    assert.equal(golden.length, 2268, "parity fixture length");
  });
});
