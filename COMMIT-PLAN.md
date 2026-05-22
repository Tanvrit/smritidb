# Commit plan for the Mission 2 working tree

Pinned to: HEAD = `17334f8` (Follow-up: text encoders, conformance corpus, SQLite adapter).

Working tree summary (at plan time):
- 36 modified tracked files
- 157 untracked files (134 of which are not in `patent/`)
- Total ~190 files across ~60 distinct logical changes

The groupings below are intentionally orthogonal so each one can be reviewed and committed in isolation. Within each group, the suggested order is `git add <files> && git commit -m "<msg>"`. Run `git status -uall --short` after each commit to verify the tree shrinks as expected.

---

## Group 1 — Mission 2 Phase A: Rust core persistence

The persistence trait, in-memory + fs + SQLite adapters, the durable `PersistentStore`, the KMF serializer, and the Hebbian consolidation pass. This is the load-bearing change every other binding lifts.

**Files:**

```
packages/core-rs/Cargo.toml
packages/core-rs/Cargo.lock
packages/core-rs/src/lib.rs
packages/core-rs/src/store.rs                       (new)
packages/core-rs/src/kmf.rs                         (new)
packages/core-rs/src/consolidate.rs                 (new)
packages/core-rs/src/persist/mod.rs                 (new)
packages/core-rs/src/persist/memory.rs              (new)
packages/core-rs/src/persist/fs.rs                  (new)
packages/core-rs/src/persist/sqlite.rs              (new)
packages/core-rs/src/text.rs
packages/core-rs/src/wasm.rs
packages/core-rs/benches/primitives.rs
packages/core-rs/tests/conformance.rs
packages/core-rs/tests/kmf_cross_impl.rs            (new)
packages/core-rs/README.md
```

**Suggested message:**

> `core-rs: Mission 2 Phase A — Store, PersistentStore, KMF, SQLite adapter`
>
> Adds the durable Store + PersistentStore types behind a PersistenceAdapter
> trait. Ships three adapters (in-memory, append-only file, SQLite via
> bundled rusqlite). Adds the KMF serializer that every binding round-trips
> bit-exactly; introduces tests/kmf_cross_impl.rs which gates the
> tests/conformance/kmf_fixture.bin against the Rust core's emitter. 50
> tests passing.

---

## Group 2 — Mission 2 Phase B: FFI + Python + core-ts + KMP common

The cross-binding persistence integration. Each binding gains
`PersistentStore.openSqlite()` / `openMemory()` factories that delegate
into the Phase A Rust core through its native lift/lower path. Also
includes the smritidb-ffi cargo config (Linux cross-compile linker
pins) and persistence tests.

**Files:**

```
packages/smritidb-ffi/Cargo.toml
packages/smritidb-ffi/Cargo.lock
packages/smritidb-ffi/src/lib.rs
packages/smritidb-ffi/src/smritidb.udl
packages/smritidb-ffi/.cargo/config.toml             (new)
packages/smritidb-ffi/tests/persistence.rs           (new)
packages/smritidb-ffi/README.md

packages/smritidb-py/Cargo.toml
packages/smritidb-py/Cargo.lock
packages/smritidb-py/pyproject.toml
packages/smritidb-py/src/lib.rs
packages/smritidb-py/python/smritidb/__init__.py
packages/smritidb-py/python/smritidb/_native.pyi
packages/smritidb-py/tests/test_persistence.py       (new)
packages/smritidb-py/README.md

packages/core-ts/package.json
packages/core-ts/package-lock.json                   (new)
packages/core-ts/src/adapters/index.ts
packages/core-ts/src/adapters/sqlite.ts
packages/core-ts/src/adapters/sqlite.test.ts
packages/core-ts/src/kmf.cross-impl.test.ts          (new)
packages/core-ts/MIGRATION-TO-RUST-CORE.md           (new)
packages/core-ts/README.md

packages/smritidb-kmp/src/commonMain/kotlin/com/tanvrit/smritidb/PersistentStore.kt   (new)
packages/smritidb-kmp/src/commonTest/kotlin/com/tanvrit/smritidb/PersistentStoreTest.kt  (new)
packages/smritidb-kmp/src/jvmMain/kotlin/com/tanvrit/smritidb/SmritidbJvm.kt
packages/smritidb-kmp/src/jvmMain/kotlin/com/tanvrit/smritidb/PersistentStoreJvm.kt    (new)
packages/smritidb-kmp/src/jvmMain/kotlin/uniffi/smritidb/smritidb.kt                  (new, generated)
packages/smritidb-kmp/build.gradle.kts
packages/smritidb-kmp/gradle.properties               (new)
packages/smritidb-kmp/README.md
```

