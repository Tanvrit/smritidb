# Smritidb polyglot interop demo

An end-to-end demonstration that a Smritidb-on-SQLite file is **bit-exactly
portable across four language stacks** — Python, Rust, Kotlin/JVM, and
TypeScript — without any conversion step in the middle.

The same file (`/tmp/smritidb-polyglot.db`) is written in one language and
opened in the three others. Each reader independently verifies:

1. The SQLite container can be opened.
2. The item count matches what the writer recorded (100 items).
3. The top-1 recall for the cue `concept_50` returns the byte-identical
   value blob `item value 50` that Python wrote.

Step 4 (TypeScript) is the strictest check: it reads the SQLite row as a
raw BLOB and parses the **KMF wire format** end-to-end in pure TypeScript,
without ever calling into the Rust core. If that succeeds, the KMF
serializer in Rust and the deserializer in TypeScript agree on every byte.

## Flow

| Step | Language     | Path through the stack                                        |
|------|--------------|---------------------------------------------------------------|
| 1    | Python       | `smritidb.PersistentStore.open_sqlite(...)` (PyO3 -> Rust)    |
| 2    | Rust         | `smritidb_core::open_persistent_store(SqliteAdapter, ...)`    |
| 3    | Kotlin/JVM   | `com.tanvrit.smritidb.PersistentStore.openSqlite(...)` (UniFFI -> JNA -> Rust) |
| 4    | TypeScript   | `better-sqlite3` -> raw BLOB -> `readKmf(...)` (pure TS)      |

Every step prints `PASS: ...` to stdout and exits with code 0; any error
prints `FAIL: ...` to stderr and exits non-zero. `run-all.sh` chains them
with `set -e`, so the whole script exits at the first failure.

## Running the demo

```bash
cd examples/polyglot-interop
./run-all.sh
```

The driver assumes the following prerequisites have already been built
(they are checked in or produced by the regular workspace build):

- **Python:** `packages/smritidb-py/python/smritidb/_native.abi3.so`
  (built by `maturin develop --release` from `packages/smritidb-py/`).
- **Rust:** the workspace builds on demand via `cargo run --release` in
  `step2_rust_read/`.
- **Kotlin:** the JAR at
  `packages/smritidb-kmp/build/libs/smritidb-kmp-jvm-0.1.0.jar`
  (built by `gradle :jvmJar` in `packages/smritidb-kmp/`). It bundles
  the UniFFI bindings and the JNA-loadable `libuniffi_smritidb.dylib`
  under the `darwin-aarch64/` resource path.
- **Node:** `npm install` once, here, to pick up `better-sqlite3` and
  the local `@tanvrit/smritidb` reference build at `../../packages/core-ts/`.

## What this proves

- **KMF byte-identity** across Rust and TypeScript (already covered by
  `tests/conformance/kmf_fixture.bin`, but here we exercise the *full
  Rust core's* KMF output, not a fixture).
- **SQLite schema identity** between the Rust persistence adapter and
  the Python / Kotlin bindings: all three open a file at the same path
  via the same `SqliteAdapter` code on the Rust side, only the language
  binding differs.
- **Hypervector dimension agreement**: Step 4 cross-checks
  `kmf_item.key.length == kmf_header.dimension`, proving the KMF header
  and the per-item key blocks agree on the wire.
- **Recall semantics agreement**: Steps 2 and 3 both run
  `recall("concept_50", ...)` and assert the top-1 similarity is
  `> 0.99` — proving the Rust core's `encode_string` -> Hamming
  similarity pipeline produces the same answer no matter which language
  binding drives it.

## Known issue: TS SQLite adapter schema drift

The TypeScript reference SQLite adapter in
`packages/core-ts/src/adapters/sqlite.ts` uses a different schema
(`key TEXT PRIMARY KEY, value BLOB, updated_at INTEGER`) than the Rust
adapter in `packages/core-rs/src/persist/sqlite.rs`
(`id INTEGER PRIMARY KEY, blob BLOB, written_at INTEGER`). This is
tracked in `KNOWN-ISSUES.md` (item P4-1). For this demo, Step 4 reads
the **Rust schema directly**, since Python (via PyO3 -> Rust) is what
wrote the file. Converging the two adapters is a Phase D item.

## Files

- `step1_python_write.py` — the producer.
- `step2_rust_read/` — small Cargo binary crate.
- `step3_kotlin_read/` — small Gradle JVM project consuming the
  `smritidb-kmp-jvm` JAR via `flatDir`.
- `step4_ts_read.ts` — `tsx`-run Node script.
- `run-all.sh` — orchestrator.
- `KNOWN-ISSUES.md` — the schema-drift caveat above.
- `package.json` — `better-sqlite3` + local `@tanvrit/smritidb` for Step 4.
