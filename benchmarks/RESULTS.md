# Smritidb Benchmark Results

**Run on:** 2026-05-22 06:14 UTC
**Host:** Darwin 25.3.0, Apple M2 Max
**Commit:** `17334f8278c1`

Workload definition: see [WORKLOAD.md](./WORKLOAD.md).

---

## Primitives (operations per second, higher is better)

| Operation | Rust | Python | Kotlin/JVM | TypeScript |
|---|---:|---:|---:|---:|
| random_hv | 125,564 | 129,874 | 7,407 | 32,763 |
| bundle | 2,132 | 1,529 | 1,748 | 138 |
| bind | 2,047,796 | 1,509,432 | 4,418 | 64,005 |
| similarity | 442,255 | 499,278 | 14,909 | 20,556 |
| encode_string | 123,647 | 122,536 | 21,991 | 38,786 |

### Rust vs other bindings ratio

| Operation | Python | Kotlin/JVM | TypeScript |
|---|---:|---:|---:|
| random_hv | 0.97x | 16.95x | 3.83x |
| bundle | 1.39x | 1.22x | 15.45x |
| bind | 1.36x | 463.48x | 31.99x |
| similarity | 0.89x | 29.66x | 21.51x |
| encode_string | 1.01x | 5.62x | 3.19x |

Read as: Rust is N× faster than the named binding at this op.

## Recall latency (milliseconds, lower is better)

| N items | Rust p50 | Rust p99 | Python p50 | Python p99 | Kotlin/JVM p50 | Kotlin/JVM p99 | TS p50 | TS p99 |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 100 | 0.173 | 0.201 | 0.164 | 0.192 | 0.208 | 0.373 | 4.41 | 4.93 |
| 1000 | 1.70 | 1.99 | 1.67 | 1.98 | 1.74 | 1.93 | 44.32 | 45.47 |
| 10000 | 18.47 | 20.41 | 17.85 | 20.09 | 18.36 | 22.43 | 444.64 | 581.33 |

## Persistence throughput (higher is better)

| Operation | Unit | Rust | Python | Kotlin/JVM | TypeScript |
|---|---|---:|---:|---:|---:|
| snapshot_write | MB/s | 33.9 | 33.8 | N/A[^kotlin-persist] | 20.2 |
| snapshot_read | MB/s | 141.4 | 140.9 | N/A[^kotlin-persist] | 38.1 |
| wal_append | ops/s | 18,037 | 14,315 | N/A[^kotlin-persist] | 14,257 |

## Memory footprint

| Binding | Bytes/item | vs theoretical floor (1250 B = 10000 bits / 8) |
|---|---:|---:|
| rust | 0.0 | 0.00× |
| python | 0.0 | 0.00× |
| kotlin | 0.0 | 0.00× |
| typescript | 711.0 | 0.57× |

The theoretical floor counts only the binary hypervector key (D=10000 bits = 1250 B). Real bindings add metadata (uuid, value bytes, tags, map overhead), so 1.5–3× the floor is normal. Ratios above 5× indicate runtime overhead worth investigating.

_Caveat: on macOS the system allocator and memory compressor often keep RSS flat across allocations under ~50 MB, so readings <100 B/item should be treated as "below RSS resolution" rather than "zero overhead". The TypeScript number is a V8 heapUsed sample taken after `--expose-gc` collection, which is more reliable but undercounts off-heap Uint8Array backing buffers. Run on Linux for the most reliable per-item numbers._

---

### Footnotes

- `python` wal_append: Python binding has no direct append_wal; this measures repeated full persist() instead.
[^kotlin-persist]: `kotlin` persistence: smritidb-kmp commonMain has only an in-memory Store on the JVM today; persistence is wired via UniFFI on iOS / Apple native but the JVM `openStore` API does not yet have an `openSqlite` actual.
- `typescript` wal_append: TS adapter exposes write/read; this measures repeated full persistStore() instead.
