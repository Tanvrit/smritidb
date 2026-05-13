import { strict as assert } from "node:assert";
import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, it, after } from "node:test";

import { Smritidb } from "../store.js";
import {
  MemoryAdapter,
  fsAdapter,
  openPersistentStore,
  persistStore,
  withPersistence,
} from "./index.js";

const tempDirs: string[] = [];

async function tempDir(): Promise<string> {
  const dir = await mkdtemp(join(tmpdir(), "smritidb-"));
  tempDirs.push(dir);
  return dir;
}

after(async () => {
  for (const d of tempDirs) await rm(d, { recursive: true, force: true });
});

describe("MemoryAdapter", () => {
  it("round-trips a Smritidb store", async () => {
    const store = new Smritidb({ dimension: 2048 });
    store.put("alpha", "alpha");
    store.put("beta", "beta");
    const adapter = new MemoryAdapter();
    await persistStore(store, adapter);

    const restored = await openPersistentStore({ dimension: 2048, adapter });
    assert.equal(restored.size(), 2);
    const hits = restored.recall("alpha", { topK: 1, minSimilarity: 0.9 });
    assert.equal(hits.length, 1);
    assert.equal(new TextDecoder().decode(hits[0]!.item.value), "alpha");
  });

  it("returns null on empty adapter", async () => {
    const adapter = new MemoryAdapter();
    assert.equal(await adapter.read(), null);
  });
});

describe("fsAdapter", () => {
  it("round-trips a Smritidb store on disk", async () => {
    const dir = await tempDir();
    const path = join(dir, "store.kmf");

    const original = new Smritidb({ dimension: 2048 });
    original.put("the cat sat on the mat", "the cat sat on the mat");
    original.put("a bird in the hand", "a bird in the hand");

    await persistStore(original, fsAdapter(path));

    const restored = await openPersistentStore({
      dimension: 2048,
      adapter: fsAdapter(path),
    });
    assert.equal(restored.size(), 2);
    const hits = restored.recall("the cat sat on the mat", { topK: 1, minSimilarity: 0.9 });
    assert.equal(hits.length, 1);
    assert.equal(hits[0]!.similarity, 1);
  });

  it("writes via temp + rename for atomicity", async () => {
    const fs = await import("node:fs/promises");
    const dir = await tempDir();
    const path = join(dir, "store.kmf");

    const store = new Smritidb({ dimension: 2048 });
    store.put("alpha", "alpha");
    await persistStore(store, fsAdapter(path));

    // The .tmp sibling should NOT remain after a successful write.
    const tmpExists = await fs.access(`${path}.tmp`).then(() => true, () => false);
    assert.equal(tmpExists, false);

    // The real file IS present.
    const realExists = await fs.access(path).then(() => true, () => false);
    assert.equal(realExists, true);
  });

  it("read returns null when file is absent", async () => {
    const dir = await tempDir();
    const path = join(dir, "missing.kmf");
    const adapter = fsAdapter(path);
    assert.equal(await adapter.read(), null);
  });

  it("openPersistentStore creates fresh when adapter is empty", async () => {
    const dir = await tempDir();
    const path = join(dir, "store.kmf");
    const store = await openPersistentStore({
      dimension: 2048,
      adapter: fsAdapter(path),
    });
    assert.equal(store.size(), 0);
  });
});

describe("withPersistence", () => {
  it("auto-persists on put after the debounce window", async () => {
    const adapter = new MemoryAdapter();
    const store = withPersistence(
      new Smritidb({ dimension: 2048 }),
      adapter,
      { debounceMs: 5 },
    );

    store.put("alpha", "alpha");
    store.put("beta", "beta");
    store.put("gamma", "gamma");

    // Wait past the debounce window
    await new Promise((r) => setTimeout(r, 30));

    const persisted = await adapter.read();
    assert.ok(persisted, "expected persisted bytes after debounce");

    const restored = await openPersistentStore({ dimension: 2048, adapter });
    assert.equal(restored.size(), 3);
  });
});
