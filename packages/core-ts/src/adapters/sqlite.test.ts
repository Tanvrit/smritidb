import { strict as assert } from "node:assert";
import { describe, it } from "node:test";

import { sqliteAdapter, type SqliteDatabase } from "./sqlite.js";
import { Smritidb } from "../store.js";
import { openPersistentStore, persistStore } from "./index.js";

/** Minimal in-memory SQLite mock satisfying the SqliteDatabase contract. */
function mockDb(): SqliteDatabase {
  const tables = new Map<string, Map<string, { value: Uint8Array; updated_at: number }>>();
  const ensureTable = (sql: string): string | null => {
    const m = sql.match(/CREATE TABLE IF NOT EXISTS "([^"]+)"/);
    if (m) {
      if (!tables.has(m[1]!)) tables.set(m[1]!, new Map());
      return m[1]!;
    }
    return null;
  };
  const tableFromSql = (sql: string): string => {
    const m = sql.match(/FROM "([^"]+)"|INTO "([^"]+)"/);
    if (!m) throw new Error(`mockDb: cannot parse table from sql: ${sql}`);
    return (m[1] ?? m[2])!;
  };

  return {
    pragma() {
      return null;
    },
    exec(sql: string) {
      ensureTable(sql);
    },
    prepare<P extends unknown[], R>(sql: string) {
      const isSelect = /^\s*SELECT/i.test(sql);
      const isInsert = /^\s*INSERT/i.test(sql);
      const isDelete = /^\s*DELETE/i.test(sql);
      const table = tableFromSql(sql);
      return {
        get: ((...params: P) => {
          if (!isSelect) throw new Error("mockDb: get on non-SELECT");
          const t = tables.get(table);
          if (!t) return undefined;
          const row = t.get(params[0] as string);
          return row ? ({ value: row.value } as unknown as R) : undefined;
        }) as (...p: P) => R | undefined,
        run: ((...params: P) => {
          const t = tables.get(table) ?? new Map();
          tables.set(table, t);
          if (isInsert) {
            const [key, value, updated_at] = params as [string, Uint8Array, number];
            t.set(key, { value, updated_at });
            return { changes: 1, lastInsertRowid: 0 };
          }
          if (isDelete) {
            const had = t.delete(params[0] as string);
            return { changes: had ? 1 : 0, lastInsertRowid: 0 };
          }
          throw new Error("mockDb: run on unknown sql kind");
        }) as (...p: P) => { changes: number; lastInsertRowid: number | bigint },
      };
    },
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

  it("supports multiple rowKeys in one table", async () => {
    const db = mockDb();
    const adapterA = sqliteAdapter(db, { rowKey: "alpha" });
    const adapterB = sqliteAdapter(db, { rowKey: "beta" });
    await adapterA.write(new Uint8Array([1, 2]));
    await adapterB.write(new Uint8Array([3, 4]));
    assert.deepEqual(Array.from((await adapterA.read())!), [1, 2]);
    assert.deepEqual(Array.from((await adapterB.read())!), [3, 4]);
  });
});
