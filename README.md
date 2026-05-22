# Smritidb

> Storage that remembers like you do.

**Smritidb** is a biology-inspired storage layer for every platform. It treats *meaning* as a first-class addressing primitive — write data, recall it by partial cue, watch it degrade gracefully when the substrate shrinks, and let frequently-co-accessed items consolidate into closer associations on their own.

Named after [Pentti Kanerva](https://en.wikipedia.org/wiki/Pentti_Kanerva), who introduced *Sparse Distributed Memory* in 1988. The math is his; the cross-platform productization is ours.

---

## Status

**Production-ready bit-exact core across nine language stacks.** A single Rust core (`packages/core-rs`) drives every binding through one of four lift/lower paths (PyO3, UniFFI, wasm-bindgen, stable C ABI). Every binding round-trips the same conformance corpus and the same on-disk KMF snapshot — a file written by Python opens byte-identically in Rust, Kotlin, and TypeScript.

| Stack                  | Status                  | Distribution                          | Tests passing |
|------------------------|-------------------------|---------------------------------------|---------------|
| Rust (`smritidb-core`) | Production-ready        | `cargo add smritidb-core`             | 50            |
| Python (`smritidb`)    | Production-ready        | `pip install smritidb` (PyO3 wheels)  | 21            |
| Kotlin/JVM (KMP)       | Production-ready        | Maven Central (planned)               | 12            |
| Kotlin/Native Apple    | Production-ready        | KMP `iosArm64`, `iosX64`, `iosSimulatorArm64`, `macosArm64`, `macosX64` | 14 |
| Kotlin/Native Linux    | Production-ready        | KMP `linuxX64`, `linuxArm64`          | 15 + 15       |
| Kotlin/JS              | Production-ready        | KMP `js` + browser IndexedDB adapter  | 22            |
| TypeScript (`core-ts`) | Production-ready        | `npm i @tanvrit/smritidb`             | 73            |
| Go (`smritidb-go`)     | Source-only (cgo / C ABI) | `go get` from repo                  | scaffolded    |
| Dart (`smritidb-dart`) | Source-only (dart:ffi / C ABI) | `dart pub get` from repo        | scaffolded    |
| .NET (`smritidb-dotnet`) | Source-only (P/Invoke / C ABI) | `dotnet add` from repo        | scaffolded    |

**Aggregate: 234+ tests passing** across the production-ready stacks. **9/11 KMP platform targets** are green end-to-end (JVM, macosArm64/X64, iosArm64/X64/SimulatorArm64, linuxX64/Arm64, jsNode/Browser). The remaining two — `androidNativeArm64/X64` and `wasmJsNode/Browser` — compile + link cleanly; running their test binaries needs an Android emulator / Node 22.1+ respectively, and is wired but disabled in CI.

---

## Quick start

```bash
# Pick a binding
pip install smritidb                    # Python
cargo add smritidb-core                 # Rust
npm install @tanvrit/smritidb           # TypeScript / Node
# Kotlin: gradle dependency on com.tanvrit.smritidb:smritidb-kmp (Maven Central, planned)
```

```python
from smritidb import PersistentStore

# Open or create a SQLite-backed associative store. The same file can be
# opened from Rust, Kotlin, and TypeScript and reads back byte-identically.
store = PersistentStore.open_sqlite("/tmp/notes.db", dimension=10000)

store.put("the cat sat on the mat", b"the cat sat on the mat")
store.put("a bird in the hand",     b"a bird in the hand")

# Fuzzy recall — top-K by cosine-like similarity over hyperdimensional encodings.
for hit in store.recall("cat on mat", top_k=3, min_similarity=0.0):
    print(hit["value"].decode(), "->", round(hit["similarity"], 4))

store.persist()      # snapshot to SQLite
store.close()
```

End-to-end polyglot demo: a Python process writes a SQLite file, then a Rust process and a Kotlin/JVM process and a TypeScript process each open the same file and verify byte-identity through the KMF schema. See [`examples/polyglot-interop/`](examples/polyglot-interop/).

---

## Architecture

```
                       +-------------------------+
                       |   SPEC.md  +  conformance
                       |     corpus + KMF fixture
                       +-----------+-------------+
                                   |
                                   v
                       +-------------------------+
                       |   Rust core (core-rs)   |    bit-exact reference
                       |   primitives, store,    |
                       |   persistence, KMF,     |
                       |   wasm-bindgen, sqlite  |
                       +-+----+----+----+----+---+
                         |    |    |    |    |
                  PyO3   |    |    |    |    | wasm-bindgen
                +--------+    |    |    |    +-----------+
                v             |    |    |                v
        +---------------+     |    |    |       +---------------+
        | smritidb-py   |     |    |    |       |  core-ts      |
        | (Python pkg)  |     |    |    |       |  (TS ref +    |
        +---------------+     |    |    |       |   SQLite +    |
                              |    |    |       |   pure-TS KMF)|
                       UniFFI |    |    |       +---------------+
                +-------------+    |    |
                v                  |    |
        +-----------------+        |    |   stable C ABI (smritidb-c)
        | smritidb-ffi    |        |    +------------------+
        | -> KMP wrapper  |        |                       |
        |   (smritidb-kmp)|        |   +-------------+ +-------------+ +-------------+
        |   JVM + Native  |        |   | smritidb-go | | smritidb-   | | smritidb-   |
        |   (Apple, Linux,|        |   | (cgo)       | | dart        | | dotnet      |
        |    AndroidNat.) |        |   |             | | (dart:ffi)  | | (P/Invoke)  |
        +-----------------+        |   +-------------+ +-------------+ +-------------+
                                   |
                            Kotlin/JS, Kotlin/Wasm
                                   |
                                   v
                       +-----------------------+
                       | smritidb-kmp JS       |
                       | (wasm-bindgen +       |
                       |  IndexedDB adapter)   |
                       +-----------------------+
```

Every arrow lifts the same `smritidb-core` symbols. The KMF wire format, the SQLite schema (`smritidb_snapshot` + `smritidb_wal`), the conformance corpus, and the BLAKE3-keyed tiebreaker are the four invariants every binding satisfies.

---

## The three load-bearing properties

| Property | Inspired by | What it means in practice |
|---|---|---|
| **Fuzzy content-addressing** | The brain's cue-based recall — a smell, a glimpse, and the whole memory comes back. | Look up data by *similarity*, not by exact hash. Partial cues, near matches, semantic queries — all native. |
| **Holographic distribution** | Cortical memory — each item spread across many synapses, no single "address." | Lose a chunk of the substrate, lose *no specific item*. Everything just gets a little fuzzier. Degrades like a hologram, not like a disk. |
| **Hebbian consolidation** | Hippocampus → cortex transfer during sleep. | Frequently co-accessed items get bound closer. Cold items summarize. The index reshapes itself based on how you actually use it. |

The math substrate is **binary hyperdimensional computing** / **Sparse Distributed Memory** — a 38-year-old, well-validated academic foundation. The contribution here is *productizing* it as a polished, drop-in storage library across JS, Rust, Python, Kotlin Multiplatform, Swift, Go, Dart, and .NET, with a stable KMF wire format and a cross-binding conformance gate that guarantees byte-identity end-to-end.

---

## What Smritidb is, and isn't

✅ **Is**: a semantic memory layer for apps — LLM long-term memory, fuzzy caches, "find similar" search, agent state. A *substrate*, not an SDK around someone else's substrate.

❌ **Isn't**: a transactional database, a blob store, a vector-DB clone, or a replacement for your existing exact-byte storage. It complements them.

**No third-party storage SDK policy.** Smritidb depends on local SQLite (bundled libsqlite3) and IndexedDB (browser-native). It does **not** wrap a hosted vector database, an external cache, or a third-party "AI memory" service. Every binding ships the same on-disk and in-memory representations end-to-end. This is intentional: the value of an open associative-memory standard collapses if half the stack is opaque.

---

## Repo layout

```
smritidb/
├── SPEC.md                      The contract every binding implements
├── notebooks/                   Phase 0 math validation (Python + numpy)
├── packages/
│   ├── core-rs/                 Rust core + Wasm + native bindings
│   ├── core-ts/                 TypeScript reference implementation
│   ├── smritidb-c/              Stable C ABI for Go / Dart / .NET
│   ├── smritidb-ffi/            UniFFI surface for Kotlin / Swift
│   ├── smritidb-py/             Python bindings (PyO3)
│   ├── smritidb-kmp/            Kotlin Multiplatform wrapper (JVM, Apple, Linux, AndroidNat, JS, WasmJs)
│   ├── smritidb-go/             Go binding (cgo over C ABI)
│   ├── smritidb-dart/           Dart / Flutter binding (dart:ffi over C ABI)
│   └── smritidb-dotnet/         .NET binding (P/Invoke over C ABI)
├── examples/
│   ├── agent-memory/            LLM long-term-memory adapter
│   ├── semantic-notebook/       Recall pipeline tour
│   └── polyglot-interop/        4-language end-to-end SQLite round-trip
├── tests/conformance/           Corpus + KMF fixture (the bit-exactness contract)
├── benchmarks/                  Cross-binding perf harness (Rust / Python / Kotlin / TS)
├── docs/                        Long-form documentation + API reference
│   ├── index.md
│   ├── guides/                  getting-started, persistence, interop, conformance
│   └── api/                     auto-generated rustdoc / pdoc / dokka / typedoc
├── patent/                      Filing-ready Form-2 complete specification
└── web/                         Next.js site — manifesto, docs, playground
```

---

## Documentation

- **[`SPEC.md`](SPEC.md)** — the wire-level contract every binding implements.
- **[`docs/index.md`](docs/index.md)** — entry point.
- **[`docs/guides/getting-started.md`](docs/guides/getting-started.md)** — install + hello-world per binding.
- **[`docs/guides/persistence.md`](docs/guides/persistence.md)** — adapter architecture, SQLite schema, browser story.
- **[`docs/guides/cross-language-interop.md`](docs/guides/cross-language-interop.md)** — how the polyglot demo works.
- **[`docs/guides/conformance-corpus.md`](docs/guides/conformance-corpus.md)** — the bit-exactness contract.
- **[`docs/api/`](docs/api/)** — auto-generated API reference per binding (rustdoc, pdoc, dokka, typedoc). Regeneration: [`docs/regen.sh`](docs/regen.sh).

---

## Patents

Smritidb's first patent application is **filing-ready** as a Form-2 Indian-Patent-Office complete specification (with 35 USC §112 enablement parity targeted for a US continuation). The full filing package — claims, drawings list, detailed description, abstract, prior-art analysis, and filing checklist — lives under [`patent/`](patent/).

Engagement of record: [`patent/AGENT-ENGAGEMENT-LETTER.md`](patent/AGENT-ENGAGEMENT-LETTER.md).

Apache-2.0 ships an irrevocable patent grant alongside the code; any patents that issue on inventions disclosed in this repository will be paired with that grant for the published implementations here.

---

## Contributing

1. Read [`SPEC.md`](SPEC.md). Disagreements between an implementation and the spec are implementation bugs.
2. Run the conformance suite for the binding you're touching (every binding has one — `cargo test --test conformance`, `pytest tests/`, `gradle :jvmTest`, `pnpm test`).
3. If your change shifts the KMF wire format, the `kmf_fixture.bin` in `tests/conformance/` must change in the same commit. The CI gate is `cmp -s` strict.

See [`CONTRIBUTING.md`](CONTRIBUTING.md) for the full process and [`CI.md`](CI.md) for the workflow architecture.

## License

Apache-2.0 — see [`LICENSE`](LICENSE) and [`NOTICE`](NOTICE). The Apache-2.0 patent grant matters here: implementations of an open associative-memory standard should travel with an explicit, irrevocable patent license from contributors.

## Citation

If Smritidb ends up in your research, please cite both the underlying SDM paper and this library. See [`CITATION.cff`](CITATION.cff).

---

*"Memory is not a thing you have. It is a thing you do." — and now, a thing you import.*
