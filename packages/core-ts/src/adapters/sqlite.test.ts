import { strict as assert } from "node:assert";
import { existsSync } from "node:fs";
import { describe, it } from "node:test";

import {
  sqliteAdapter,
  migrateLegacySchema,
  type SqliteDatabase,
  type SqliteStatement,
} from "./sqlite.js";
import { Smritidb } from "../store.js";
import { openPersistentStore, persistStore } from "./index.js";
import { readKmf } from "../kmf.js";

/**
 * Minimal in-memory SQLite mock satisfying the `SqliteDatabase` contract.
 *
 * Only implements the handful of SQL shapes the adapter actually issues:
 *
 *   - DDL: `CREATE TABLE IF NOT EXISTS …` / `DROP TABLE …`
 *   - `SELECT name FROM sqlite_master …`               (migration probe)
 *   - `SELECT blob FROM smritidb_snapshot WHERE id = ?` (snapshot read)
 *   - `INSERT … ON CONFLICT … DO UPDATE`               (snapshot upsert)
 *   - `DELETE FROM smritidb_snapshot WHERE id = ?`     (snapshot delete)
 *   - `INSERT INTO smritidb_wal …`                     (wal append)
 *   - `SELECT blob FROM smritidb_wal ORDER BY seq ASC` (wal read)
 *   - `DELETE FROM smritidb_wal`                       (wal truncate)
 *   - Legacy: `SELECT value, updated_at FROM smritidb ORDER BY …`
 *
 * Plus enough hooks for the legacy-migration test to pre-populate the old
 * `smritidb(key, value, updated_at)` table.
 */
interface MockDb extends SqliteDatabase {
  /** Test-only: directly seed a legacy `smritidb` row. */
  __seedLegacy(key: string, value: Uint8Array, updatedAt: number): void;
  /** Test-only: peek at the current tables. */
  __tables(): string[];
}

interface SnapshotRow {
  id: number;
  blob: Uint8Array;
  written_at: number;
}
interface WalRow {
  seq: number;
  blob: Uint8Array;
  written_at: number;
}
interface LegacyRow {
  key: string;
  value: Uint8Array;
  updated_at: number;
}

