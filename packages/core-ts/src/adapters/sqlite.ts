import type { StorageAdapter } from "./index.js";

/**
 * SQLite adapter using `better-sqlite3` as a peer dependency.
 *
 *     npm install better-sqlite3
 *
 * The substrate is stored as a single BLOB row in a `smritidb_snapshot`
 * table. WAL journal mode is enabled for concurrent reads. The schema is
 * intentionally minimal — KMF is the on-the-wire contract; SQLite is just
 * the durable container.
 *
 * Multiple stores can coexist in one database file by passing different
 * `tableName` values.
 */
export function sqliteAdapter(
  pathOrDatabase: string | SqliteDatabase,
  options: SqliteAdapterOptions = {},
): StorageAdapter {
  const tableName = options.tableName ?? "smritidb_snapshot";
  const rowKey = options.rowKey ?? "default";

  let dbPromise: Promise<SqliteDatabase> | null = null;
  const getDb = async (): Promise<SqliteDatabase> => {
    if (dbPromise) return dbPromise;
    dbPromise = (async () => {
      const db =
        typeof pathOrDatabase === "string"
          ? await openDatabase(pathOrDatabase)
          : pathOrDatabase;
      db.pragma?.("journal_mode = WAL");
      db.exec(
        `CREATE TABLE IF NOT EXISTS "${tableName}" (
           key   TEXT PRIMARY KEY,
           value BLOB NOT NULL,
           updated_at INTEGER NOT NULL
         )`,
      );
      return db;
    })();
    return dbPromise;
  };

  return {
    kind: "sqlite",

    async read(): Promise<Uint8Array | null> {
      const db = await getDb();
      const row = db
        .prepare<[string], { value: Buffer | Uint8Array } | undefined>(
          `SELECT value FROM "${tableName}" WHERE key = ?`,
        )
        .get(rowKey);
      if (!row) return null;
      return row.value instanceof Uint8Array
        ? new Uint8Array(row.value.buffer, row.value.byteOffset, row.value.byteLength)
        : new Uint8Array(row.value);
    },

    async write(bytes: Uint8Array): Promise<void> {
      const db = await getDb();
      db.prepare<[string, Uint8Array, number]>(
        `INSERT INTO "${tableName}" (key, value, updated_at) VALUES (?, ?, ?)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at`,
      ).run(rowKey, bytes, Date.now());
    },

    async remove(): Promise<void> {
      const db = await getDb();
      db.prepare<[string]>(`DELETE FROM "${tableName}" WHERE key = ?`).run(rowKey);
    },
  };
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

// Minimal structural type covering the parts of better-sqlite3 we touch.
// Avoids declaring it as a hard dependency at type level.
export interface SqliteDatabase {
  pragma?: (statement: string) => unknown;
  exec(sql: string): unknown;
  prepare<P extends unknown[] = unknown[], R = unknown>(sql: string): {
    get(...params: P): R | undefined;
    run(...params: P): { changes: number; lastInsertRowid: number | bigint };
  };
}

export interface SqliteAdapterOptions {
  /** Table name to use. Default `smritidb_snapshot`. */
  readonly tableName?: string;
  /** Row key (lets multiple stores share one table). Default `default`. */
  readonly rowKey?: string;
}
