import { strict as assert } from "node:assert";
import { describe, it } from "node:test";

import { Smritidb } from "./store.js";
import { similarity } from "./hypervector.js";
import {
  CoactivationTracker,
  flagColdItems,
  pullCloser,
} from "./consolidate.js";
import { encodeString } from "./encode.js";

describe("CoactivationTracker", () => {
  it("counts pair co-activations across a window", () => {
    const t = new CoactivationTracker(10);
    t.record(["a", "b", "c"]);
    t.record(["a", "b"]);
    t.record(["b", "c"]);
    const pairs = t.pairsAtOrAbove(2);
    const ab = pairs.find((p) => p.a === "a" && p.b === "b");
    assert.equal(ab?.count, 2);
  });

  it("decrements as the window rolls", () => {
    const t = new CoactivationTracker(2);
    t.record(["a", "b"]);
    t.record(["a", "b"]);
    t.record(["x", "y"]);
    t.record(["x", "y"]);
    const pairs = t.pairsAtOrAbove(1);
    assert.equal(pairs.find((p) => p.a === "a" && p.b === "b"), undefined);
    assert.ok(pairs.find((p) => p.a === "x" && p.b === "y"));
  });
});

describe("pullCloser", () => {
  it("increases similarity by ~maxSimDelta deterministically", () => {
    const D = 4096;
    const a = encodeString("subject", D);
    const b = encodeString("predicate", D);
    const before = similarity(a, b);
    const result = pullCloser(a, b, 0.05, 1);
    const after = similarity(result.a, result.b);
    const lift = after - before;
    assert.ok(lift > 0.03 && lift <= 0.06, `expected lift 0.03..0.06, got ${lift}`);
    // Determinism check
    const result2 = pullCloser(a, b, 0.05, 1);
    assert.deepEqual(result.a, result2.a);
    assert.deepEqual(result.b, result2.b);
  });
});

describe("flagColdItems", () => {
  it("flags items that are old and rarely accessed", () => {
    const now = 1_000_000_000_000;
    const day = 24 * 60 * 60 * 1000;
    const items = [
      { id: "hot", accessCount: 50, lastAccessedAt: now },
      { id: "cold-rare", accessCount: 1, lastAccessedAt: now - 60 * day },
      { id: "cold-old-but-popular", accessCount: 50, lastAccessedAt: now - 60 * day },
      { id: "recent-rare", accessCount: 1, lastAccessedAt: now - 5 * day },
    ];
    const cold = flagColdItems(items, { coldDays: 30, coldMinAccess: 3 }, now);
    assert.deepEqual(cold, ["cold-rare"]);
  });
});

describe("Smritidb.consolidate", () => {
  it("pulls co-activated keys closer", () => {
    const store = new Smritidb({
      dimension: 4096,
      consolidation: { pullThreshold: 2, maxSimDelta: 0.05, windowSize: 100 },
    });
    store.put("subject", "subject", { id: "subject" });
    store.put("predicate", "predicate", { id: "predicate" });
    store.put("object", "object", { id: "object" });

    const subjectKey = store.get("subject").key;
    const predicateKey = store.get("predicate").key;
    const before = similarity(subjectKey, predicateKey);

    // Force three batches that recall both items together.
    for (let i = 0; i < 3; i++) {
      store.recall("subject", { topK: 3, minSimilarity: 0 });
    }

    const report = store.consolidate();
    assert.ok(report.pairsPulled >= 1, "expected at least one pair pulled");

    const after = similarity(
      store.get("subject").key,
      store.get("predicate").key,
    );
    assert.ok(after > before, `expected similarity to increase, before=${before} after=${after}`);
  });

  it("flags cold items based on access patterns", () => {
    const store = new Smritidb({
      dimension: 4096,
      consolidation: { coldDays: 1, coldMinAccess: 2, pullThreshold: 999 },
    });
    const hot = store.put("hot", "hot");
    const cold = store.put("cold", "cold");
    // Simulate the hot item being accessed plenty.
    for (let i = 0; i < 5; i++) {
      store.recall("hot", { topK: 1, minSimilarity: 0.9 });
    }
    // Manually age the cold item far past the cold threshold.
    cold.lastAccessedAt = Date.now() - 30 * 24 * 60 * 60 * 1000;
    const report = store.consolidate();
    assert.equal(report.coldItemsFlagged, 1);
    assert.equal(store.get(cold.id).cold, true);
    assert.equal(store.get(hot.id).cold, false);
  });

  it("is deterministic on identical state", () => {
    const make = () => {
      const s = new Smritidb({
        dimension: 4096,
        consolidation: { pullThreshold: 2, maxSimDelta: 0.05, windowSize: 100 },
      });
      s.put("a", "a", { id: "a" });
      s.put("b", "b", { id: "b" });
      for (let i = 0; i < 3; i++) s.recall("a", { topK: 2, minSimilarity: 0 });
      return s;
    };
    const s1 = make();
    const s2 = make();
    s1.consolidate();
    s2.consolidate();
    assert.deepEqual(s1.get("a").key, s2.get("a").key);
    assert.deepEqual(s1.get("b").key, s2.get("b").key);
  });
});
