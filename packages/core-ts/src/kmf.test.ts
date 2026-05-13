import { strict as assert } from "node:assert";
import { describe, it } from "node:test";

import { Smritidb } from "./store.js";
import { snapshot, restore } from "./snapshot.js";
import { KMF_SPEC_VERSION, readKmf, writeKmf } from "./kmf.js";
import { similarity } from "./hypervector.js";
import { encodeString } from "./encode.js";

describe("KMF wire format", () => {
  it("starts with KMF magic and the spec version", () => {
    const bytes = writeKmf({ dimension: 1024, createdAt: 0, items: [] });
    assert.deepEqual([...bytes.subarray(0, 4)], [0x4b, 0x4d, 0x46, 0x00]);
    assert.equal(KMF_SPEC_VERSION, "0.1.0");
  });

  it("round-trips a populated store", () => {
    const store = new Smritidb({ dimension: 2048 });
    store.put("alpha", "alpha", { tags: ["greek"] });
    store.put("beta",  "beta",  { tags: ["greek"], metadata: { rank: 2 } });
    store.put("one",   "one",   { tags: ["english"] });

    const bytes = snapshot(store);
    const restored = restore(bytes);

    assert.equal(restored.size(), 3);

    const hits = restored.recall("alpha", { topK: 1, minSimilarity: 0.9 });
    assert.equal(hits.length, 1);
    assert.equal(new TextDecoder().decode(hits[0]!.item.value), "alpha");
    assert.equal(hits[0]!.similarity, 1);
  });

  it("preserves bit-exact hypervectors through pack/unpack", () => {
    const D = 4096;
    const store = new Smritidb({ dimension: D });
    const original = encodeString("test-bit-exact", D);
    store.put(original, "payload", { id: "fixed-id" });

    const bytes = snapshot(store);
    const restored = restore(bytes);
    const after = restored.get("fixed-id").key;

    assert.equal(after.length, D);
    assert.equal(similarity(original, after), 1);
  });

  it("rejects truncated files", () => {
    const store = new Smritidb({ dimension: 1024 });
    store.put("x", "x");
    const bytes = snapshot(store);
    assert.throws(() => readKmf(bytes.subarray(0, bytes.length - 1)), /trailer|invalid/i);
  });

  it("detects corruption via BLAKE3 mismatch", () => {
    const store = new Smritidb({ dimension: 1024 });
    store.put("alpha", "alpha");
    store.put("beta", "beta");
    const bytes = snapshot(store);
    // Flip a byte inside the hv_block region (after the magic/version/offset).
    const tampered = new Uint8Array(bytes);
    tampered[64] ^= 0xff;
    assert.throws(() => readKmf(tampered), /BLAKE3 mismatch/);
  });

  it("rejects unsupported spec versions", () => {
    const store = new Smritidb({ dimension: 1024 });
    store.put("alpha", "alpha");
    const bytes = snapshot(store);
    const tampered = new Uint8Array(bytes);
    // Overwrite version field with "9.0.0\0".
    tampered.set(new TextEncoder().encode("9.0.0\0"), 4);
    assert.throws(() => readKmf(tampered), /unsupported spec_version/);
  });

  it("preserves item metadata and tags", () => {
    const store = new Smritidb({ dimension: 1024 });
    store.put("alpha", "alpha", {
      id: "id-1",
      tags: ["x", "y"],
      metadata: { score: 0.97, kind: "test", flag: true, missing: null },
    });
    const bytes = snapshot(store);
    const restored = restore(bytes);
    const item = restored.get("id-1");
    assert.deepEqual(item.tags, ["x", "y"]);
    assert.equal(item.metadata.score, 0.97);
    assert.equal(item.metadata.kind, "test");
    assert.equal(item.metadata.flag, true);
    assert.equal(item.metadata.missing, null);
  });
});