function mockDb(): MockDb {
  let snapshot: SnapshotRow | undefined; // single-row
  const wal: WalRow[] = [];
  let nextSeq = 1;
  const legacy = new Map<string, LegacyRow>();
  const tables = new Set<string>();

  const exec = (sql: string): unknown => {
    // Split on `;` and process each non-empty statement.
    for (const raw of sql.split(";")) {
      const stmt = raw.trim();
      if (!stmt) continue;
      const create = stmt.match(/^CREATE TABLE IF NOT EXISTS\s+(\w+)/i);
      if (create) {
        tables.add(create[1]!);
        continue;
      }
      const drop = stmt.match(/^DROP TABLE\s+(\w+)/i);
      if (drop) {
        const name = drop[1]!;
        tables.delete(name);
        if (name === "smritidb") legacy.clear();
        continue;
      }
      throw new Error(`mockDb.exec: unsupported sql: ${stmt}`);
    }
    return undefined;
  };

  const prepare = (sql: string): SqliteStatement => {
    const trimmed = sql.replace(/\s+/g, " ").trim();

    // sqlite_master probe: legacy table detection.
    if (/^SELECT name FROM sqlite_master/i.test(trimmed)) {
      return {
        get: () => (tables.has("smritidb") ? { name: "smritidb" } : undefined),
        run: () => ({ changes: 0, lastInsertRowid: 0 }),
        all: () => (tables.has("smritidb") ? [{ name: "smritidb" }] : []),
      };
    }

    // Snapshot SELECT by id.
    if (/^SELECT blob FROM smritidb_snapshot WHERE id = \?/i.test(trimmed)) {
      return {
        get: (...params: unknown[]) =>
          snapshot && snapshot.id === params[0] ? { blob: snapshot.blob } : undefined,
        run: () => ({ changes: 0, lastInsertRowid: 0 }),
        all: () => (snapshot ? [{ blob: snapshot.blob }] : []),
      };
    }
    if (/^SELECT id FROM smritidb_snapshot WHERE id = \?/i.test(trimmed)) {
      return {
        get: (...params: unknown[]) =>
          snapshot && snapshot.id === params[0] ? { id: snapshot.id } : undefined,
        run: () => ({ changes: 0, lastInsertRowid: 0 }),
        all: () => (snapshot ? [{ id: snapshot.id }] : []),
      };
    }

    // Snapshot upsert.
    if (/^INSERT INTO smritidb_snapshot/i.test(trimmed)) {
      return {
        run: (...params: unknown[]) => {
          const [id, blob, written_at] = params as [number, Uint8Array, number];
          snapshot = { id, blob, written_at };
          return { changes: 1, lastInsertRowid: id };
        },
        get: () => undefined,
        all: () => [],
      };
    }

    // Snapshot delete.
    if (/^DELETE FROM smritidb_snapshot WHERE id = \?/i.test(trimmed)) {
      return {
        run: (...params: unknown[]) => {
          if (snapshot && snapshot.id === params[0]) {
            snapshot = undefined;
            return { changes: 1, lastInsertRowid: 0 };
          }
          return { changes: 0, lastInsertRowid: 0 };
        },
        get: () => undefined,
        all: () => [],
      };
    }

    // WAL append.
    if (/^INSERT INTO smritidb_wal/i.test(trimmed)) {
      return {
        run: (...params: unknown[]) => {
          const [blob, written_at] = params as [Uint8Array, number];
          const row = { seq: nextSeq++, blob, written_at };
          wal.push(row);
          return { changes: 1, lastInsertRowid: row.seq };
        },
        get: () => undefined,
        all: () => [],
      };
    }

    // WAL read.
    if (/^SELECT blob FROM smritidb_wal ORDER BY seq ASC/i.test(trimmed)) {
      return {
        all: () => wal.slice().sort((a, b) => a.seq - b.seq).map((r) => ({ blob: r.blob })),
        get: () => (wal[0] ? { blob: wal[0].blob } : undefined),
        run: () => ({ changes: 0, lastInsertRowid: 0 }),
      };
    }

    // WAL truncate.
    if (/^DELETE FROM smritidb_wal$/i.test(trimmed)) {
      return {
        run: () => {
          const n = wal.length;
          wal.length = 0;
          return { changes: n, lastInsertRowid: 0 };
        },
        get: () => undefined,
        all: () => [],
      };
    }

    // Legacy snapshot read used by the migrator.
    if (
      /^SELECT value, updated_at FROM smritidb ORDER BY updated_at DESC LIMIT 1/i.test(trimmed)
    ) {
      const rows = Array.from(legacy.values()).sort((a, b) => b.updated_at - a.updated_at);
      const top = rows[0];
      return {
        get: () => (top ? { value: top.value, updated_at: top.updated_at } : undefined),
        run: () => ({ changes: 0, lastInsertRowid: 0 }),
        all: () => rows.map((r) => ({ value: r.value, updated_at: r.updated_at })),
      };
    }

    throw new Error(`mockDb.prepare: unsupported sql: ${trimmed}`);
  };

  return {
    pragma: () => null,
    exec,
    prepare,
    close: () => {},
    __seedLegacy(key, value, updatedAt) {
      tables.add("smritidb");
      legacy.set(key, { key, value, updated_at: updatedAt });
    },
    __tables: () => Array.from(tables),
  };
}

