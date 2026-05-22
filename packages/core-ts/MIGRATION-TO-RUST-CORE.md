# Migration Plan: TypeScript Core → Rust Core

## Status as of SHA `17334f8`

The Rust core (`packages/core-rs`) now provides a complete implementation of
the SPEC.md surface:

- `persist::PersistenceAdapter` trait + `MemoryAdapter`, `FsAdapter`, and
  `SqliteAdapter` (under `packages/core-rs/src/persist/`).
- A full `Store` (`packages/core-rs/src/store.rs`) covering `put`, `recall`,
  `delete`, `upsert`, and the consolidation pipeline.
- KMF I/O (`packages/core-rs/src/kmf.rs`) that is **byte-identical** to the TS
  reference. The contract is enforced by the shared fixture at
  `tests/conformance/kmf_fixture.bin`, which is emitted from TypeScript and
  parsed bit-for-bit by both implementations.
- A wasm build target (`packages/core-rs/pkg/`, `packages/core-rs/pkg-node/`).
- UniFFI/NAPI bindings under `packages/smritidb-ffi/`, `packages/smritidb-py/`,
  `packages/smritidb-kmp/`.

The TypeScript core (`packages/core-ts`) remains the **reference
implementation** for now. Both implementations are bit-exact via the
conformance corpus (`tests/conformance/golden.json`) and the KMF cross-impl
fixture. Every new algorithm, encoder, or KMF revision lands first in
`core-ts`, gets golden coverage, and only then ports to `core-rs`.

## Roadmap

### Phase X.1 — Browser parity (wasm cutover)

Goal: a single algorithm implementation (Rust) running in browsers via wasm,
with no behavioural drift.

Concrete moves:

- Publish the existing `pkg/` build as `@smritidb/core-wasm` (or fold it under
  the umbrella package).
- Replace the in-process algorithm modules in `core-ts` with thin wrappers
  that delegate to `core-wasm`:
  - `src/hypervector.ts` → calls `core-wasm` for `randomHv`, `bind`, `bundle`,
    `permute`, `similarity`.
  - `src/encode.ts`, `src/text.ts` → call `core-wasm` for the text encoders.
  - `src/consolidate.ts` → calls `core-wasm` for `pullCloser`, `flagColdItems`,
    and `CoactivationTracker`.
  - `src/kmf.ts` → calls `core-wasm` for `readKmf` / `writeKmf`.
- `src/store.ts` either (a) wraps the wasm `Store` directly, or (b) stays in
  TS but operates over wasm-produced primitives. (a) is the long-term target;
  (b) is a safe intermediate step.
- Adapters stay TS-side:
  - `src/adapters/memory.ts` — unchanged.
  - `src/adapters/fs.ts` — unchanged. (`core-rs::FsAdapter` exists, but is not
    reachable from a browser; the TS FS adapter targets Node.)
  - `src/adapters/indexeddb.ts` — **stays TS-only**. Browsers cannot expose
    `rusqlite` over wasm without an OPFS-wrapped sqlite-wasm runtime, and
    IndexedDB has no Rust analogue. This is the one piece of `core-ts` that
    will outlive the migration unless we eventually ship an OPFS-backed
    `core-rs` adapter.

Exit criterion: `pnpm test` in `packages/core-ts` still passes against the
wasm-backed implementation, including the full conformance corpus and the KMF
cross-impl fixture. No SHA in `tests/conformance/golden.json` changes.

### Phase X.2 — Node parity (NAPI cutover)

Goal: kill the algorithmic duplication on the Node side, freeing us from
maintaining two implementations of every consolidation kernel.

Concrete moves:

- Add a NAPI binding to `core-rs` (a new `packages/smritidb-napi/` crate,
  separate from the wasm and UniFFI targets). NAPI is preferred over the
  UniFFI Kotlin path for Node — UniFFI's Node story is overkill for a
  same-process binding.
- On Node, `core-ts` selects the NAPI implementation at import time; on the
  browser it selects the wasm implementation. A small runtime switch lives in
  `src/index.ts`.
- `src/adapters/sqlite.ts` (which currently expects a `better-sqlite3`-shaped
  peer dependency) becomes a thin wrapper over `core-rs::SqliteAdapter`. The
  externally-injected `SqliteDatabase` contract is retained for backwards
  compatibility, but the recommended path will be to let `core-rs` open and
  manage the database file directly via rusqlite.
