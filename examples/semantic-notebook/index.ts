/* eslint-disable no-console */
// A Smritidb walkthrough that fits on one screen.
//
// 1. Put a few sentences in. The cue and the value are the same string here
//    to keep the example tight; in real apps the cue would be a description
//    or embedding, and the value would be the actual data.
//
// 2. Query by a partial phrase. Fuzzy content-addressing: the nearest stored
//    sentence is returned, with its similarity score.
//
// 3. Compositional bind/unbind: bundle three role-filler pairs into one
//    record, then unbind a role to recover its filler. This is how relational
//    data lives natively in Smritidb.
//
// Run: pnpm demo

import {
  Smritidb,
  bind,
  bundle,
  encodeString,
  similarity,
  unbind,
  type Hypervector,
} from "@tanvrit/smritidb";

const D = 10_000;

/**
 * A simple bag-of-words encoder: bundle a hypervector per word. This is what
 * makes partial cues ("cat on mat") match longer stored sentences ("the cat
 * sat on the mat"). The encoder lives in the demo on purpose — Smritidb
 * itself ships the math primitives; how text becomes a key is a *modelling*
 * choice your app makes.
 */
function encodeBagOfWords(text: string, dim: number): Hypervector {
  const words = text
    .toLowerCase()
    .replace(/[^a-z0-9 ]+/g, " ")
    .split(/\s+/)
    .filter((w) => w.length > 2);
  if (words.length === 0) return encodeString(text, dim);
  return bundle(words.map((w) => encodeString(`word:${w}`, dim)));
}

function header(s: string) {
  console.log(`\n[1m${s}[0m`);
  console.log("─".repeat(s.length));
}

// ---------------------------------------------------------------------------
// 1. Fuzzy semantic recall
header("1. Fuzzy semantic recall");

const store = new Smritidb({ dimension: D });

for (const sentence of [
  "the cat sat on the mat",
  "a bird in the hand is worth two in the bush",
  "the early bird catches the worm",
  "all that glitters is not gold",
  "a rolling stone gathers no moss",
  "actions speak louder than words",
  "fortune favours the bold",
]) {
  store.put(encodeBagOfWords(sentence, D), sentence);
}

for (const query of [
  "cat on mat",
  "bird in hand",
  "rolling stone",
  "speak louder than words",
]) {
  const cueHv = encodeBagOfWords(query, D);
  const hits = store.recall(cueHv, { topK: 3, minSimilarity: 0 });
  console.log(`\nquery: ${query}`);
  for (const h of hits) {
    const matched = new TextDecoder().decode(h.item.value);
    console.log(`  ${h.similarity.toFixed(3)}  ${matched}`);
  }
}

// ---------------------------------------------------------------------------
// 2. Compositional bind/unbind on a single bundled record
header("2. Compositional bind/unbind");

const role = (name: string) => encodeString(`role:${name}`, D);
const filler = (val: string) => encodeString(`filler:${val}`, D);

const candidates = ["alice", "bob", "carol", "thirty", "twentyfive", "boston", "denver"];

const record = bundle([
  bind(role("name"), filler("alice")),
  bind(role("age"), filler("thirty")),
  bind(role("city"), filler("boston")),
]);

for (const r of ["name", "age", "city"]) {
  const query = unbind(record, role(r));
  const ranked = candidates
    .map((c) => ({ c, s: similarity(filler(c), query) }))
    .sort((x, y) => y.s - x.s)
    .slice(0, 3);
  console.log(`\nrecord["${r}"] → ${ranked.map((x) => `${x.c}=${x.s.toFixed(3)}`).join("  ")}`);
}

// ---------------------------------------------------------------------------
// 3. Holographic degradation: corrupt the cue HV directly (not the string)
header("3. Holographic degradation — recall@1 vs cue bit-flip noise");

const N = 200;
const fresh = new Smritidb({ dimension: D });
const itemHvs: Hypervector[] = [];
for (let i = 0; i < N; i++) {
  const hv = encodeString(`item-${i}`, D);
  itemHvs.push(hv);
  fresh.put(hv, `item-${i}`);
}

function flipBits(hv: Hypervector, rate: number): Hypervector {
  const out = new Uint8Array(hv);
  for (let i = 0; i < out.length; i++) {
    if (Math.random() < rate) out[i] = out[i]! ^ 1;
  }
  return out;
}

for (const noise of [0, 0.1, 0.2, 0.3, 0.4]) {
  let correct = 0;
  const trials = 100;
  for (let i = 0; i < trials; i++) {
    const idx = Math.floor(Math.random() * N);
    const noisy = flipBits(itemHvs[idx]!, noise);
    const hits = fresh.recall(noisy, { topK: 1, minSimilarity: 0 });
    if (hits[0] && new TextDecoder().decode(hits[0].item.value) === `item-${idx}`) correct++;
  }
  console.log(`bit-flip rate=${noise.toFixed(2)}  recall@1=${(correct / trials).toFixed(2)}`);
}

console.log("\n✓ demo complete — see examples/semantic-notebook/persistent.ts for the FS adapter example.");
