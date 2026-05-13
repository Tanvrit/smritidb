import { strict as assert } from "node:assert";
import { describe, it } from "node:test";

import { Kanerva } from "./store.js";

describe("Kanerva store", () => {
  it("rejects dimension < 1024", () => {
    assert.throws(() => new Kanerva({ dimension: 512 }), /dimension must be >= 1024/);
  });

  it("put + recall round-trips a string", () => {
    const store = new Kanerva({ dimension: 8192 });
    store.put("the cat sat on the mat", "the cat sat on the mat");
    const hits = store.recall("the cat sat on the mat", { topK: 1, minSimilarity: 0.9 });
    assert.equal(hits.length, 1);
    assert.equal(new TextDecoder().decode(hits[0]!.item.value), "the cat sat on the mat");
    assert.equal(hits[0]!.similarity, 1);
  });

  it("recalls multiple items ranked by similarity", () => {
    const store = new Kanerva({ dimension: 8192 });
    store.put("alpha", "alpha");
    store.put("beta", "beta");
    store.put("gamma", "gamma");
    const hits = store.recall("alpha", { topK: 3, minSimilarity: 0 });
    assert.equal(hits.length, 3);
    assert.equal(new TextDecoder().decode(hits[0]!.item.value), "alpha");
    assert.equal(hits[0]!.similarity, 1);
  });

  it("respects topK", () => {
    const store = new Kanerva({ dimension: 8192 });
    for (let i = 0; i < 25; i++) store.put(`item-${i}`, `value-${i}`);
    const hits = store.recall("item-7", { topK: 3, minSimilarity: 0 });
    assert.equal(hits.length, 3);
  });

  it("filter is applied before ranking", () => {
    const store = new Kanerva({ dimension: 8192 });
    store.put("alpha", "alpha", { tags: ["greek"] });
    store.put("beta", "beta", { tags: ["greek"] });
    store.put("one", "one", { tags: ["english"] });
    const hits = store.recall("alpha", {
      topK: 5,
      minSimilarity: 0,
      filter: (item) => item.tags.includes("english"),
    });
    assert.equal(hits.length, 1);
    assert.equal(new TextDecoder().decode(hits[0]!.item.value), "one");
  });

  it("accessCount increments on recall hit", () => {
    const store = new Kanerva({ dimension: 8192 });
    const item = store.put("alpha", "alpha");
    assert.equal(item.accessCount, 0);
    store.recall("alpha", { topK: 1, minSimilarity: 0.9 });
    store.recall("alpha", { topK: 1, minSimilarity: 0.9 });
    assert.equal(store.get(item.id).accessCount, 2);
  });

  it("delete removes the item", () => {
    const store = new Kanerva({ dimension: 8192 });
    const item = store.put("alpha", "alpha");
    assert.equal(store.delete(item.id), true);
    assert.equal(store.size(), 0);
    assert.throws(() => store.get(item.id), /NotFound/);
  });

  it("upsert by id preserves createdAt", async () => {
    const store = new Kanerva({ dimension: 8192 });
    const first = store.put("alpha", "alpha", { id: "fixed-id-1" });
    await new Promise((r) => setTimeout(r, 5));
    const second = store.put("alpha-prime", "alpha-prime", { id: "fixed-id-1" });
    assert.equal(second.id, "fixed-id-1");
    assert.equal(second.createdAt, first.createdAt);
    assert.equal(store.size(), 1);
  });

  it("rejects values larger than the cap", () => {
    const store = new Kanerva({ dimension: 8192, valueCapBytes: 16 });
    assert.throws(
      () => store.put("alpha", "x".repeat(64)),
      /ValueTooLarge/,
    );
  });
});
