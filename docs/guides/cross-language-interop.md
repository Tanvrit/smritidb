# Cross-Language Interop

Smritidb data is portable across language stacks at the byte level. A
SQLite file written by the Python binding can be opened by the Rust core,
by the Kotlin/JVM wrapper, and by the TypeScript reference — and each
reader independently agrees on every byte of state.

The polyglot demo at `examples/polyglot-interop/` exercises this in
end-to-end form.

## The demo

```text
+----------------+              writes              +---------------------+
| Python         | -------------------------------> | /tmp/smritidb-      |
| PyO3 -> Rust   |                                  |  polyglot.db        |
+----------------+                                  | (SQLite, KMF blobs) |
                                                    +----------+----------+
                                                               |
                                                               | opens & reads
                          +------------------+-----------------+--------------------+
                          v                  v                                      v
                +-------------------+   +----------------+                  +------------------+
                | Rust step2        |   | Kotlin/JVM     |                  | TypeScript       |
                | open_persistent_  |   | UniFFI -> JNA  |                  | better-sqlite3   |
                | store(Sqlite)     |   | -> Rust        |                  | + pure-TS KMF    |
                +-------------------+   +----------------+                  +------------------+
                          |                  |                                      |
                          v                  v                                      v
                  recall("concept_50") == top-1 similarity > 0.99,
                  value == "item value 50" (bit-identical bytes)
```

### Pipeline

1. **Step 1 — Python writes.** `smritidb.PersistentStore.open_sqlite(...)`
   creates `/tmp/smritidb-polyglot.db` and inserts 100 items keyed by
   `concept_0`..`concept_99` with values `item value 0`..`item value 99`.
2. **Step 2 — Rust reads.** A tiny Cargo bin opens the same path via
   `smritidb_core::open_persistent_store(SqliteAdapter, ...)`, asserts
   `size() == 100`, and runs `recall("concept_50", ...)` with the
   top-1 similarity expected to be `> 0.99` and value equal to
   `b"item value 50"`.
3. **Step 3 — Kotlin/JVM reads.** `com.tanvrit.smritidb.PersistentStore
   .openSqlite(...)`. Under the hood this is UniFFI-generated Kotlin
   loading `libsmritidb_ffi.dylib` via JNA, which calls back into the
   same Rust `SqliteAdapter`. Same recall, same assertion.
4. **Step 4 — TypeScript reads (strictest).** Uses `better-sqlite3`
   directly to fetch the raw BLOB from the snapshot row, then calls
   `readKmf(blob)` from `@tanvrit/smritidb` to deserialize the KMF wire
   format end-to-end **in pure TypeScript**, with no call into the Rust
   core. If this succeeds, the Rust serializer and the TS deserializer
   agree on every byte of KMF.

`run-all.sh` orchestrates the four steps under `set -e`; the demo passes
iff every step prints `PASS` and exits 0.

## Why this works

### KMF — the Smritidb Memory Format

KMF is the on-disk wire format defined in SPEC §8. It is the only
serialization format Smritidb uses. Snapshots go into the `snapshots`
table of the SQLite adapter as raw BLOBs.

KMF is JSON for the header + per-item metadata, with explicit
insertion-order preservation, and length-prefixed binary blocks for the
hypervector payloads. The Rust core uses `serde_json` with the
`preserve_order` feature to match the TypeScript reference's
insertion-order semantics inherited from JavaScript.

The output is **bit-exact** — `sha256(rust_kmf) == sha256(ts_kmf)` for
every snapshot. The conformance corpus pins this with a fixture
(`tests/conformance/kmf_fixture.bin`).

### Unified Rust persistence

Every non-TypeScript binding routes persistence calls into the **same**
`SqliteAdapter` in `packages/core-rs/src/persist/sqlite.rs`. Python
(via PyO3), Kotlin (via UniFFI -> JNA), C/Go/Dart/.NET (via the
`smritidb-c` ABI) all converge on the same Rust code path, so schema
drift between bindings is impossible.

TypeScript has its own SQLite adapter
(`packages/core-ts/src/adapters/sqlite.ts`) that reimplements the same
schema in pure TS. Converging the two schemas exactly is tracked as a
Phase D item; see `examples/polyglot-interop/KNOWN-ISSUES.md`.

### Hypervector determinism

`encode_string("concept_50", 10_000)` produces the same 10,000-bit hypervector
in every language (BLAKE3-seeded, deterministic). The recall pipeline
uses Hamming distance over packed bytes, which is also language-agnostic.
The conformance corpus's `random_hv`, `encode_string`, `bundle`, and
`bind_round_trip` entries lock all of this in via SHA-256 hashes of the
raw bit-bytes.

## Running the demo locally

```sh
cd examples/polyglot-interop
./run-all.sh
```

Prerequisites:

- Python: `cd packages/smritidb-py && maturin develop --release`.
- Rust: builds on demand.
- Kotlin: `cd packages/smritidb-kmp && gradle :jvmJar`.
- Node: `npm install` in `examples/polyglot-interop/`.

## What this proves

- **KMF byte-identity** across Rust and TypeScript.
- **SQLite schema identity** between every binding that goes through the
  Rust `SqliteAdapter`.
- **Hypervector dimension agreement** at the per-item and per-header
  level (Step 4 cross-checks
  `kmf_item.key.length == kmf_header.dimension`).
- **Recall semantics agreement**: Hamming-similarity ranking produces
  the same top-K answer regardless of which language drives the recall.

For the deeper "why" — the data model, the operations, the KMF wire
format — see [`SPEC.md`](../../SPEC.md).
