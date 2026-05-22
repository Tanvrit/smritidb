# Persistence Architecture

Smritidb separates the in-memory store (`Store`) from durable storage via
a small **adapter trait** in `core-rs`. Every binding (Python, Kotlin,
Go, .NET, Dart) reaches durable storage through the same trait, so all
bindings see the same schema and the same on-disk wire format.

## The trait

`packages/core-rs/src/persist/mod.rs` defines the
`PersistenceAdapter` trait:

```rust
pub trait PersistenceAdapter: Send {
    fn load_snapshot(&mut self) -> Result<Option<Vec<u8>>, PersistenceError>;
    fn save_snapshot(&mut self, bytes: &[u8]) -> Result<(), PersistenceError>;
}
```

The adapter is the only thing that touches durable storage. The bytes it
reads and writes are always a **KMF snapshot** — the wire format defined
in SPEC §8. There is no per-item I/O path; persistence is whole-store
and snapshot-shaped.

## Adapters

| Adapter         | Language(s)               | What it does                                                                 |
|-----------------|---------------------------|------------------------------------------------------------------------------|
| `MemoryAdapter` | Rust, TS, Kotlin, Python  | Stores the snapshot in RAM. Useful for tests and ephemeral workloads.        |
| File adapter    | Rust (`FsAdapter`), TS (`fsAdapter`) | Writes the KMF snapshot to a single file path. Atomic via rename. |
| SQLite adapter  | Rust (`SqliteAdapter`), TS (`sqliteAdapter`) | Stores the snapshot in a `snapshots` table BLOB. Bundled libsqlite3 on Rust. |
| IndexedDB       | TS only (`indexedDbAdapter`) | Browser-side: stores the snapshot in an IndexedDB key/value store.       |

On Rust, the SQLite adapter is gated behind the `persist-sqlite` feature
(on by default) so `wasm32-unknown-unknown` builds can opt out — `rusqlite`
does not compile to that target. The IndexedDB adapter is TS-only because
it is the canonical browser persistence story for JS bundles built from
`core-ts`.

## Cross-binding SQLite schema

Every non-TypeScript binding stores SQLite snapshots via the same Rust
`SqliteAdapter`. There is one schema, defined once, and the Python,
Kotlin, C, Go, Dart, and .NET bindings all reach it through the Rust
core. Schema drift between bindings is structurally impossible.

The Rust schema:

```sql
CREATE TABLE IF NOT EXISTS snapshots (
    id         INTEGER PRIMARY KEY,
    blob       BLOB NOT NULL,
    written_at INTEGER NOT NULL
);
```

The TypeScript reference adapter
(`packages/core-ts/src/adapters/sqlite.ts`) reimplements the same shape
in pure TS using `better-sqlite3`. The two adapters are not yet
bit-identical in column naming — TS uses
`(key TEXT PRIMARY KEY, value BLOB, updated_at INTEGER)` historically —
this drift is tracked as Phase D item P4-1 in
`examples/polyglot-interop/KNOWN-ISSUES.md`. The polyglot demo handles
this today by having Step 4 read the **Rust-written schema** directly,
since Python (via PyO3 → Rust) is the writer.

## Browser persistence

For JS-target consumers of `core-ts`, the IndexedDB adapter is the path
forward. The same KMF snapshot bytes that a Rust SQLite row holds are
serialized into an IndexedDB key/value entry; the underlying KMF stream
is identical.

For Kotlin/JS and Kotlin/Wasm, the KMP package currently exposes
**memory-only** persistence — `openSqlite` and `openFile` throw
`UnsupportedOperationException` on those targets because `rusqlite` does
not compile to `wasm32-unknown-unknown`. The Phase D plan is to wire
the IndexedDB adapter and an OPFS-sqlite-wasm adapter through to the
KMP web targets.

## Phase D — current status

Phase D is the persistence-convergence track. The objective is:

1. **TS / Rust SQLite schema convergence** — TS reference adapter
   adopts the Rust schema verbatim so any binding can write a file any
   other binding can open.
2. **Browser adapters for the KMP wasm targets** — IndexedDB and
   OPFS-sqlite-wasm reachable from Kotlin/JS + Kotlin/Wasm.
3. **Snapshot streaming** — incremental KMF append for large stores,
   rather than full-snapshot rewrites.

## No third-party storage SDK

Smritidb deliberately ships zero third-party storage SDKs. We use:

- `rusqlite` (bundled libsqlite3) on Rust.
- `better-sqlite3` on the TS reference.
- The browser's native IndexedDB API on web targets.

There is no Firebase adapter, no S3 adapter, no Postgres adapter, no
"smritidb-cloud" service. The KMF wire format is small and portable
enough that an operator can drop it into whatever storage they want
with a thin adapter; that is the contract. We do not ship those
adapters because each one would introduce a third-party transitive
dependency surface that compromises the bit-identity story.

## Implementation pointers

- Rust: `packages/core-rs/src/persist/`
  ([`mod.rs`](../../packages/core-rs/src/persist/mod.rs),
  [`memory.rs`](../../packages/core-rs/src/persist/memory.rs),
  [`fs.rs`](../../packages/core-rs/src/persist/fs.rs),
  [`sqlite.rs`](../../packages/core-rs/src/persist/sqlite.rs)).
- TS: `packages/core-ts/src/adapters/`
  ([`index.ts`](../../packages/core-ts/src/adapters/index.ts),
  `memory.ts`, `fs.ts`, `sqlite.ts`, `indexeddb.ts`).
- Python: re-exports `PersistentStore` from `_native` (PyO3 → Rust core).
- Kotlin: `com.tanvrit.smritidb.PersistentStore` — `expect`/`actual`
  per target; JVM goes via UniFFI, Apple/Android Native via Cinterop,
  JS/Wasm via wasm-bindgen.
