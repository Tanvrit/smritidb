# Polyglot interop demo — known issues

## P4-1: TypeScript SQLite adapter schema drift — RESOLVED (Phase D)

**Status:** RESOLVED. The TypeScript adapter at
`packages/core-ts/src/adapters/sqlite.ts` was migrated to the canonical
Rust schema. Step 4 of this demo now goes through `sqliteAdapter()`
end-to-end (see `step4_ts_read.ts`).

**Original problem:** The two SQLite adapters used different schemas for the
snapshot table:

| Adapter                  | Schema (before)                                                    |
|--------------------------|--------------------------------------------------------------------|
| Rust (canonical)         | `id INTEGER PRIMARY KEY, blob BLOB NOT NULL, written_at INTEGER`   |
| TypeScript (legacy)      | `key TEXT PRIMARY KEY, value BLOB NOT NULL, updated_at INTEGER`    |

The KMF blob itself was already bit-identical between the two stacks — only
the container schema had drifted.

**Resolution (Phase D):**

1. The Rust schema is now the canonical layout shared by both adapters:
   ```sql
   CREATE TABLE smritidb_snapshot (
       id INTEGER PRIMARY KEY,
       blob BLOB NOT NULL,
       written_at INTEGER NOT NULL
   );
   CREATE TABLE smritidb_wal (
       seq INTEGER PRIMARY KEY AUTOINCREMENT,
       blob BLOB NOT NULL,
       written_at INTEGER NOT NULL
   );
   ```
2. The TS adapter now exposes a `SqliteStorageAdapter` interface with
   `appendWal()`, `readWal()`, `truncateWal()`, and `close()` in addition to
   the base `StorageAdapter` triple — matching the Rust `PersistenceAdapter`
   trait.
3. Legacy databases (TS-only deployments written by the Phase-1 adapter) are
   detected and migrated automatically on first open. The one-shot migration
   reads the most-recent legacy row, upserts it into `smritidb_snapshot`,
   drops the old `smritidb` table, and logs a single warning. See
   `migrateLegacySchema()` exported from
   `packages/core-ts/src/adapters/sqlite.ts`.
4. A cross-binary compatibility test
   (`packages/core-ts/src/adapters/sqlite.test.ts`) opens the fixture this
   demo writes (`/tmp/smritidb-polyglot.db`) through the TS adapter and
   asserts the KMF item count. It runs whenever the fixture is present.

**Where things landed:**

- TS adapter: `packages/core-ts/src/adapters/sqlite.ts`
- TS tests:   `packages/core-ts/src/adapters/sqlite.test.ts` (77 passing)
- Demo step 4: `examples/polyglot-interop/step4_ts_read.ts`
- Rust core: `packages/core-rs/src/persist/sqlite.rs` (unchanged)
