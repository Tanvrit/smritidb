# semantic-notebook

A small Smritidb walkthrough split across two files:

- **`index.ts`** — a one-screen in-memory tour: fuzzy semantic recall,
  compositional bind/unbind, and holographic degradation under cue noise.
- **`persistent.ts`** — the same store backed by SQLite, exercising
  cross-run persistence, the recall pipeline (top-k with similarity bars),
  and a SIGINT-safe shutdown path.

The SQLite-backed file uses the canonical Smritidb schema, so the same
`/tmp/semantic-notebook.db` is readable from any other language binding
(see `examples/polyglot-interop/`).

## Install and run

This example links `@tanvrit/smritidb` via `workspace:*`, so install with
pnpm (which understands the workspace protocol):

```bash
pnpm install                  # from repo root
cd examples/semantic-notebook

# Modernised SQLite-backed persistent demo (default `start`):
pnpm start                    # or: npm start

# In-memory walkthrough (no on-disk state):
pnpm demo                     # or: npm run demo
```

## What `persistent.ts` demonstrates

1. **Open-or-create.** On first run it seeds ten sentences and persists.
   On subsequent runs it reopens the file and reports the existing count —
   no re-seeding. Every run also appends a session marker, so successive
   invocations visibly grow the same store.
2. **Recall pipeline.** Five partial-cue queries, top-3 hits each, with a
   24-character similarity bar so the ranking is visible at a glance. The
   top match is highlighted with a green arrow.
3. **Ctrl-C safety.** A SIGINT/SIGTERM handler persists any in-flight
   additions through `persistStore()` and closes the SQLite connection
   cleanly before exiting (with code 130 on SIGINT). Try interrupting the
   script mid-run with `Ctrl-C`, then re-running it — the live notes that
   made it to disk before the interrupt are still there.

## What `index.ts` demonstrates

1. **Fuzzy semantic recall** — store a handful of sentences, query by
   partial phrases, get back the nearest stored sentence ranked by similarity.
2. **Compositional bind/unbind** — bundle three role-filler pairs into one
   hypervector, then unbind any role to recover its filler against a
   candidate set.
3. **Holographic degradation** — recall@1 over a 200-item corpus with cue
   corruption from 0% to 40%, still hitting 100% recall@1 at this scale.

## Inspecting the SQLite file

```bash
sqlite3 /tmp/semantic-notebook.db ".schema"
sqlite3 /tmp/semantic-notebook.db "SELECT length(blob), datetime(written_at/1000, 'unixepoch') FROM smritidb_snapshot;"
```

Override the file path with `SMRITIDB_NOTEBOOK_PATH=/some/other.db npm start`.

## What changed in this release

`persistent.ts` previously used the FS adapter and wrote a self-contained
`.kmf` file. The modernised version targets the SQLite-backed
`PersistentStore` — same on-the-wire KMF snapshot inside the blob column,
but stored in a SQLite container that's transactional, journal-able with
WAL mode, and bit-compatible with the Rust core. The earlier in-memory
walkthrough in `index.ts` is unchanged apart from a footer tweak.

## License

Apache-2.0.