**Suggested message:**

> `bindings: Mission 2 Phase B — persistence across Python, TS, FFI, KMP/JVM`
>
> Lifts the Phase A PersistentStore + SqliteAdapter through every binding's
> native lift/lower path: PyO3 (Python), wasm-bindgen + better-sqlite3
> (TypeScript), UniFFI (smritidb-ffi -> KMP/JVM). Adds binding-side
> persistence test suites and the cross-impl KMF parity gate
> (tests/conformance/kmf_fixture.bin written by the Rust core, parsed by
> the pure-TS KMF reader).

---

## Group 3 — Mission 2 Phase B: stable C ABI + Go + Dart + .NET scaffolds

A separate, narrower commit because it introduces three new language
package directories that share a common C ABI.

**Files:**

```
packages/smritidb-c/                                  (entire new package)
  Cargo.toml, Cargo.lock, cbindgen.toml,
  include/smritidb.h, src/lib.rs, tests/smoke.c,
  .gitignore, README.md

packages/smritidb-go/                                 (entire new package)
  go.mod, smritidb.go, smritidb_test.go, README.md

packages/smritidb-dart/                               (entire new package)
  pubspec.yaml, lib/smritidb.dart, lib/src/{bindings,ffi,library,spec}.dart,
  example/round_trip.dart, test/smritidb_test.dart,
  .gitignore, README.md

packages/smritidb-dotnet/                             (entire new package)
  Smritidb.sln, src/Smritidb/{Smritidb.csproj, Store.cs,
  NativeMethods.cs, NativeLibraryResolver.cs},
  tests/Smritidb.Tests/{Smritidb.Tests.csproj, StoreTests.cs},
  .gitignore, README.md
```

**Suggested message:**

> `bindings: add stable C ABI + Go / Dart / .NET source-only scaffolds`
>
> Introduces packages/smritidb-c — a deliberately narrow extern "C"
> surface over smritidb-core (open_memory / open_sqlite / put / recall /
> size / close). Adds three source-only language bindings on top of it:
> smritidb-go (cgo), smritidb-dart (dart:ffi), smritidb-dotnet (P/Invoke).
> All three share the same C ABI and the same on-disk SQLite schema —
> the bit-exactness contract still holds end-to-end.

---

## Group 4 — Mission 2 Phase B: KMP per-target actuals (Apple + Linux + AndroidNative + JS + WasmJs)

The Kotlin/Native and Kotlin/JS implementations of the `expect`
declarations from Group 2's `commonMain`.

**Files:**

