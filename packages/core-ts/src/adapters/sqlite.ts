import type { StorageAdapter } from "./index.js";

/**
 * SQLite adapter using `better-sqlite3` as a peer dependency.
 *
 *     npm install better-sqlite3
 *
 * Schema (created idempotently on first use):
 *
 * ```sql
 * CREATE TABLE IF NOT EXISTS smritidb_snapshot (
 *     id INTEGER PRIMARY KEY,
 *     blob BLOB NOT NULL,
 *     written_at INTEGER NOT NULL
 * );
 * CREATE TABLE IF NOT EXISTS smritidb_wal (
 *     seq INTEGER PRIMARY KEY AUTOINCREMENT,
 *     blob BLOB NOT NULL,
 *     written_at INTEGER NOT NULL
 * );
 * ```
 *
 * This schema is the canonical Smritidb persistence layout, shared bit-exactly
 * with the Rust core (`packages/core-rs/src/persist/sqlite.rs`). A SQLite file
 * written by either side opens transparently in the other.
 *
 * - `smritidb_snapshot` is single-row at `id = 1`. Writes use `INSERT … ON
 *   CONFLICT(id) DO UPDATE` so the row count stays at one.
 * - `smritidb_wal` is append-only with an autoincrement `seq`. Reads return
 *   blobs in ascending order. `truncateWal()` clears the table.
 * - `PRAGMA journal_mode=WAL` is enabled on open. Note this is SQLite's *own*
 *   write-ahead log — orthogonal to the Smritidb WAL stored in `smritidb_wal`.
 *
 * Backwards compat: a database opened with the legacy Phase-1 schema
 * (`smritidb(key TEXT, value BLOB, updated_at INTEGER)`) is migrated to the
 * canonical schema on first open. See `migrateLegacySchema()`.
 */
export function sqliteAdapter(
  pathOrDatabase: string | SqliteDatabase,
  _options: SqliteAdapterOptions = {},
): SqliteStorageAdapter {
  let dbPromise: Promise<SqliteDatabase> | null = null;

  const getDb = async (): Promise<SqliteDatabase> => {
    if (dbPromise) return dbPromise;
    dbPromise = (async () => {
      const db =
        typeof pathOrDatabase === "string"
          ? await openDatabase(pathOrDatabase)
          : pathOrDatabase;
      // WAL is best-effort: not all targets (e.g. :memory:) can enable it,
      // but the call is harmless.
      try {
        db.pragma?.("journal_mode = WAL");
      } catch {
        // ignore — best effort
      }
      db.exec(
        `CREATE TABLE IF NOT EXISTS smritidb_snapshot (
           id INTEGER PRIMARY KEY,
           blob BLOB NOT NULL,
           written_at INTEGER NOT NULL
         );
         CREATE TABLE IF NOT EXISTS smritidb_wal (
           seq INTEGER PRIMARY KEY AUTOINCREMENT,
           blob BLOB NOT NULL,
           written_at INTEGER NOT NULL
         );`,
      );
      migrateLegacySchema(db);
      return db;
    })();
    return dbPromise;
  };

  return {
    kind: "sqlite",

    async read(): Promise<Uint8Array | null> {
      const db = await getDb();
      const row = db
        .prepare(`SELECT blob FROM smritidb_snapshot WHERE id = ?`)
        .get(SNAPSHOT_ROW_ID) as { blob: Buffer | Uint8Array } | undefined;
      if (!row) return null;
      return toUint8Array(row.blob);
    },

    async write(bytes: Uint8Array): Promise<void> {
      const db = await getDb();
      db.prepare(
        `INSERT INTO smritidb_snapshot (id, blob, written_at) VALUES (?, ?, ?)
         ON CONFLICT(id) DO UPDATE SET blob = excluded.blob,
                                       written_at = excluded.written_at`,
      ).run(SNAPSHOT_ROW_ID, bytes, Date.now());
    },

    async remove(): Promise<void> {
      const db = await getDb();
      db.prepare(`DELETE FROM smritidb_snapshot WHERE id = ?`).run(SNAPSHOT_ROW_ID);
    },

    async appendWal(bytes: Uint8Array): Promise<void> {
      const db = await getDb();
      db.prepare(
        `INSERT INTO smritidb_wal (blob, written_at) VALUES (?, ?)`,
      ).run(bytes, Date.now());
    },

    async readWal(): Promise<Uint8Array[]> {
      const db = await getDb();
      const rows = db
        .prepare(`SELECT blob FROM smritidb_wal ORDER BY seq ASC`)
        .all() as Array<{ blob: Buffer | Uint8Array }>;
      return rows.map((r) => toUint8Array(r.blob));
    },

    async truncateWal(): Promise<void> {
      const db = await getDb();
      db.prepare(`DELETE FROM smritidb_wal`).run();
    },

    async close(): Promise<void> {
      if (!dbPromise) return;
      const db = await dbPromise;
      dbPromise = null;
      db.close?.();
    },
  };
}

/** Row id reserved for the single snapshot row. Mirrors `SNAPSHOT_ROW_ID` in Rust. */
const SNAPSHOT_ROW_ID = 1;

function toUint8Array(v: Buffer | Uint8Array): Uint8Array {
  return v instanceof Uint8Array
    ? new Uint8Array(v.buffer, v.byteOffset, v.byteLength)
    : new Uint8Array(v);
}