- `src/adapters/fs.ts` migrates the same way: still callable, but the
  recommended path goes through `core-rs::FsAdapter`.

Exit criterion: same as X.1 plus the SQLite adapter test runs against the
Rust-backed adapter, and `better-sqlite3` is removed from the documented
peer-dependency list.

### Phase X.3 — Single-source-of-truth

Goal: the TS package becomes a thin facade. New algorithm work happens in
Rust only.

Concrete moves:

- `core-ts` retains its public API surface (`Smritidb`, `snapshot`/`restore`,
  the encoders, the KMF helpers, the adapter contracts), but every
  implementation file becomes a delegation shim.
- The conformance corpus remains the contract. Both `core-ts` and `core-rs`
  continue to ship the same test suite against `tests/conformance/`.
- The TypeScript-only artefacts that survive are:
  - `src/adapters/indexeddb.ts` (browser-only, no Rust path).
  - The TS surface API itself (type definitions, error classes, the
    `Smritidb` constructor signature).
  - The fixture emitter at `tests/conformance/emit_kmf_fixture.mjs`, which
    keeps the corpus reproducible from the canonical reference.

Exit criterion: any algorithm change in `core-rs` automatically propagates to
the published `@tanvrit/smritidb` package on the next release; no parallel TS
patch is required.

## What does NOT change

- **SPEC.md and the conformance corpus are the legal authorities.** Both
  implementations must satisfy them byte-for-byte. Any byte-level divergence
  is a release blocker — not a wart to be patched per-implementation.
- **The TS public API is preserved across the migration.** User code does not
  need changes. Existing imports (`import { Smritidb } from
  "@tanvrit/smritidb"`) keep working through every phase.
- **KMF format stability.** The on-disk layout described in SPEC.md §8 does
  not move. The fixture at `tests/conformance/kmf_fixture.bin` is a frozen
  artefact for the lifetime of `spec_version 0.1.0`.
- **The browser IndexedDB adapter stays TypeScript.** It is the one piece of
  algorithmic-adjacent code that has no Rust analogue and is not on the path
  to consolidation.

## File-by-file disposition (target end state)

| Path                                    | Disposition                                                 |
| --------------------------------------- | ----------------------------------------------------------- |
| `src/index.ts`                          | Stays — runtime selector for wasm vs NAPI backend.          |
| `src/hypervector.ts`                    | Becomes a delegation shim over `core-rs`.                   |
| `src/encode.ts`                         | Becomes a delegation shim over `core-rs`.                   |
| `src/text.ts`                           | Becomes a delegation shim over `core-rs`.                   |
| `src/consolidate.ts`                    | Becomes a delegation shim over `core-rs`.                   |
| `src/kmf.ts`                            | Becomes a delegation shim over `core-rs`.                   |
| `src/store.ts`                          | Becomes a delegation shim over `core-rs::Store`.            |
| `src/snapshot.ts`                       | Stays — owns the public `snapshot`/`restore` API surface.   |
| `src/errors.ts`                         | Stays — TS-flavoured error classes for the public surface.  |
| `src/uuid.ts`                           | Stays or moves to `core-rs` (low-stakes; either works).     |
| `src/adapters/memory.ts`                | Stays for browser; Node uses `core-rs::MemoryAdapter`.      |
| `src/adapters/fs.ts`                    | Wrapper over `core-rs::FsAdapter` on Node.                  |
| `src/adapters/sqlite.ts`                | Wrapper over `core-rs::SqliteAdapter` on Node.              |
| `src/adapters/indexeddb.ts`             | **Stays TS-only.** Browser-only, no Rust analogue.          |
| `src/adapters/index.ts`                 | Stays — public adapter export surface.                      |
| `src/*.test.ts`                         | Stays. Same tests, run against the new backend.             |
| `tests/conformance/golden.json`         | Stays. Frozen contract.                                     |
| `tests/conformance/kmf_fixture.bin`     | Stays. Frozen contract.                                     |
| `tests/conformance/emit_kmf_fixture.mjs`| Stays. Reproducibility tool.                                |