```
packages/smritidb-kmp/src/ffiNativeMain/kotlin/com/tanvrit/smritidb/PersistentStoreNative.kt   (new)
packages/smritidb-kmp/src/ffiNativeMain/kotlin/com/tanvrit/smritidb/SmritidbNative.kt          (new)
packages/smritidb-kmp/src/ffiNativeMain/kotlin/com/tanvrit/smritidb/UniffiBridge.kt            (new)
packages/smritidb-kmp/src/ffiNativeTest/kotlin/com/tanvrit/smritidb/PersistentStoreAppleTest.kt (new)
packages/smritidb-kmp/src/nativeInterop/cinterop/smritidb_ffi.def                              (new)
packages/smritidb-kmp/src/linuxTest/kotlin/com/tanvrit/smritidb/PersistentStoreLinuxTest.kt    (new)
packages/smritidb-kmp/src/androidNativeTest/kotlin/com/tanvrit/smritidb/PersistentStoreAndroidTest.kt (new)

packages/smritidb-kmp/src/jsMain/kotlin/com/tanvrit/smritidb/SmritidbCoreJs.kt                 (new)
packages/smritidb-kmp/src/jsMain/kotlin/com/tanvrit/smritidb/SmritidbJs.kt                     (new)
packages/smritidb-kmp/src/jsMain/kotlin/com/tanvrit/smritidb/PersistentStoreJs.kt              (new)
packages/smritidb-kmp/src/jsTest/kotlin/com/tanvrit/smritidb/PersistentStoreJsTest.kt          (new)

packages/smritidb-kmp/src/wasmJsMain/kotlin/com/tanvrit/smritidb/SmritidbWasmJs.kt             (new)
packages/smritidb-kmp/src/wasmJsMain/kotlin/com/tanvrit/smritidb/PersistentStoreWasmJs.kt      (new)
packages/smritidb-kmp/src/wasmJsTest/kotlin/com/tanvrit/smritidb/Bootstrap.kt                  (new)
packages/smritidb-kmp/src/wasmJsTest/kotlin/com/tanvrit/smritidb/PersistentStoreWasmJsTest.kt  (new)

packages/smritidb-kmp/kotlin-js-store/                                                         (npm lockfile dir, ignored if added to .gitignore — currently tracked)
```

Note: `kotlin-js-store/` is in `.gitignore` (the new version). Verify
the tree no longer lists it as untracked before committing this group.

**Suggested message:**

> `kmp: per-target actuals — Apple, Linux, AndroidNative, JS, WasmJs`
>
> Implements the expect declarations from Group 2's commonMain across
> Kotlin/Native (Apple + Linux + Android NDK toolchains, all sharing
> ffiNativeMain over the UniFFI C ABI in smritidb_ffi.def) and Kotlin/JS
> (wasm-bindgen + @JsModule("smritidb-core")). Kotlin/Wasm compiles and
> links cleanly; runtime tests gated on Node 22.1+ per the gradle pin.

---

## Group 5 — Mission 2 Phase D: browser persistence (IndexedDB adapter)

The Kotlin/JS-only `IndexedDbAdapter` plus the fake-IndexedDB harness
that makes it testable under `jsNodeTest`. Shipped after Group 4 because
it sits inside the `jsMain` source set.

**Files:**

```
packages/smritidb-kmp/src/jsMain/kotlin/com/tanvrit/smritidb/IndexedDbAdapter.kt  (new)
packages/smritidb-kmp/src/jsTest/kotlin/com/tanvrit/smritidb/IndexedDbAdapterTest.kt  (new)
packages/smritidb-kmp/src/jsTest/kotlin/com/tanvrit/smritidb/FakeIndexedDb.kt        (new)
```

**Suggested message:**

> `kmp: Phase D — IndexedDB persistence adapter for Kotlin/JS`
>
> Adds PersistentStore.openIndexedDb(dbName) on the Kotlin/JS target.
> Hosts the wasm memory store inside the page and snapshots a KMF blob
> to IndexedDB on every persistAsync() / closeAsync(). Reopening reads
> the snapshot back via storeRestoreFromSnapshot across the wasm
> boundary. fake-indexeddb wired as a jsTest-only dev dep so the
> jsNodeTest task exercises the adapter under Node.

---

## Group 6 — Cross-language test infrastructure

Conformance corpus extensions, KMF fixture emission, and the conformance
README errata.

**Files:**

```
tests/conformance/kmf_fixture.bin                 (new — Rust-emitted golden bytes)
tests/conformance/kmf_fixture.json                (new — human-readable mirror)
tests/conformance/emit_kmf_fixture.mjs            (new — TS-side emitter)
tests/conformance/README.md
```

**Suggested message:**

> `conformance: add KMF binary + JSON fixtures and TS-side emitter`
>
> Locks the bit-exact KMF wire contract: kmf_fixture.bin is emitted by
> the Rust core, parsed identically by the pure-TS KMF reader, and
> consumed by every binding's kmf_cross_impl test. The conformance.yml
> CI workflow gates kmf_fixture.bin under cmp -s strict.

---

## Group 7 — CI workflows

The existing `ci.yml` modification plus two new workflows.

**Files:**

