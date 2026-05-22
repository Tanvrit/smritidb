# Smritidb Benchmark Suite

A cross-binding benchmark harness that measures Smritidb's performance
across its four production-ready bindings:

| Binding      | Toolchain                  | Source path                |
|---           |---                         |---                         |
| Rust         | cargo + criterion 0.5      | `rust/`                    |
| Python       | CPython + PyO3 module      | `python/`                  |
| Kotlin/JVM   | Gradle + JMH (manual)      | `kotlin/`                  |
| TypeScript   | Node + tinybench           | `typescript/`              |

Each binding runs the **same workload definition** so numbers are directly
comparable. The shared workload is documented in `WORKLOAD.md` and reflected
identically in every implementation.

## What the suite measures

1. **Primitives throughput** — `random_hv`, `bundle`, `bind`, `similarity`,
   `encode_string`. Each operation is timed across a fixed input batch and
   reported as operations per second.

2. **Recall latency** — wall-clock time to brute-force search a cleanup
   memory of size N ∈ {100, 1000, 10000}. p50, p99, p999 reported.

3. **Persistence throughput** — SQLite snapshot write/read at N=10000 items
   (D=10000 bits = ~12.5 MB of raw key bytes). MB/s of the serialised KMF
   blob.

4. **Memory footprint** — process RSS before/after inserting 10000 items,
   divided by N to give bytes/item. Compared against the theoretical floor
   D / 8 = 1250 bytes/item.

These numbers give numeric backing to the quantitative claims in the
patent and serve as a baseline for future regression detection.

## Running

```bash
# Run every binding, collect results, render RESULTS.md
bash run-all.sh

# Or run an individual binding
cd rust && cargo bench
cd python && ../../.venv/bin/python bench_primitives.py
cd typescript && node --import tsx bench-primitives.ts
cd kotlin && gradle benchmark
```

Each binding writes its raw timings to `results/<lang>.json`. The
`scripts/render_results.py` script aggregates those into `RESULTS.md`.

## Configuration constants

These are fixed across every binding so comparisons are valid:

- `D = 10000` (hypervector dimension; one byte per bit on the wire).
- `RANDOM_HV_BATCH = 1000` (number of random hypervectors generated).
- `BUNDLE_BATCH = 100` (pairs).
- `BIND_BATCH = 100` (pairs).
- `SIMILARITY_BATCH = 10000` (pairs).
- `ENCODE_STRING_BATCH = 10000` (varied inputs).
- `RECALL_N = [100, 1000, 10000]`.
- `RECALL_QUERIES = 1000`.
- `PERSIST_N = 10000`.
- `WAL_APPENDS = 1000`.

Warm-up: each binding runs three throwaway iterations before measuring,
to flush JIT compilation (Kotlin/JVM, V8), to prime instruction caches,
and to give branch predictors a chance to settle.

## Toolchain prerequisites

- Rust 1.75+ (we tested with 1.95).
- Python 3.10+ with the `smritidb` PyO3 module installed (`pip install -e packages/smritidb-py`).
- Node 22+ with `pnpm install` already run at the repo root.
- JDK 17+ and Gradle 8+ for the JVM benchmark. The JVM bench depends on
  `smritidb-kmp` being built first (`cd packages/smritidb-kmp && gradle jvmJar`).

If any toolchain is missing, that binding's results column will read
"N/A" in `RESULTS.md` with a footnote explaining why.

## Notes on methodology

- Each binding measures **the full call path through its public API**.
  That means the Python and Kotlin numbers include FFI overhead, while
  the TypeScript numbers reflect the pure-JS reference implementation
  (no native fallback). This is intentional: the goal is to measure
  what a real consumer of each binding sees.
- All benchmarks run single-threaded with network disabled.
- Disable laptop power-saving / Turbo Boost variation as best the OS
  allows. On macOS the script suggests using `caffeinate`.
- `GC` pauses in the JVM bench are mitigated by a 5 s warmup pre-run.
- Process RSS is sampled via the platform's native tooling
  (`getrusage()` on Unix / `psutil` in Python / `Runtime.totalMemory()`
  in JVM / `process.memoryUsage().rss` in Node).

## Layout

```
benchmarks/
  README.md          this file
  WORKLOAD.md        canonical workload spec
  RESULTS.md         rendered output of run-all.sh
  run-all.sh         orchestrator
  rust/              criterion benches
  python/            pytest-benchmark + plain timing
  kotlin/            Gradle / JMH wrapper
  typescript/        tinybench scripts
  scripts/           result aggregation
  results/           per-binding JSON output (gitignored)
```
