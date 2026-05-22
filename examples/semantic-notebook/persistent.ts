/* eslint-disable no-console */
// Persistent semantic notebook, backed by the SQLite-canonical PersistentStore.
//
// The TS reference SQLite adapter writes the same `smritidb_snapshot` /
// `smritidb_wal` schema the Rust core uses, so a notebook file is portable
// across every Smritidb language binding (see examples/polyglot-interop).
//
// What this script demonstrates, in three short scenes:
//
//   1. Open-or-create a persistent notebook at /tmp/semantic-notebook.db.
//      On first run, seed it with a few sentences; on subsequent runs, those
//      sentences are already there (cross-run survival, including across
//      Ctrl-C of the previous process).
//
//   2. Recall pipeline — run a few queries, highlight the top-k matches with
//      a similarity bar, so the ranking is visible at a glance.
//
//   3. Signal-safe persistence — install a SIGINT / SIGTERM handler that
//      persists any in-flight additions before exiting. This is the part that
//      makes "Ctrl-C and restart" honest.
//
// Inspect with:
//
//     sqlite3 /tmp/semantic-notebook.db ".schema"
//     sqlite3 /tmp/semantic-notebook.db "SELECT length(blob) FROM smritidb_snapshot;"
//
// Run: pnpm demo:persistent  (or npm start)

import { existsSync } from "node:fs";
import {
  openPersistentStore,
  persistStore,
  sqliteAdapter,
  encodeBagOfWords,
} from "@tanvrit/smritidb";

const D = 4096;
const PATH = process.env.SMRITIDB_NOTEBOOK_PATH ?? "/tmp/semantic-notebook.db";

const SEED_SENTENCES = [
  "the cat sat on the mat",
  "a bird in the hand is worth two in the bush",
  "the early bird catches the worm",
  "all that glitters is not gold",
  "a rolling stone gathers no moss",
  "actions speak louder than words",
  "fortune favours the bold",
  "absence makes the heart grow fonder",
  "a stitch in time saves nine",
  "good fences make good neighbours",
];

function header(s: string) {
  console.log(`\n\x1b[1m${s}\x1b[0m`);
  console.log("─".repeat(s.length));
}

function bar(sim: number, width = 24): string {
  const filled = Math.max(0, Math.min(width, Math.round(sim * width)));
  return "█".repeat(filled) + "·".repeat(width - filled);
}

// ---------------------------------------------------------------------------
// 1. Open-or-create.
// ---------------------------------------------------------------------------
const firstRun = !existsSync(PATH);
const adapter = sqliteAdapter(PATH);
const store = await openPersistentStore({ dimension: D, adapter });

header(firstRun ? "1. First run — seeding the notebook" : "1. Subsequent run — reusing notebook on disk");

if (firstRun) {
  for (const s of SEED_SENTENCES) {
    store.put(encodeBagOfWords(s, D), s);
  }
  await persistStore(store, adapter);
  console.log(`seeded ${store.size()} sentences and persisted to ${PATH}`);
} else {
  console.log(`reopened ${PATH} with ${store.size()} sentences already present`);
}

// Append a "session marker" each run so successive invocations are visibly
// growing the same store. Ctrl-C between scenes 2 and 3 still leaves these on
// disk because of the SIGINT handler installed below.
const sessionTag = `session @ ${new Date().toISOString()}`;
store.put(encodeBagOfWords(sessionTag, D), sessionTag, { tags: ["session-marker"] });
await persistStore(store, adapter);
console.log(`+ appended a session marker; store now holds ${store.size()} items`);

// ---------------------------------------------------------------------------
// 2. Recall pipeline.
// ---------------------------------------------------------------------------
header("2. Recall pipeline — top-3 with similarity bars");

const decoder = new TextDecoder();
const QUERIES = [
  "cat on mat",
  "bird in hand",
  "rolling stone",
  "speak louder than words",
  "fence and neighbours",
];

for (const q of QUERIES) {
  console.log(`\n  query: \x1b[1m${q}\x1b[0m`);
  const hits = store.recall(encodeBagOfWords(q, D), { topK: 3, minSimilarity: 0 });
  for (let i = 0; i < hits.length; i++) {
    const h = hits[i]!;
    const matched = decoder.decode(h.item.value);
    const marker = i === 0 ? "\x1b[32m▶\x1b[0m" : " ";
    console.log(`  ${marker} ${h.similarity.toFixed(3)}  ${bar(h.similarity)}  ${matched}`);
  }
}

// ---------------------------------------------------------------------------
// 3. Signal-safe persistence — survive Ctrl-C / kill.
// ---------------------------------------------------------------------------
header("3. Ctrl-C safety");

let persisting = false;
async function gracefulPersist(signal: string) {
  if (persisting) return;
  persisting = true;
  console.log(`\n  caught ${signal}; persisting store before exit...`);
  try {
    await persistStore(store, adapter);
    await adapter.close();
    console.log(`  done. ${store.size()} items safe on disk at ${PATH}.`);
  } catch (e) {
    console.error(`  persist on shutdown failed:`, e);
  }
  process.exit(signal === "SIGINT" ? 130 : 0);
}
process.on("SIGINT", () => void gracefulPersist("SIGINT"));
process.on("SIGTERM", () => void gracefulPersist("SIGTERM"));

// Demonstrate by writing a few "live" entries with a small async delay between
// them, then exiting normally. If you interrupt this script with Ctrl-C mid-
// loop, re-running it shows the partial state survived.
const LIVE_NOTES = [
  "live note: a watched pot never boils",
  "live note: too many cooks spoil the broth",
  "live note: birds of a feather flock together",
];
for (const note of LIVE_NOTES) {
  store.put(encodeBagOfWords(note, D), note, { tags: ["live"] });
  await persistStore(store, adapter);
  await new Promise((r) => setTimeout(r, 50));
  console.log(`  wrote + persisted: "${note}"`);
}

console.log(`\nfinal store size: ${store.size()} items`);
console.log(`(rerun this script — the count grows by one session marker + ${LIVE_NOTES.length} live notes each time.)`);

await adapter.close();