```
.github/workflows/ci.yml                          (modified — added per-binding jobs)
.github/workflows/conformance.yml                 (new)
.github/workflows/release.yml                     (new — scaffold; publish steps commented)
CI.md                                             (new — workflow architecture doc)
```

**Suggested message:**

> `ci: per-binding jobs, conformance gate, release scaffold`
>
> Splits ci.yml into per-binding jobs so they run in parallel. Adds
> conformance.yml — the single required branch-protection check that
> asserts bit-exact KMF + corpus byte-identity across every binding.
> Adds release.yml as a scaffold for tagged builds; publish steps are
> commented pending registry secrets. Documents the workflow shape in
> CI.md.

---

## Group 8 — Examples modernization

The three runnable examples plus the umbrella `examples/README.md`.

**Files:**

```
examples/README.md                                                     (new)

examples/agent-memory/README.md
examples/agent-memory/package.json
examples/agent-memory/demo.ts
examples/agent-memory/src/AgentMemory.ts
examples/agent-memory/python_companion.py                              (new)

examples/semantic-notebook/README.md
examples/semantic-notebook/package.json
examples/semantic-notebook/index.ts
examples/semantic-notebook/persistent.ts

examples/polyglot-interop/                                             (entire new directory)
  README.md, KNOWN-ISSUES.md, run-all.sh,
  package.json, package-lock.json,
  step1_python_write.py,
  step2_rust_read/{Cargo.toml, Cargo.lock, src/main.rs},
  step3_kotlin_read/{build.gradle.kts, settings.gradle.kts, src/main/kotlin/Main.kt},
  step4_ts_read.ts
```

**Suggested message:**

> `examples: persistence + polyglot interop demos`
>
> Modernizes agent-memory and semantic-notebook against the Phase B
> PersistentStore. Adds the polyglot-interop demo: Python writes a
> SQLite file, then Rust, Kotlin/JVM, and TypeScript each open the same
> file and verify byte-identity through the KMF schema. Adds a Python
> companion to agent-memory that reads the file an agent run wrote.

---

## Group 9 — SPEC errata

**Files:**

```
SPEC.md
```

**Suggested message:**

> `spec: errata — clarify tiebreaker domain string, KMF section, dimension floor`

(Adjust the message after reviewing `git diff SPEC.md` — the change is
relatively small and the message should mirror the actual edits.)

---

## Group 10 — Patent filing package

The entire 18-file `patent/` directory. **Do NOT modify; commit as-is.**

**Files:**

```
patent/00-cover.md  →  patent/13-filing-checklist.md
patent/AGENT-ENGAGEMENT-LETTER.md
patent/FORM-2-COMPLETE-SPECIFICATION.md
patent/FORM-2-FILING-PROCEDURE.md
patent/FORM-2-PAGE-AUDIT.md
patent/REFINEMENT-LOG.md
patent/critiques/                                  (entire subtree — p0 + round-2 critiques)
```

**Suggested message:**

> `patent: filing-ready Form-2 complete specification + critique rounds`
>
> Lands the first patent filing package: 13 numbered sections of the
> specification (00-cover through 13-filing-checklist), the engagement
> letter of record, the consolidated FORM-2 complete specification
> with page audit and filing procedure, and two rounds of internal
> critique (p0 prior-art search results + round-2 per-section
> critiques). Apache-2.0 patent grant continues to apply to all
> published implementations.

---

## Group 11 — Documentation

Long-form guides + the regen script for auto-generated API docs.
Note: the `docs/api/**/*` HTML output is **gitignored** (see
`.gitignore`); only `docs/api/.gitkeep` files are tracked so the
regen script has somewhere to write.

**Files:**

```
docs/index.md                                     (new)
docs/CONTRIBUTING-API-DOCS.md                     (new)
docs/regen.sh                                     (new)
docs/dokka-init.gradle.kts                        (new)
docs/guides/getting-started.md                    (new)
docs/guides/persistence.md                        (new)
docs/guides/cross-language-interop.md             (new)
docs/guides/conformance-corpus.md                 (new)
docs/api/.gitkeep                                 (new — placeholder)
docs/api/rust/.gitkeep                            (new — placeholder)
docs/api/python/.gitkeep                          (new — placeholder)
docs/api/kotlin/.gitkeep                          (new — placeholder)
docs/api/typescript/.gitkeep                      (new — placeholder)
```

