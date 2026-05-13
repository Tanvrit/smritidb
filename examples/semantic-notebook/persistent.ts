/* eslint-disable no-console */
// Persistence walkthrough using the FS adapter.
// Demonstrates: open -> populate -> persist -> close -> reopen -> recall.
//
// Run: pnpm demo:persistent

import {
  openPersistentStore,
  fsAdapter,
  persistStore,
  bundle,
  encodeString,
  type Hypervector,
} from "@tanvrit/smritidb";
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

function bagOfWords(text: string, dim: number): Hypervector {
  const words = text
    .toLowerCase()
    .replace(/[^a-z0-9 ]+/g, " ")
    .split(/\s+/)
    .filter((w) => w.length > 2);
  if (words.length === 0) return encodeString(text, dim);
  return bundle(words.map((w) => encodeString(`word:${w}`, dim)));
}

const dir = mkdtempSync(join(tmpdir(), "smritidb-persistent-"));
const path = join(dir, "notebook.kmf");

try {
  const D = 4096;

  // ---- session 1: write
  {
    const store = await openPersistentStore({ dimension: D, adapter: fsAdapter(path) });
    for (const sentence of [
      "the cat sat on the mat",
      "a bird in the hand is worth two in the bush",
      "actions speak louder than words",
    ]) {
      store.put(bagOfWords(sentence, D), sentence);
    }
    await persistStore(store, fsAdapter(path));
    console.log(`[session 1] wrote ${store.size()} items to ${path}`);
  }

  // ---- session 2: cold start, restore from disk
  {
    const store = await openPersistentStore({ dimension: D, adapter: fsAdapter(path) });
    console.log(`[session 2] restored ${store.size()} items`);

    const hits = store.recall(bagOfWords("cat on mat", D), { topK: 1, minSimilarity: 0 });
    if (hits.length > 0) {
      const matched = new TextDecoder().decode(hits[0]!.item.value);
      console.log(`  query "cat on mat" -> "${matched}" (sim=${hits[0]!.similarity.toFixed(3)})`);
    }
  }
} finally {
  rmSync(dir, { recursive: true, force: true });
}
