// Persistence + memory benchmark — TypeScript binding.

import { performance } from "node:perf_hooks";
import { mkdtempSync, statSync, unlinkSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import Database from "better-sqlite3";
import {
  Smritidb,
  openPersistentStore,
  persistStore,
  sqliteAdapter,
} from "@tanvrit/smritidb";

import { D, PERSIST_N, WAL_APPENDS, WARMUP_ITERS } from "./bench-utils.js";

export interface PersistOp {
  n: number;
  bytes: number;
  wall_seconds: number;
  mb_per_sec: number;
}

export interface WalResult {
  n: number;
  wall_seconds: number;
  ops_per_sec: number;
  note?: string;
}

export interface PersistenceOut {
  snapshot_write: PersistOp;
  snapshot_read: PersistOp;
  wal_append: WalResult;
}

function populate(store: Smritidb, n: number): void {
  for (let i = 0; i < n; i++) {
    store.put(`item_${i}`, `value for item ${i}`);
  }
}

function fileSize(path: string): number {
  try {
    return statSync(path).size;
  } catch {
    return 0;
  }
}

function cleanupFile(path: string) {
  for (const suffix of ["", "-journal", "-wal", "-shm"]) {
    try {
      unlinkSync(path + suffix);
    } catch {
      // ignore
    }
  }
}

export async function runPersistence(): Promise<PersistenceOut> {
  console.log("[3/4] persistence (TypeScript)");

  const dir = mkdtempSync(join(tmpdir(), "smritidb-bench-"));
  const writePath = join(dir, "write.db");
  const readPath = join(dir, "read.db");

  // -- snapshot_write --
  cleanupFile(writePath);
  const writeStore = new Smritidb({ dimension: D });
  populate(writeStore, PERSIST_N);
  const writeDb = new Database(writePath);
  const writeAdapter = sqliteAdapter(writeDb);
  const tw0 = performance.now();
  await persistStore(writeStore, writeAdapter);
  const writeWall = (performance.now() - tw0) / 1000;
  await writeAdapter.close();
  const writeBytes = fileSize(writePath);
  const writeMbs = writeBytes / 1_000_000 / writeWall;
  console.log(
    `  snapshot_write N=${PERSIST_N} bytes=${writeBytes} wall=${writeWall.toFixed(
      3,
    )}s = ${writeMbs.toFixed(1)} MB/s`,
  );

  // -- snapshot_read --
  cleanupFile(readPath);
  const seedStore = new Smritidb({ dimension: D });
  populate(seedStore, PERSIST_N);
  const seedDb = new Database(readPath);
  const seedAdapter = sqliteAdapter(seedDb);
  await persistStore(seedStore, seedAdapter);
  await seedAdapter.close();
  const readBytes = fileSize(readPath);

  for (let i = 0; i < WARMUP_ITERS; i++) {
    const db = new Database(readPath, { readonly: true });
    const ad = sqliteAdapter(db);
    await openPersistentStore({ dimension: D, adapter: ad });
    await ad.close();
  }

  const tr0 = performance.now();
  const readDb = new Database(readPath, { readonly: true });
  const readAdapter = sqliteAdapter(readDb);
  const restored = await openPersistentStore({ dimension: D, adapter: readAdapter });
  const readWall = (performance.now() - tr0) / 1000;
  if (restored.size() !== PERSIST_N) {
    throw new Error(`restored.size() = ${restored.size()}, expected ${PERSIST_N}`);
  }
  await readAdapter.close();
  const readMbs = readBytes / 1_000_000 / readWall;
  console.log(
    `  snapshot_read  N=${PERSIST_N} bytes=${readBytes} wall=${readWall.toFixed(
      3,
    )}s = ${readMbs.toFixed(1)} MB/s`,
  );

  // -- wal_append (surrogate: TS adapter exposes write/read but no
  // append_wal on the public surface; persist() rewrites the snapshot.
  // Footnote in RESULTS.md.).
  const walPath = join(dir, "wal.db");
  cleanupFile(walPath);
  const walDb = new Database(walPath);
  const walAdapter = sqliteAdapter(walDb);
  const walStore = new Smritidb({ dimension: D });
  walStore.put("seed", "x");

  for (let i = 0; i < WARMUP_ITERS; i++) {
    await persistStore(walStore, walAdapter);
  }
  const tWal0 = performance.now();
  for (let i = 0; i < WAL_APPENDS; i++) {
    await persistStore(walStore, walAdapter);
  }
  const walWall = (performance.now() - tWal0) / 1000;
  const walOps = WAL_APPENDS / walWall;
  await walAdapter.close();
  console.log(
    `  wal_append*   N=${WAL_APPENDS} wall=${walWall.toFixed(3)}s = ${walOps.toFixed(
      0,
    )} ops/s (surrogate: repeated persistStore())`,
  );

  return {
    snapshot_write: {
      n: PERSIST_N,
      bytes: writeBytes,
      wall_seconds: writeWall,
      mb_per_sec: writeMbs,
    },
    snapshot_read: {
      n: PERSIST_N,
      bytes: readBytes,
      wall_seconds: readWall,
      mb_per_sec: readMbs,
    },
    wal_append: {
      n: WAL_APPENDS,
      wall_seconds: walWall,
      ops_per_sec: walOps,
      note: "TS adapter exposes write/read; this measures repeated full persistStore() instead.",
    },
  };
}

export function runMemory(): {
  n_items: number;
  rss_before_bytes: number;
  rss_after_bytes: number;
  bytes_per_item: number;
  theoretical_floor_bytes_per_item: number;
} {
  console.log("[4/4] memory (TypeScript)");
  // V8 schedules background GC asynchronously, so process.rss is noisy.
  // We prefer heapUsed when --expose-gc is available (run-all.sh passes it);
  // otherwise we fall back to rss, which can swing wildly. To make the
  // benchmark useful even without --expose-gc we synthesize the floor from
  // the dominant cost: PERSIST_N × D bytes of Uint8Array key data.
  const hasGc = typeof (globalThis as { gc?: () => void }).gc === "function";
  if (hasGc) (globalThis as { gc: () => void }).gc();
  const before = hasGc ? process.memoryUsage().heapUsed : process.memoryUsage().rss;
  const store = new Smritidb({ dimension: D });
  for (let i = 0; i < PERSIST_N; i++) {
    store.put(`item_${i}`, `value for item ${i}`);
  }
  if (hasGc) (globalThis as { gc: () => void }).gc();
  const after = hasGc ? process.memoryUsage().heapUsed : process.memoryUsage().rss;
  const delta = Math.max(0, after - before);
  const perItem = delta / PERSIST_N;
  console.log(
    `  ${hasGc ? "heapUsed" : "rss"} before=${before}B after=${after}B ` +
      `delta=${delta}B = ${perItem.toFixed(1)} bytes/item ` +
      `${hasGc ? "" : "(noisy without --expose-gc)"}`,
  );
  // Keep store alive past the sample.
  void store.size();
  return {
    n_items: PERSIST_N,
    rss_before_bytes: before,
    rss_after_bytes: after,
    bytes_per_item: perItem,
    theoretical_floor_bytes_per_item: D / 8,
  };
}

if (import.meta.url === `file://${process.argv[1]}`) {
  runPersistence().then((p) => {
    console.log(JSON.stringify({ persistence: p, memory: runMemory() }, null, 2));
  });
}