**Suggested message:**

> `docs: long-form guides + API regeneration toolchain`
>
> Adds the docs/index.md entry point, four how-to guides
> (getting-started, persistence, cross-language-interop,
> conformance-corpus), CONTRIBUTING-API-DOCS.md with regeneration
> steps per binding, and docs/regen.sh which drives rustdoc + pdoc +
> dokka + typedoc end-to-end. The generated docs/api/<lang>/ output
> is gitignored; .gitkeep placeholders keep the four sub-directory
> shells around for the regen script.

---

## Group 12 — Benchmarks

Cross-binding perf harness — pending the perf-benchmark agent.

**Files:**

```
benchmarks/                                       (entire new directory)
  README.md, WORKLOAD.md, run-all.sh, package.json,
  rust/{Cargo.toml, Cargo.lock, src/main.rs, benches/{primitives,recall,persistence}.rs},
  python/{bench_primitives.py, bench_recall.py, bench_persistence.py, run_all.py},
  kotlin/{settings.gradle.kts, build.gradle.kts, src/main/kotlin/com/tanvrit/smritidb/bench/Benchmarks.kt},
  typescript/{tsconfig.json, package.json, bench-utils.ts, bench-primitives.ts, bench-recall.ts, bench-persistence.ts, run-all.ts},
  scripts/render_results.py,
  results/{rust,python,kotlin,typescript}.json
```

**Suggested message:**

> `benchmarks: cross-binding perf harness (Rust / Python / Kotlin / TS)`
>
> Adds the benchmark suite tracking primitives throughput, recall
> latency (p50/p99/p999 at N ∈ {100, 1000, 10000}), persistence MB/s,
> and bytes/item RSS. WORKLOAD.md is the shared spec every binding
> implements identically. results/ ships the first reference run so
> regressions are detectable.

---

## Group 13 — Root-level glue

The small set of root files that fit nowhere else but are needed to
make the whole tree resolve. Commit last so groups above land first.

**Files:**

```
.gitignore                                        (modified — adds target/, build/, criterion/, *.db, docs/api/**, etc.)
README.md                                         (modified — polished status, architecture diagram)
package.json                                      (modified — workspace scripts)
pnpm-lock.yaml                                    (modified — new workspace deps)
pnpm-workspace.yaml                               (modified — new package globs)
CHANGELOG.md                                      (new — if intended for tracking; otherwise gitignore)
COMMIT-PLAN.md                                    (this file — gitignore or commit alongside)
```

**Suggested message:**

> `repo: gitignore audit, README polish, workspace deps refresh`
>
> Updates the gitignore to cover every language stack now in the tree
> (Rust target/, Gradle .gradle/, Kotlin kotlin-js-store/, Python
> .pytest_cache/, etc.) plus the docs/api/**/* auto-generated output.
> Rewrites the root README to reflect 6 production-ready bindings,
> 9/11 KMP targets, 234+ passing tests, the polyglot interop demo,
> and the patent filing-ready package. Refreshes pnpm workspace globs
> for the new package directories.

(Whether to commit `COMMIT-PLAN.md` itself is a judgement call —
leaving it out keeps the history cleaner; committing it documents the
Mission 2 staging plan for posterity.)

---

## Suggested commit order

1. Patent (10) — independent, no code dependencies, can land first or last.
2. Rust core persistence (1).
3. Conformance fixtures (6).
4. CI workflows (7) — so the conformance gate is live before the bindings catch up.
5. Bindings + KMP common (2).
6. C ABI + Go/Dart/.NET (3).
7. KMP per-target actuals (4).
8. KMP IndexedDB adapter (5).
9. Examples (8).
10. Documentation (11).
11. Benchmarks (12).
12. SPEC errata (9).
13. Root glue (13).

Each commit is independently `cargo test` / `pytest` / `gradle test` /
`pnpm test` green at its own slice. The conformance gate becomes live
once Group 7 lands; from Group 1 onward, every commit either keeps it
green or is itself the only commit that touches `kmf_fixture.bin`.