/**
 * Detect and migrate the legacy Phase-1 schema in-place.
 *
 * The legacy schema was:
 *
 * ```sql
 * CREATE TABLE smritidb (key TEXT PRIMARY KEY, value BLOB, updated_at INTEGER);
 * ```
 *
 * If a `smritidb` table is present and the canonical `smritidb_snapshot` row
 * for `id = 1` is empty, we:
 *
 *   1. Read the most recent legacy `value` (any rowKey — typically `"default"`).
 *   2. Upsert it into `smritidb_snapshot` at `id = 1`.
 *   3. Drop the legacy `smritidb` table.
 *
 * One-shot: after migration the legacy table is gone, so subsequent opens are
 * no-ops. Exported for testing and for callers who want to invoke migration
 * explicitly before opening a store.
 */
export function migrateLegacySchema(db: SqliteDatabase): boolean {
  // Does the legacy table exist?
  const legacy = db
    .prepare(
      `SELECT name FROM sqlite_master WHERE type = 'table' AND name = 'smritidb'`,
    )
    .get() as { name: string } | undefined;
  if (!legacy) return false;

  // Is the canonical row already populated? If so, the legacy table is stale.
  // We still drop it so we don't migrate twice, but we don't overwrite.
  const existing = db
    .prepare(`SELECT id FROM smritidb_snapshot WHERE id = ?`)
    .get(SNAPSHOT_ROW_ID) as { id: number } | undefined;

  if (!existing) {
    // Pick the most-recent legacy row (highest updated_at). The Phase-1 adapter
    // used `rowKey` to multiplex stores in one table; we only carry forward
    // one row, which matches the new single-snapshot semantics.
    const row = db
      .prepare(
        `SELECT value, updated_at FROM smritidb ORDER BY updated_at DESC LIMIT 1`,
      )
      .get() as { value: Buffer | Uint8Array; updated_at: number } | undefined;
    if (row) {
      db.prepare(
        `INSERT INTO smritidb_snapshot (id, blob, written_at) VALUES (?, ?, ?)
         ON CONFLICT(id) DO UPDATE SET blob = excluded.blob,
                                       written_at = excluded.written_at`,
      ).run(SNAPSHOT_ROW_ID, toUint8Array(row.value), row.updated_at);
    }
  }

  db.exec(`DROP TABLE smritidb`);
  // One-line warning so operators see the migration in their logs.
  // eslint-disable-next-line no-console
  console.warn(
    "[smritidb/sqlite] migrated legacy `smritidb` table to `smritidb_snapshot` (one-shot)",
  );
  return true;
}

async function openDatabase(path: string): Promise<SqliteDatabase> {
  // Dynamic import so users who don't need SQLite don't pay the cost.
  // better-sqlite3 is a peer dependency (optional). The dynamic specifier
  // hides it from tsc's static checker so installs without it still type-check.
  const specifier = "better-sqlite3";
  const mod = (await import(/* @vite-ignore */ specifier)) as {
    default: new (path: string) => SqliteDatabase;
  };
  return new mod.default(path);
}

/**
 * `StorageAdapter` extended with WAL and lifecycle methods. The base
 * `StorageAdapter` contract only requires `read`/`write`/`remove`; the SQLite
 * adapter exposes more so callers can drive the snapshot-and-WAL model
 * directly (matching the Rust `PersistenceAdapter` trait).
 */
export interface SqliteStorageAdapter extends StorageAdapter {
  /** Append a WAL entry. */
  appendWal(bytes: Uint8Array): Promise<void>;
  /** Read all WAL entries in seq order. */
  readWal(): Promise<Uint8Array[]>;
  /** Clear the WAL. */
  truncateWal(): Promise<void>;
  /** Close the underlying SQLite connection. Safe to call multiple times. */
  close(): Promise<void>;
}

/**
 * Minimal structural type covering the parts of better-sqlite3 we touch.
 * Avoids declaring it as a hard dependency at type level.
 *
 * The shape matches `better-sqlite3`'s Database / Statement APIs:
 *
 *   - `pragma(stmt)`         — execute a PRAGMA, returns the result.
 *   - `exec(sql)`            — run one or more statements with no params.
 *   - `prepare(sql)`         — compile, then `.run / .get / .all` it.
 *   - `close()`              — close the connection.
 */
export interface SqliteDatabase {
  pragma?: (statement: string) => unknown;
  exec(sql: string): unknown;
  prepare(sql: string): SqliteStatement;
  close?(): void;
}

export interface SqliteStatement {
  /** Run an INSERT/UPDATE/DELETE/DDL. */
  run(...params: unknown[]): { changes: number; lastInsertRowid: number | bigint };
  /** Fetch a single row. */
  get(...params: unknown[]): unknown;
  /** Fetch all matching rows. */
  all(...params: unknown[]): unknown[];
}

export interface SqliteAdapterOptions {
  // Reserved for future use. The legacy `tableName` / `rowKey` knobs from
  // Phase 1 were removed when the schema was unified with the Rust core; the
  // canonical layout is single-snapshot in `smritidb_snapshot`. Kept as an
  // empty interface so the call signature stays stable for downstream users.
}
