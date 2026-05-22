# smritidb-core (Rust)

Native + WebAssembly core for [Smritidb](https://smritidb.com) — the open biology-inspired associative memory standard.

This crate is the bit-exact reference every other binding lifts. The TypeScript reference (`packages/core-ts`) verifies primitive byte-for-byte; the Python, Kotlin, Swift, Go, Dart, and .NET bindings each delegate into this crate through PyO3, UniFFI, wasm-bindgen, or the stable C ABI.

## What it is

- Binary hyperdimensional computing (HDC) primitives: `random_hv`, `bind`, `bundle`, `permute`, `similarity`, `encode_string`, `encode_embedding`.
- In-memory `Store` and durable `PersistentStore` with a `PersistenceAdapter` trait (in-memory + SQLite-bundled implementations).
- KMF (Kanerva / Smritidb Memory Format) serializer and deserializer.
- Hebbian consolidation pass.
- WebAssembly build behind the `wasm` feature, consumed by the Kotlin/JS and Kotlin/Wasm targets in `smritidb-kmp` plus the `@tanvrit/smritidb` browser bundle.

## Install

```toml
# Cargo.toml
[dependencies]
smritidb-core = "0.1"
```

Feature flags:

| Feature           | Default | What it does                                                                 |
|-------------------|---------|------------------------------------------------------------------------------|
| `persist-sqlite`  | on      | SQLite adapter via `rusqlite` (bundled libsqlite3, no system dep).           |
| `wasm`            | off     | Pulls in `wasm-bindgen`, `js-sys`, panic hook, `serde-wasm-bindgen`.         |

The wasm build automatically turns `persist-sqlite` off — `libsqlite3` doesn't compile to `wasm32-unknown-unknown`.

## Hello, world

```rust
use smritidb_core::{open_persistent_store, PersistenceAdapter, SqliteAdapter, Cue};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let adapter = SqliteAdapter::open("/tmp/notes.db", 10_000)?;
    let mut store = open_persistent_store(Box::new(adapter), 10_000)?;

    store.put(
        Cue::Text("the cat sat on the mat".into()),
        b"the cat sat on the mat".to_vec(),
        vec![],
    )?;

    let hits = store.recall(&Cue::Text("cat on mat".into()), 5, 0.0)?;
    for h in hits {
        println!("{}  ->  {:.4}", String::from_utf8_lossy(&h.value), h.similarity);
    }

    store.persist()?;
    store.close()?;
    Ok(())
}
```

## Build

```bash
# Native
cargo build --release

# WebAssembly (browser ESM)
wasm-pack build --target web    --release -- --no-default-features --features wasm

# WebAssembly (Node CJS)
wasm-pack build --target nodejs --release --out-dir pkg-node -- --no-default-features --features wasm

# Tests
cargo test --release           # 50 tests passing

# Cross-impl KMF + conformance gates (also part of conformance.yml)
cargo test --release --test conformance --test kmf_cross_impl

# Benchmarks
cargo bench
```

## API surface

| Rust                                     | TypeScript                          |
|------------------------------------------|-------------------------------------|
| `random_hv(seed, dim) -> Hypervector`    | `randomHv(seed, dim)`               |
| `similarity(a, b) -> f64`                | `similarity(a, b)`                  |
| `bind(a, b) -> Hypervector`              | `bind(a, b)`                        |
| `bundle(&[&Hypervector]) -> Hypervector` | `bundle(hvs)`                       |
| `permute(hv, k) -> Hypervector`          | `permute(hv, k)`                    |
| `encode_string(s, dim) -> Hypervector`   | `encodeString(s, dim)`              |
| `encode_embedding(v, dim) -> Hypervector`| `encodeEmbedding(v, dim)`           |
| `Store::new(dim)`                        | `new Smritidb({ dimension })`       |
| `open_persistent_store(adapter, dim)`    | `openPersistentStore({ adapter, dim })` |
| `kmf::write(...) / kmf::read(...)`       | `writeKmf(...) / readKmf(...)`      |

All operations are deterministic and bit-exact across both implementations — a snapshot written by one reads identically in the other. See [`tests/conformance.rs`](tests/conformance.rs) and [`tests/kmf_cross_impl.rs`](tests/kmf_cross_impl.rs).

## Docs

- API reference (rustdoc): [`docs/api/rust/doc/smritidb_core/`](../../docs/api/rust/doc/smritidb_core/index.html)
- Wire format: [`SPEC.md`](../../SPEC.md) §8 (KMF)
- Persistence architecture: [`docs/guides/persistence.md`](../../docs/guides/persistence.md)

## Tests

**50 tests passing** (in-crate unit + integration: primitives, encoders, store, persistence, KMF, consolidation, cross-impl conformance). Run with `cargo test --release`.

## License

Apache-2.0. See the repo root [`LICENSE`](../../LICENSE) and [`NOTICE`](../../NOTICE).
