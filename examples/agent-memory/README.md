# agent-memory

A tiny framework-agnostic long-term-memory adapter for LLM agents, built on
`@tanvrit/smritidb`. ~150 lines of `AgentMemory.ts` plus a demo that exercises
three scenarios end-to-end on top of a real SQLite file.

Drop the adapter next to your orchestration layer (LangChain.js, LlamaIndex.ts,
Mastra, OpenAI Assistants, a custom loop, …):

```ts
import { openSqliteBackedMemory } from "./src/AgentMemory.js";

const memory = await openSqliteBackedMemory("/var/lib/myagent/memory.db");
await memory.remember("The user prefers brevity in summaries.", ["style"]);
const block = memory.recallAsPromptBlock("draft a release note");
// Prepend `block` to your prompt before calling the model.
```

`AgentMemory.recall()` returns plain `{ text, score, tags }` records — no
LLM vendor SDK is pulled in. You decide how to thread them into your prompt.

## What this demo shows

`demo.ts` runs three scenes against a single SQLite file
(`/tmp/agent-memory.db` by default, override with `SMRITIDB_AGENT_MEMORY_PATH`):

1. **In-session recall** — seed seven facts about the user, run three
   partial-cue queries, get ranked recall hits with similarity scores.
2. **Cross-run persistence** — open a *new* `AgentMemory` instance against
   the same file, with no re-seeding, and recall everything written by
   scene 1. The SQLite snapshot survives process exit.
3. **Two agents, one file** — a second "background reflector" agent opens
   the same file, writes its own observations, and the primary agent picks
   them up on the next open. This is the same property the
   `examples/polyglot-interop/` demo proves across languages, here shown
   cross-process inside Node.

## Install and run

This example links `@tanvrit/smritidb` via `workspace:*`, so install with
pnpm (which understands the workspace protocol):

```bash
pnpm install                  # from repo root, sets up symlinks for all examples
cd examples/agent-memory
pnpm start                    # or: npm start  (uses the pre-installed symlinks)
```

The `start` script is an alias for `tsx demo.ts`.

Expected output (abridged): three labelled scenes, each printing recall
hits in the form

```
0.66  [bio]  The user is the founder of Tanvrit Private Limited.
```

Scene 3 finishes with a pointer to `python_companion.py`.

## Inspecting the SQLite file

The adapter uses the canonical Smritidb schema, byte-compatible with the Rust
core and every language binding:

```bash
sqlite3 /tmp/agent-memory.db ".schema"
# CREATE TABLE smritidb_snapshot (id INTEGER PRIMARY KEY, blob BLOB NOT NULL, written_at INTEGER NOT NULL);
# CREATE TABLE smritidb_wal      (seq INTEGER PRIMARY KEY AUTOINCREMENT, blob BLOB NOT NULL, written_at INTEGER NOT NULL);

sqlite3 /tmp/agent-memory.db "SELECT length(blob), datetime(written_at/1000, 'unixepoch') FROM smritidb_snapshot;"
```

The single `smritidb_snapshot` row holds the KMF snapshot blob; the WAL table
is reserved for incremental updates (Phase 5 / SPEC §6) and is empty here.

## Cross-language interop: Python companion

After `npm start`, the same file is directly readable from Python via the
PyO3 bindings:

```bash
python3 examples/agent-memory/python_companion.py
```

This proves the demo's own SQLite file is portable across language bindings
without any conversion — Python and TypeScript meet at the canonical
`smritidb_snapshot` byte string. Requires the Python extension to be built
first:

```bash
cd packages/smritidb-py && maturin develop --release
```

## Why this isn't a vector-DB plugin

The three load-bearing Smritidb properties — fuzzy content-addressing,
holographic distribution, Hebbian consolidation — are exactly what agent
memory needs: partial-cue retrieval, graceful degradation under substrate
loss, and use-pattern-driven reshaping. Smritidb is purpose-built for this
shape of workload, not retrofitted to it.

See [the manifesto](https://smritidb.com/manifesto) for the long version.

## License

Apache-2.0.
