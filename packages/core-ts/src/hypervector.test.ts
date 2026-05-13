import { strict as assert } from "node:assert";
import { describe, it } from "node:test";

import { bind, bundle, permute, randomHv, similarity, unbind } from "./hypervector.js";
import { encodeString } from "./encode.js";

const D = 8192;

describe("hypervector primitives", () => {
  it("similarity(a, a) === 1", () => {
    const a = randomHv(new TextEncoder().encode("hello"), D);
    assert.equal(similarity(a, a), 1);
  });

  it("similarity(random, random) ~ 0.5", () => {
    const a = randomHv(new TextEncoder().encode("a"), D);
    const b = randomHv(new TextEncoder().encode("b"), D);
    const s = similarity(a, b);
    assert.ok(s > 0.45 && s < 0.55, `expected ~0.5, got ${s}`);
  });

  it("bind is self-inverse: unbind(bind(a, b), b) === a", () => {
    const a = randomHv(new TextEncoder().encode("a"), D);
    const b = randomHv(new TextEncoder().encode("b"), D);
    const round = unbind(bind(a, b), b);
    assert.equal(similarity(a, round), 1);
  });

  it("bundle preserves similarity to each component", () => {
    const xs = [
      randomHv(new TextEncoder().encode("x1"), D),
      randomHv(new TextEncoder().encode("x2"), D),
      randomHv(new TextEncoder().encode("x3"), D),
    ];
    const bundled = bundle(xs);
    for (const x of xs) {
      const s = similarity(bundled, x);
      assert.ok(s > 0.65, `expected bundle similarity > 0.65 to component, got ${s}`);
    }
  });

  it("permute is invertible by inverse rotation", () => {
    const a = randomHv(new TextEncoder().encode("p"), D);
    const round = permute(permute(a, 7), -7);
    assert.equal(similarity(a, round), 1);
  });

  it("randomHv is deterministic for the same seed", () => {
    const seed = new TextEncoder().encode("seed");
    const a = randomHv(seed, D);
    const b = randomHv(seed, D);
    assert.equal(similarity(a, b), 1);
  });
});

describe("compositional recall (role/filler)", () => {
  it("recovers fillers from a bundled record", () => {
    const dimension = D;
    const roles = {
      name: encodeString("role:name", dimension),
      age: encodeString("role:age", dimension),
      city: encodeString("role:city", dimension),
    };
    const fillers = {
      alice: encodeString("filler:alice", dimension),
      bob: encodeString("filler:bob", dimension),
      thirty: encodeString("filler:30", dimension),
      twentyfive: encodeString("filler:25", dimension),
      boston: encodeString("filler:boston", dimension),
      denver: encodeString("filler:denver", dimension),
    };

    const record = bundle([
      bind(roles.name, fillers.alice),
      bind(roles.age, fillers.thirty),
      bind(roles.city, fillers.boston),
    ]);

    const queryName = unbind(record, roles.name);
    const candidates = Object.entries(fillers).map(
      ([k, hv]) => [k, similarity(hv, queryName)] as const,
    );
    candidates.sort((a, b) => b[1] - a[1]);
    assert.equal(candidates[0]![0], "alice", `expected alice on top, got ${candidates[0]![0]}`);
  });
});