describe("sqliteAdapter", () => {
  it("round-trips through the adapter interface", async () => {
    const db = mockDb();
    const adapter = sqliteAdapter(db);

    assert.equal(await adapter.read(), null, "fresh table is empty");
    await adapter.write(new Uint8Array([1, 2, 3, 4]));
    const got = await adapter.read();
    assert.ok(got);
    assert.deepEqual(Array.from(got!), [1, 2, 3, 4]);
    await adapter.remove?.();
    assert.equal(await adapter.read(), null);
  });

  it("persists a Smritidb store end-to-end", async () => {
    const db = mockDb();
    const original = new Smritidb({ dimension: 2048 });
    original.put("alpha", "alpha", { tags: ["greek"] });
    original.put("beta", "beta");

    await persistStore(original, sqliteAdapter(db));

    const restored = await openPersistentStore({
      dimension: 2048,
      adapter: sqliteAdapter(db),
    });
    assert.equal(restored.size(), 2);
    const hits = restored.recall("alpha", { topK: 1, minSimilarity: 0.9 });
    assert.equal(hits.length, 1);
    assert.equal(new TextDecoder().decode(hits[0]!.item.value), "alpha");
  });

  it("supports the WAL append / read / truncate cycle", async () => {
    const db = mockDb();
    const adapter = sqliteAdapter(db);

    await adapter.appendWal(new Uint8Array([1]));
    await adapter.appendWal(new Uint8Array([2, 2]));
    await adapter.appendWal(new Uint8Array([3, 3, 3]));

    const entries = await adapter.readWal();
    assert.equal(entries.length, 3);
    assert.deepEqual(Array.from(entries[0]!), [1]);
    assert.deepEqual(Array.from(entries[1]!), [2, 2]);
    assert.deepEqual(Array.from(entries[2]!), [3, 3, 3]);

    await adapter.truncateWal();
    assert.deepEqual(await adapter.readWal(), []);
  });

  it("migrates from the legacy `smritidb` schema on first open", async (t) => {
    const db = mockDb();
    // Pretend the file was written by the Phase-1 TS adapter.
    const legacyBlob = new Uint8Array([7, 7, 7, 7, 7, 7, 7]);
    db.__seedLegacy("default", legacyBlob, 1_700_000_000_000);

    // Silence the migration warning.
    const origWarn = console.warn;
    const warnings: string[] = [];
    console.warn = (msg?: unknown) => {
      warnings.push(String(msg));
    };
    t.after(() => {
      console.warn = origWarn;
    });

    const adapter = sqliteAdapter(db);
    const got = await adapter.read();
    assert.ok(got, "legacy blob should be readable through the new adapter");
    assert.deepEqual(Array.from(got!), Array.from(legacyBlob));

    // Legacy table should be gone after the one-shot migration.
    assert.equal(db.__tables().includes("smritidb"), false);
    assert.match(warnings.join("\n"), /migrated legacy/);
  });

  it("close() is idempotent", async () => {
    const db = mockDb();
    const adapter = sqliteAdapter(db);
    await adapter.write(new Uint8Array([1, 2, 3]));
    await adapter.close();
    await adapter.close(); // no-op
  });

  it("migrateLegacySchema returns false when there's nothing to migrate", () => {
    const db = mockDb();
    // Bootstrap canonical tables (mirrors what the adapter does on open).
    db.exec(
      `CREATE TABLE IF NOT EXISTS smritidb_snapshot (id INTEGER PRIMARY KEY, blob BLOB NOT NULL, written_at INTEGER NOT NULL);
       CREATE TABLE IF NOT EXISTS smritidb_wal (seq INTEGER PRIMARY KEY AUTOINCREMENT, blob BLOB NOT NULL, written_at INTEGER NOT NULL);`,
    );
    assert.equal(migrateLegacySchema(db), false);
  });
});

/**
 * Cross-binary compatibility against a SQLite file produced by the Rust
 * polyglot demo. Skipped when the fixture is absent — the test only runs
 * after `examples/polyglot-interop/run-all.sh` has been executed at least
 * once on this machine.
 */
describe("sqliteAdapter (cross-binary compat with Rust)", () => {
  const FIXTURE = "/tmp/smritidb-polyglot.db";
  const fixtureExists = existsSync(FIXTURE);

  it("reads a SQLite file produced by the Rust core via the polyglot demo", { skip: !fixtureExists }, async () => {
    // We need the real `better-sqlite3` here — the mockDb won't deserialise an
    // on-disk database. Dynamic import keeps this test optional in
    // environments where the native module isn't built.
    let Database: { default: new (path: string, opts?: { readonly?: boolean }) => SqliteDatabase };
    try {
      Database = (await import("better-sqlite3")) as unknown as typeof Database;
    } catch (e) {
      // No native module available — skip rather than fail.
      // (Node's test runner has no late-skip, so we just assert true.)
      console.warn(`[sqlite.test] better-sqlite3 unavailable, skipping: ${(e as Error).message}`);
      return;
    }
    const handle = new Database.default(FIXTURE, { readonly: true });
    const adapter = sqliteAdapter(handle);
    const blob = await adapter.read();
    assert.ok(blob, "polyglot fixture should expose a snapshot blob");
    const snap = readKmf(blob!);
    // The polyglot demo writes 100 items (see step1_python_write.py).
    assert.equal(snap.items.length, 100);
    await adapter.close();
  });
});
