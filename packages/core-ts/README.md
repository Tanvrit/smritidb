# @tanvrit/smritidb (core-ts)

TypeScript reference implementation of [Smritidb](https://github.com/Tanvrit/smritidb) — the open biology-inspired associative memory layer.

This package is both the reference (the bit-exact contract every other binding satisfies) and the published Node + browser SDK. The Rust core (`packages/core-rs`) reproduces its primitive output byte-for-byte; the conformance corpus and KMF fixture are emitted from this package.

## What it is

- Pure-TypeScript HDC primitives: `randomHv`, `bind`, `bundle`, `permute`, `similarity`, `encodeString`, `encodeEmbedding`.
- `Smritidb` in-memory store with `put` / `recall` / `consolidate`.
- `openPersistentStore({ adapter })` with `MemoryAdapter` and `SqliteAdapter` implementations.
- Pure-TypeScript KMF serializer and deserializer (used by the polyglot interop demo to validate Rust-emitted snapshots without ever calling into wasm).

## Install

```bash
npm install @tanvrit/smritidb
# or
pnpm add @tanvrit/smritidb
```

Works on Node 20+ and modern browsers. The SQLite adapter is Node-only (requires `better-sqlite3`); the browser story is the Kotlin/JS-backed `IndexedDbAdapter` in `smritidb-kmp` (see [`docs/guides/persistence.md`](../../docs/guides/persistence.md)).

## Hello, world

```ts
import { Smritidb } from "@tanvrit/smritidb";

const store = new Smritidb({ dimension: 10_000, backend: "memory" });

await store.put("the cat sat on the mat", { tags: ["sentence"] });
await store.put("a bird in the hand",     { tags: ["sentence"] });

const hits = await store.recall("cat on mat", { topK: 5 });
//   [{ item: { value: "the cat sat on the mat", ... }, similarity: 0.91 }, ...]
for (const h of hits) console.log(h.item.value, "->", h.similarity.toFixed(4));
```

SQLite-backed persistent store:

```ts
import { openPersistentStore, SqliteAdapter } from "@tanvrit/smritidb";

const adapter = new SqliteAdapter("/tmp/notes.db");
const store   = await openPersistentStore({ adapter, dimension: 10_000 });

await store.put("the cat sat on the mat", { tags: [] });
await store.persist();
await store.close();
```

The same `/tmp/notes.db` file is openable from Python, Rust, and Kotlin via the polyglot interop demo (`examples/polyglot-interop/`).

## Docs

- API reference (typedoc): [`docs/api/typescript/`](../../docs/api/typescript/index.html)
- Persistence architecture: [`docs/guides/persistence.md`](../../docs/guides/persistence.md)
- Cross-language interop: [`docs/guides/cross-language-interop.md`](../../docs/guides/cross-language-interop.md)
- Migration plan to the Rust core: [`MIGRATION-TO-RUST-CORE.md`](MIGRATION-TO-RUST-CORE.md)

## Tests

**73 tests passing**, including the cross-impl KMF parity test that verifies byte-identity against the Rust-emitted `kmf_fixture.bin`. Run:

```bash
pnpm install     # from repo root, once
pnpm -F @tanvrit/smritidb test
```

## License

Apache-2.0 — with the patent grant. See the repo root [`LICENSE`](../../LICENSE) and [`NOTICE`](../../NOTICE).
