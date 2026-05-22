/* eslint-disable no-console */
// Agent-memory demo, modernised onto the SQLite-backed PersistentStore.
//
// What this file demonstrates, in three short scenes:
//
//   1. The classic in-session recall — seed a few facts, recall by partial cue.
//      (Same as the original demo; kept so the file still teaches the basics.)
//
//   2. Cross-run persistence — the agent remembers things across process
//      restarts. We run the same SQLite-backed agent twice from one script
//      and prove the second invocation finds memories written by the first.
//
//   3. Two agents, one file — a second `AgentMemory` instance opens the same
//      .db file, writes its own observations, and the original agent recalls
//      them after a reload. This is the same property the polyglot-interop
//      demo proves across languages (`smritidb_snapshot` is the canonical
//      cross-binding schema), shown here cross-process within Node.
//
// Inspect the resulting database with:
//
//     sqlite3 /tmp/agent-memory.db ".schema"
//     sqlite3 /tmp/agent-memory.db "SELECT length(blob) FROM smritidb_snapshot;"
//
// Run: pnpm demo  (or npm start)

import { rmSync, existsSync } from "node:fs";
import { AgentMemory } from "./src/AgentMemory.js";
import { sqliteAdapter } from "@tanvrit/smritidb";

const DB_PATH = process.env.SMRITIDB_AGENT_MEMORY_PATH ?? "/tmp/agent-memory.db";
const DIMENSION = 8192;

function header(s: string) {
  console.log(`\n\x1b[1m${s}\x1b[0m`);
  console.log("─".repeat(s.length));
}

function printRecall(memory: AgentMemory, query: string) {
  console.log(`\nquery: ${query}`);
  const entries = memory.recall(query, { topK: 4, minSimilarity: 0 });
  if (entries.length === 0) {
    console.log("  (no matches above floor)");
    return;
  }
  for (const e of entries) {
    console.log(`  ${e.score.toFixed(2)}  [${e.tags.join(", ") || "—"}]  ${e.text}`);
  }
}

// Start each run from a clean slate so the demo's "before/after" is honest.
// In a real deployment you'd keep the file around — that's the whole point.
for (const suffix of ["", "-journal", "-wal", "-shm"]) {
  const p = DB_PATH + suffix;
  if (existsSync(p)) rmSync(p);
}

// ---------------------------------------------------------------------------
// Scene 1 — In-session recall (this is the original demo content).
// ---------------------------------------------------------------------------
header("1. In-session recall");

{
  const memory = await AgentMemory.open({
    dimension: DIMENSION,
    topK: 4,
    minSimilarity: 0,
    adapter: sqliteAdapter(DB_PATH),
  });

  await memory.remember("The user prefers brevity in summaries — no trailing recap.", ["style"]);
  await memory.remember("The user runs a Next.js + Cloudflare Pages stack for marketing sites.", ["stack"]);
  await memory.remember("The user is the founder of Tanvrit Private Limited.", ["bio"]);
  await memory.remember("The user lives in India and works from a single Mac mini cluster.", ["bio"]);
  await memory.remember("Always cite Sanskrit-rooted terminology when naming things.", ["style"]);
  await memory.remember("When asked for a code review, look for premature abstractions first.", ["style"]);
  await memory.remember(
    "The user shipped Smritidb in May 2026 — an open associative-memory standard.",
    ["bio", "smritidb"],
  );

  printRecall(memory, "what does the user like in writing style?");
  printRecall(memory, "what is Smritidb?");
  printRecall(memory, "what tech stack do they use?");

  console.log(`\n(persisted ${memory.size()} items to ${DB_PATH})`);
}

// ---------------------------------------------------------------------------
// Scene 2 — Cross-run persistence: open a *new* AgentMemory on the same file,
// no re-seeding. Everything below works because the SQLite snapshot survives.
// ---------------------------------------------------------------------------
header("2. Cross-run persistence (same file, fresh process-local state)");

{
  const memory = await AgentMemory.open({
    dimension: DIMENSION,
    topK: 3,
    minSimilarity: 0,
    adapter: sqliteAdapter(DB_PATH),
  });
  console.log(`\nreloaded ${memory.size()} items from disk — no re-seeding above this line.`);
  printRecall(memory, "tell me about the user's company");
  printRecall(memory, "style preferences");
}

// ---------------------------------------------------------------------------
// Scene 3 — Two agents, one file. A second logical agent ("the agent's
// background reflector") appends its own observations to the shared store;
// the primary agent picks them up on the next open.
//
// This works because `smritidb_snapshot` is the canonical schema — the same
// adapter contract every language binding uses (see examples/polyglot-interop/).
// ---------------------------------------------------------------------------
header("3. Two agents, one SQLite file");

{
  const reflector = await AgentMemory.open({
    dimension: DIMENSION,
    topK: 3,
    minSimilarity: 0,
    adapter: sqliteAdapter(DB_PATH),
  });
  console.log(`\n[reflector] opened, sees ${reflector.size()} prior memories.`);
  await reflector.remember(
    "Background reflection: the user tends to ask code-review questions on Fridays.",
    ["reflection", "pattern"],
  );
  await reflector.remember(
    "Background reflection: when the user says 'ship it', they mean 'merge after one more pass'.",
    ["reflection", "vocab"],
  );
  console.log(`[reflector] wrote 2 new memories; store now has ${reflector.size()} items.`);
}

{
  const primary = await AgentMemory.open({
    dimension: DIMENSION,
    topK: 3,
    minSimilarity: 0,
    adapter: sqliteAdapter(DB_PATH),
  });
  console.log(`\n[primary] reopened, sees ${primary.size()} memories (including the reflector's).`);
  printRecall(primary, "what does 'ship it' mean to the user?");
  printRecall(primary, "when does the user usually ask for reviews?");
}

// ---------------------------------------------------------------------------
// Final crib for the operator. The same file is readable from any Smritidb
// language binding — see `python_companion.py` in this directory for a Python
// reader of the very file we just wrote.
// ---------------------------------------------------------------------------
console.log("\n──");
console.log(`SQLite file:  ${DB_PATH}`);
console.log("Inspect:      sqlite3 " + DB_PATH + " '.schema'");
console.log("Python read:  python3 examples/agent-memory/python_companion.py");
