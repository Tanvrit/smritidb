import { strict as assert } from "node:assert";
import { describe, it } from "node:test";

import { similarity } from "./hypervector.js";
import { encodeBagOfWords, encodeCharNgrams, encodeWordNgrams } from "./text.js";

const D = 8192;

describe("encodeBagOfWords", () => {
  it("is deterministic", () => {
    const a = encodeBagOfWords("the cat sat on the mat", D);
    const b = encodeBagOfWords("the cat sat on the mat", D);
    assert.deepEqual(a, b);
  });

  it("shared-word inputs land above the random baseline", () => {
    const a = encodeBagOfWords("the cat sat on the mat", D);
    const b = encodeBagOfWords("cat sat", D);
    const c = encodeBagOfWords("zebra danced", D);
    const s_ab = similarity(a, b);
    const s_ac = similarity(a, c);
    assert.ok(s_ab > 0.55, `expected shared-word sim > 0.55, got ${s_ab}`);
    assert.ok(s_ab > s_ac, `shared-word should rank above unrelated`);
  });

  it("ignores stop-words by default", () => {
    // Default minWordLength=3 drops "is" and "a".
    const a = encodeBagOfWords("cat", D);
    const b = encodeBagOfWords("a cat is", D);
    assert.equal(similarity(a, b), 1);
  });

  it("can be made case-sensitive", () => {
    const a = encodeBagOfWords("Cat", D, { lowercase: false });
    const b = encodeBagOfWords("cat", D, { lowercase: false });
    assert.notEqual(similarity(a, b), 1);
  });
});

describe("encodeWordNgrams", () => {
  it("n=1 equals bag-of-words", () => {
    const a = encodeWordNgrams("the cat sat", D, 1);
    const b = encodeBagOfWords("the cat sat", D);
    assert.deepEqual(a, b);
  });

  it("preserves order for n>=2", () => {
    const ab = encodeWordNgrams("alpha beta", D, 2);
    const ba = encodeWordNgrams("beta alpha", D, 2);
    // Same words, different order → similarity below 1 (and noticeably below
    // the bag-of-words case which would be ~1).
    assert.notEqual(similarity(ab, ba), 1);
  });

  it("falls back to bag-of-words on short inputs", () => {
    const a = encodeWordNgrams("cat", D, 3);
    const b = encodeBagOfWords("cat", D);
    assert.deepEqual(a, b);
  });
});

describe("encodeCharNgrams", () => {
  it("is deterministic", () => {
    const a = encodeCharNgrams("hello world", D);
    const b = encodeCharNgrams("hello world", D);
    assert.deepEqual(a, b);
  });

  it("typo-tolerant: 1-char edit stays well above random", () => {
    const a = encodeCharNgrams("photograph", D, 3);
    const b = encodeCharNgrams("phtograph", D, 3); // missing 'o'
    const s = similarity(a, b);
    assert.ok(s > 0.7, `expected typo-tolerant sim > 0.7, got ${s}`);
  });

  it("substring matches above the random baseline", () => {
    const a = encodeCharNgrams("hyperdimensional computing", D, 3);
    const b = encodeCharNgrams("hyperdim", D, 3);
    const c = encodeCharNgrams("vector database", D, 3);
    const s_ab = similarity(a, b);
    const s_ac = similarity(a, c);
    assert.ok(s_ab > s_ac, `substring match should rank above unrelated text`);
  });
});
