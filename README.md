# Kanerva

> Storage that remembers like you do.

**Kanerva** is a biology-inspired storage layer for every platform. It treats *meaning* as a first-class addressing primitive — write data, recall it by partial cue, watch it degrade gracefully when the substrate shrinks, and let frequently-co-accessed items consolidate into closer associations on their own.

Named after [Pentti Kanerva](https://en.wikipedia.org/wiki/Pentti_Kanerva), who introduced *Sparse Distributed Memory* in 1988. The math is his; the cross-platform productization is ours.

---

## Status

**Phase 0 — spec & math validation.** No production code yet. Nothing is published. APIs in this README are aspirational and reflect the design contract being written into [`SPEC.md`](SPEC.md).

If you want to follow along: star the repo, read the [manifesto](docs/MANIFESTO.md), or open an issue with use-cases we should design for.

---

## The three load-bearing properties

| Property | Inspired by | What it means in practice |
|---|---|---|
| **Fuzzy content-addressing** | The brain's cue-based recall — a smell, a glimpse, and the whole memory comes back. | Look up data by *similarity*, not by exact hash. Partial cues, near matches, semantic queries — all native. |
| **Holographic distribution** | Cortical memory — each item spread across many synapses, no single "address." | Lose a chunk of the substrate, lose *no specific item*. Everything just gets a little fuzzier. Degrades like a hologram, not like a disk. |
| **Hebbian consolidation** | Hippocampus → cortex transfer during sleep. | Frequently co-accessed items get bound closer. Cold items summarize. The index reshapes itself based on how you actually use it. |

The math substrate is **Hyperdimensional Computing** / **Sparse Distributed Memory** — a 38-year-old, well-validated academic foundation. The contribution here is *productizing* it as a polished, drop-in storage library across JS, Rust, Python, Kotlin Multiplatform, and Swift.

---

## What Kanerva is, and isn't

✅ **Is**: a semantic memory layer for apps — LLM long-term memory, fuzzy caches, "find similar" search, agent state.

❌ **Isn't**: a transactional database, a blob store, a vector-DB clone, or a replacement for your existing exact-byte storage. It complements them.

---

## Sketch of the API (subject to Phase 0 lock)

```ts
import { Kanerva } from "kanerva";

const store = new Kanerva({ dimension: 10000, backend: "memory" });

await store.put("the cat sat on the mat", { tags: ["sentence"] });

const hits = await store.recall("cat on mat", { topK: 5 });
// → [{ value: "the cat sat on the mat", similarity: 0.91 }, ...]

const composed = store.bind("ROLE:subject", "FILLER:cat");
await store.consolidate();
```

Same API surface across:

- `npm i kanerva` — Node + browser (Wasm)
- `cargo add kanerva` — Rust
- `pip install kanerva` — Python
- Maven Central — Kotlin Multiplatform (JVM/Android/iOS/JS/Native)
- SwiftPM — Swift

---

## Repo layout

```
kanerva/
├── SPEC.md                The contract every binding implements
├── notebooks/             Phase 0 math validation (Python + numpy)
├── packages/
│   ├── core-ts/           TypeScript reference implementation
│   ├── core-rs/           Rust core + Wasm + native bindings (Phase 2)
│   ├── kanerva-py/        Python bindings (Phase 3)
│   ├── kanerva-kmp/       Kotlin Multiplatform bindings (Phase 3)
│   └── kanerva-swift/     Swift bindings (Phase 3)
├── web/                   Next.js site — manifesto, docs, playground
├── examples/              Sample apps
├── benchmarks/            Reproducible benchmarks vs. vector DBs
└── docs/                  Long-form documentation
```

---

## Contributing

We're at the spec stage. The most valuable contributions right now:

1. Review [`SPEC.md`](SPEC.md) — point out anything the math doesn't support.
2. Propose use-cases — file an issue describing the kind of memory you wish you had.
3. Wait for Phase 1, then write code.

See [`CONTRIBUTING.md`](CONTRIBUTING.md) for the full process.

## License

Apache-2.0 — see [`LICENSE`](LICENSE) and [`NOTICE`](NOTICE). The Apache-2.0 patent grant matters here: implementations of an open associative-memory standard should travel with an explicit, irrevocable patent license from contributors.

## Citation

If Kanerva ends up in your research, please cite both the underlying SDM paper and this library. See [`CITATION.cff`](CITATION.cff).

---

*"Memory is not a thing you have. It is a thing you do." — and now, a thing you import.*
