# Smritidb Documentation

Smritidb is an open-source binary-hyperdimensional associative memory system
with bit-exact cross-language implementations. The same SQLite file written
by the Python binding can be opened by the Rust core, the Kotlin/JVM
wrapper, and the TypeScript reference — every implementation produces
byte-identical hypervectors and byte-identical KMF wire snapshots.

- [Manifesto](MANIFESTO.md) — project vision and design philosophy
- [Getting Started](guides/getting-started.md) — installation and hello-world
  per binding
- [Cross-Language Interop](guides/cross-language-interop.md) — how the polyglot
  demo works
- [Persistence Architecture](guides/persistence.md) — adapters, schema, browser story
- [Conformance Corpus](guides/conformance-corpus.md) — the bit-exactness contract

## API Reference

Auto-generated from source. Regeneration steps are in
[CONTRIBUTING-API-DOCS.md](CONTRIBUTING-API-DOCS.md).

- [Rust core (`smritidb-core`)](api/rust/doc/smritidb_core/index.html) —
  hypervector primitives, in-memory `Store`, `PersistentStore`, KMF serializer,
  persistence trait, native + WebAssembly.
- [Rust FFI (`smritidb-ffi`)](api/rust/doc/smritidb_ffi/index.html) — UniFFI
  surface that generates the Kotlin and Swift bindings.
- [Rust C ABI (`smritidb-c`)](api/rust/doc/smritidb_c/index.html) — stable
  `extern "C"` surface consumed by Go, Dart, and .NET. The raw header is
  [`packages/smritidb-c/include/smritidb.h`](../packages/smritidb-c/include/smritidb.h).
- [Python (`smritidb-py`)](api/python/smritidb.html) — PyO3 bindings over the
  Rust core. `Store`, `PersistentStore`, hypervector primitives.
- [Kotlin (`smritidb-kmp`)](api/kotlin/index.html) — Kotlin Multiplatform
  wrapper. JVM target via UniFFI/JNA; Apple, Android Native, and Linux targets
  via Cinterop; JS/Wasm targets via wasm-bindgen.
- [TypeScript (`core-ts`)](api/typescript/index.html) — pure-TS reference
  implementation. The source of truth for the conformance corpus.

## Repository layout

```text
packages/
  core-rs/         Rust core (native + wasm).
  core-ts/         TypeScript reference implementation.
  smritidb-c/      Stable C ABI over the Rust core.
  smritidb-ffi/    UniFFI surface for Kotlin / Swift.
  smritidb-py/     Python bindings (PyO3).
  smritidb-kmp/    Kotlin Multiplatform wrapper.
  smritidb-go/     Go binding over the C ABI.
  smritidb-dart/   Dart binding over the C ABI.
  smritidb-dotnet/ .NET binding over the C ABI.
examples/
  polyglot-interop/  Python writes -> Rust/Kotlin/TS read the same file.
  agent-memory/      Small agent loop backed by smritidb.
  semantic-notebook/ Jupyter notebook tour.
tests/conformance/   The bit-exact corpus + KMF fixture.
```
