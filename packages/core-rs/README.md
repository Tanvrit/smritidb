# smritidb-core (Rust)

Native + WebAssembly core for [Smritidb](https://smritidb.com) — the open biology-inspired associative memory standard.

Implements the same binary HDC primitives as the TypeScript reference (`packages/core-ts`), bit-exactly. Designed to be dropped in behind the published `smritidb` npm package as a transparent 10–100× speedup on the hot paths.

## Build

```bash
# Native
cargo build --release

# WebAssembly (for browser embedding)
wasm-pack build --target web --features wasm

# Tests
cargo test

# Benchmarks
cargo bench
```

## API

Same operations as the TypeScript reference, mapped 1:1:

| Rust                                   | TypeScript                          |
|----------------------------------------|-------------------------------------|
| `random_hv(seed, dim) -> Hypervector`  | `randomHv(seed, dim)`               |
| `similarity(a, b) -> f64`              | `similarity(a, b)`                  |
| `bind(a, b) -> Hypervector`            | `bind(a, b)`                        |
| `bundle(&[&Hypervector]) -> Hypervector` | `bundle(hvs)`                     |
| `permute(hv, k) -> Hypervector`        | `permute(hv, k)`                    |
| `encode_string(s, dim) -> Hypervector` | `encodeString(s, dim)`              |
| `encode_embedding(v, dim) -> Hypervector` | `encodeEmbedding(v, dim)`        |

All operations are deterministic and bit-exact across both implementations — a snapshot written by one reads identically in the other.

## License

Apache-2.0. See the repo root [`LICENSE`](../../LICENSE) and [`NOTICE`](../../NOTICE).
