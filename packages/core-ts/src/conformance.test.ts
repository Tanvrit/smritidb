// Cross-binding conformance suite. Every Smritidb implementation runs this
// corpus (or its equivalent) as part of its own test suite.
//
// The golden file lives at tests/conformance/golden.json.

import { strict as assert } from "node:assert";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";
import { describe, it } from "node:test";

import {
  bind,
  bundle,
  encodeBagOfWords,
  encodeCharNgrams,
  encodeString,
  randomHv,
  similarity,
} from "./index.js";

const here = dirname(fileURLToPath(import.meta.url));
const goldenPath = resolve(here, "../../../tests/conformance/golden.json");

interface Golden {
  spec_version: string;
  random_hv: { seed_utf8: string; dim: number; sha256: string }[];
  encode_string: { input: string; dim: number; sha256: string }[];
  similarity_pairs: { a_seed: string; b_seed: string; dim: number; expected: number }[];
  bind_round_trip: { a_seed: string; b_seed: string; dim: number; expected_similarity_to_a: number }[];
  bundle: { seeds: string[]; dim: number; sha256: string }[];
  text_bag_of_words: { text: string; dim: number; sha256: string }[];
  text_char_ngrams: { text: string; dim: number; n: number; sha256: string }[];
}

const golden: Golden = JSON.parse(readFileSync(goldenPath, "utf8"));

const sha256 = (b: Uint8Array): string => createHash("sha256").update(b).digest("hex");

describe("conformance: spec_version", () => {
  it("matches the golden file", async () => {
    const { SPEC_VERSION } = await import("./index.js");
    assert.equal(SPEC_VERSION, golden.spec_version);
  });
});

describe("conformance: randomHv", () => {
  for (const r of golden.random_hv) {
    it(`randomHv("${r.seed_utf8}", ${r.dim}) hashes to ${r.sha256.slice(0, 8)}…`, () => {
      const hv = randomHv(new TextEncoder().encode(r.seed_utf8), r.dim);
      assert.equal(sha256(hv), r.sha256);
    });
  }
});

describe("conformance: encodeString", () => {
  for (const r of golden.encode_string) {
    it(`encodeString("${r.input}", ${r.dim}) hashes to ${r.sha256.slice(0, 8)}…`, () => {
      const hv = encodeString(r.input, r.dim);
      assert.equal(sha256(hv), r.sha256);
    });
  }
});

describe("conformance: similarity_pairs", () => {
  for (const r of golden.similarity_pairs) {
    it(`sim(randomHv("${r.a_seed}"), randomHv("${r.b_seed}")) ≈ ${r.expected}`, () => {
      const a = randomHv(new TextEncoder().encode(r.a_seed), r.dim);
      const b = randomHv(new TextEncoder().encode(r.b_seed), r.dim);
      const s = similarity(a, b);
      assert.ok(Math.abs(s - r.expected) < 1e-9, `expected ${r.expected}, got ${s}`);
    });
  }
});

describe("conformance: bind round-trip", () => {
  for (const r of golden.bind_round_trip) {
    it(`bind(bind(a, b), b) === a for (${r.a_seed}, ${r.b_seed})`, () => {
      const a = encodeString(r.a_seed, r.dim);
      const b = encodeString(r.b_seed, r.dim);
      const round = bind(bind(a, b), b);
      assert.equal(similarity(round, a), r.expected_similarity_to_a);
    });
  }
});

describe("conformance: bundle", () => {
  for (const r of golden.bundle) {
    it(`bundle([${r.seeds.join(", ")}], ${r.dim}) hashes to ${r.sha256.slice(0, 8)}…`, () => {
      const hvs = r.seeds.map((s) => randomHv(new TextEncoder().encode(s), r.dim));
      assert.equal(sha256(bundle(hvs)), r.sha256);
    });
  }
});

describe("conformance: encodeBagOfWords", () => {
  for (const r of golden.text_bag_of_words) {
    it(`encodeBagOfWords("${r.text}", ${r.dim}) hashes to ${r.sha256.slice(0, 8)}…`, () => {
      assert.equal(sha256(encodeBagOfWords(r.text, r.dim)), r.sha256);
    });
  }
});

describe("conformance: encodeCharNgrams", () => {
  for (const r of golden.text_char_ngrams) {
    it(`encodeCharNgrams("${r.text}", ${r.dim}, n=${r.n}) hashes to ${r.sha256.slice(0, 8)}…`, () => {
      assert.equal(sha256(encodeCharNgrams(r.text, r.dim, r.n)), r.sha256);
    });
  }
});
