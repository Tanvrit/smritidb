# Canonical Benchmark Workload

Every binding implements this workload identically. Any divergence is
a benchmark bug, not a binding bug.

## Constants

```
D                     = 10000
RANDOM_HV_BATCH       = 1000        # ops
BUNDLE_BATCH          = 100         # pair ops (each pair = bundle of 2 HVs)
BIND_BATCH            = 100         # pair ops
SIMILARITY_BATCH      = 10000       # pair ops
ENCODE_STRING_BATCH   = 10000       # varied inputs

RECALL_NS             = [100, 1000, 10000]
RECALL_QUERIES        = 1000
RECALL_TOP_K          = 10
RECALL_MIN_SIM        = 0.5

PERSIST_N             = 10000
WAL_APPENDS           = 1000

WARMUP_ITERS          = 3
```

## Seeded inputs

Every input is derived deterministically so all bindings hit the same
bytes. Seeds are little-endian decimal strings of the loop index unless
otherwise specified.

### Primitives

- `random_hv` batch: seeds `b"hv_0"..b"hv_999"`, dim D.
- `bundle` batch: for `i in 0..BUNDLE_BATCH`, bundle of
  `[random_hv(b"a{i}"), random_hv(b"b{i}")]`.
- `bind` batch: pairs `(random_hv(b"a{i}"), random_hv(b"b{i}"))`.
- `similarity` batch: same pair-generation as `bind`, then compute
  similarity of each pair. The two HVs in each pair are pre-generated
  outside the measured region.
- `encode_string` batch: input `"item_{i}"` for `i in 0..10000`.

### Recall

- For each N in `[100, 1000, 10000]`:
  1. Insert N items with key+value = `"item_{i}"`.
  2. Run 1000 recall queries, cycling cues `"item_{i % N}"`, top_k=10, min_sim=0.5.
  3. Record per-query latency. Report p50, p99, p999 (milliseconds).

### Persistence

- Insert PERSIST_N items keyed `"item_{i}"`, value `"value for item {i}"`.
- Snapshot to SQLite at `/tmp/smritidb-bench-{lang}.db`.
- Measure: wall-clock of `persist()` and final file size in bytes.
  Throughput in MB/s = blob_size / persist_seconds.
- Read it back: re-open the file, time the load. Throughput = blob_size / load_seconds.
- WAL: append WAL_APPENDS deltas (one put each). For bindings whose
  adapter doesn't expose an append API, fall back to repeated full
  persist() calls and record N=1 throughput instead.

### Memory footprint

- Sample process RSS before insert.
- Insert PERSIST_N items (same workload as persistence).
- Force a GC / allocator settle (best effort).
- Sample RSS again.
- Report `(rss_after - rss_before) / PERSIST_N` bytes per item.

The theoretical floor is `D / 8 = 1250` bytes for the key alone. Add
~64 bytes for the UUID + metadata + map overhead → roughly 1.3–1.5
KB/item is a reasonable target. Higher numbers reveal interpreter /
runtime tax.

## Output schema

Every binding emits `results/<lang>.json` matching this schema:

```json
{
  "binding": "rust" | "python" | "kotlin" | "typescript",
  "version": "0.1.0",
  "spec_version": "0.1.0-draft",
  "host": { "os": "...", "arch": "...", "cpu": "..." },
  "primitives": {
    "random_hv":     { "ops_per_sec": 12345.6, "iters": 1000, "wall_seconds": 0.08 },
    "bundle":        { "ops_per_sec": ..., "iters": 100,  "wall_seconds": ... },
    "bind":          { "ops_per_sec": ..., "iters": 100,  "wall_seconds": ... },
    "similarity":    { "ops_per_sec": ..., "iters": 10000, "wall_seconds": ... },
    "encode_string": { "ops_per_sec": ..., "iters": 10000, "wall_seconds": ... }
  },
  "recall": [
    { "n": 100,   "queries": 1000, "p50_ms": ..., "p99_ms": ..., "p999_ms": ..., "insert_seconds": ... },
    { "n": 1000,  ... },
    { "n": 10000, ... }
  ],
  "persistence": {
    "snapshot_write": { "n": 10000, "bytes": 12345678, "wall_seconds": ..., "mb_per_sec": ... },
    "snapshot_read":  { "n": 10000, "bytes": 12345678, "wall_seconds": ..., "mb_per_sec": ... },
    "wal_append":     { "n": 1000,  "wall_seconds": ..., "ops_per_sec": ... }
  },
  "memory": {
    "n_items": 10000,
    "rss_before_bytes": ...,
    "rss_after_bytes":  ...,
    "bytes_per_item":   ...,
    "theoretical_floor_bytes_per_item": 1250
  }
}
```
