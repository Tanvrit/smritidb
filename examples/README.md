# Smritidb examples

Runnable examples that exercise Smritidb at three different surface areas.
All three sit on top of the same canonical persistence layer
(`smritidb_snapshot` + `smritidb_wal` tables, KMF snapshot blob),
so files written by one example are readable from any other binding.

| Example                                  | What it shows                                                                                                                       |
|------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------|
| [`agent-memory/`](./agent-memory)        | A framework-agnostic long-term-memory adapter for LLM agents, plus three SQLite-backed scenes: in-session recall, cross-run persistence, and two agents sharing one file. Ships with a Python companion that reads the same file. |
| [`semantic-notebook/`](./semantic-notebook) | A short Smritidb tour: in-memory walkthrough (`index.ts`) plus a SQLite-backed persistent notebook (`persistent.ts`) with the recall pipeline visualised and Ctrl-C-safe persistence. |
| [`polyglot-interop/`](./polyglot-interop) | A 4-language end-to-end interop run: Python writes a SQLite file, Rust / Kotlin / TypeScript each read it back and verify byte-identity through the canonical schema. |

## Quick start

The two TS examples link the workspace package via `workspace:*`, so use pnpm
to set up dependencies (npm doesn't understand the workspace protocol):

```bash
pnpm install   # from repo root, once

cd examples/agent-memory      && pnpm start   # or: npm start
cd examples/semantic-notebook && pnpm start   # or: npm start
cd examples/polyglot-interop  && ./run-all.sh # see its README for prereqs
```

Once `pnpm install` has linked everything, `npm start` works too — it just
runs the script through the pre-installed symlinks.

## Shared persistence layer

All examples (and every Smritidb language binding) use the same SQLite
schema:

```sql
CREATE TABLE smritidb_snapshot (id INTEGER PRIMARY KEY, blob BLOB NOT NULL, written_at INTEGER NOT NULL);
CREATE TABLE smritidb_wal      (seq INTEGER PRIMARY KEY AUTOINCREMENT, blob BLOB NOT NULL, written_at INTEGER NOT NULL);
```

The single snapshot row at `id = 1` holds a KMF (Kanerva Memory Format)
snapshot byte string. The TS reference adapter and the Rust core write
identical bytes, which is what the `polyglot-interop/` demo exercises end
to end.

Inspect any example's file with:

```bash
sqlite3 /path/to/file.db ".schema"
sqlite3 /path/to/file.db "SELECT length(blob), datetime(written_at/1000, 'unixepoch') FROM smritidb_snapshot;"
```

## License

Apache-2.0.
