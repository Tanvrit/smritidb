/**
 * Step 4 — TypeScript reader for the polyglot interop demo.
 *
 * Opens the SQLite file written by Step 1 (Python) through the canonical TS
 * `sqliteAdapter()`, pulls out the snapshot BLOB, decodes it with `readKmf`
 * from the TS reference implementation, and asserts:
 *
 *   - the KMF item count matches what the producer wrote
 *   - the value bytes for `id == "<the concept_50 row>"` are byte-identical
 *     to what Python wrote
 *
 * Schema note (KNOWN-ISSUES.md, item P4-1 — RESOLVED):
 *   The Rust and TypeScript adapters now share the canonical schema
 *
 *       smritidb_snapshot(id INTEGER PRIMARY KEY, blob BLOB, written_at INTEGER)
 *       smritidb_wal(seq INTEGER PRIMARY KEY AUTOINCREMENT, blob BLOB, written_at INTEGER)
 *
 *   so step 4 now goes through the TS adapter end-to-end (no raw SQL).
 */

import Database from "better-sqlite3";
import { readKmf, sqliteAdapter, type KmfItem } from "@tanvrit/smritidb";

const PATH = "/tmp/smritidb-polyglot.db";
const EXPECTED_ITEMS = 100;
const EXPECTED_TOP_VALUE = Buffer.from("item value 50", "ascii");

async function main(): Promise<number> {
  // Open read-only via better-sqlite3, then hand the connection to the
  // canonical TS adapter. The adapter speaks the same schema as the Rust core,
  // so no manual SQL is needed.
  const db = new Database(PATH, { readonly: true });
  const adapter = sqliteAdapter(db);

  const blob = await adapter.read();
  if (!blob) {
    console.error("FAIL: no snapshot row in smritidb_snapshot table");
    return 1;
  }
  const snapshot = readKmf(blob);

  if (snapshot.items.length !== EXPECTED_ITEMS) {
    console.error(`FAIL: KMF item_count = ${snapshot.items.length}, expected ${EXPECTED_ITEMS}`);
    return 1;
  }

  // Walk the KMF items, find the one Python keyed as "concept_50". The KMF
  // wire format doesn't carry the original string cue (only the hypervector
  // derived from it) — instead we identify the row by its *value* bytes,
  // which are exactly what byte-identity is supposed to prove.
  const match: KmfItem | undefined = snapshot.items.find(
    (it) => Buffer.from(it.value).equals(EXPECTED_TOP_VALUE),
  );
  if (!match) {
    console.error(`FAIL: no KMF item with value == "item value 50"`);
    // Dump first 3 ids to help debug.
    const sample = snapshot.items.slice(0, 3).map((it) => ({
      id: it.id,
      valueLen: it.value.length,
      valuePreview: Buffer.from(it.value).subarray(0, 20).toString("ascii"),
    }));
    console.error("first items:", JSON.stringify(sample, null, 2));
    return 1;
  }

  // Cross-check: the binary-HDC hypervector key length equals `dimension`
  // (one byte per dim, holding 0 or 1) — proves the KMF header dimension
  // and the per-item key blocks agree on the wire.
  if (match.key.length !== snapshot.dimension) {
    console.error(
      `FAIL: hypervector length mismatch: ${match.key.length} bytes, expected ${snapshot.dimension}`,
    );
    return 1;
  }

  console.log(
    `PASS: TypeScript read ${snapshot.items.length} items from ${PATH} ` +
      `(via sqliteAdapter, matched value="${Buffer.from(match.value).toString("ascii")}", ` +
      `id=${match.id}, dim=${snapshot.dimension}, hv_len=${match.key.length} bytes)`,
  );

  await adapter.close();
  return 0;
}

main().then((code) => process.exit(code));
