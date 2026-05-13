# smritidb-ffi

UniFFI bindings for [Smritidb](https://smritidb.com) — generates Kotlin (Android / JVM) and Swift (iOS / macOS) surfaces over the Rust core, from a single declarative interface (`src/smritidb.udl`).

Same primitives as the TypeScript, Python, and pure Rust bindings, bit-exact across all implementations.

## What gets generated

```
bindings/
├── kotlin/
│   └── uniffi/smritidb/smritidb.kt        # 2200+ lines, full API
└── swift/
    ├── smritidb.swift                     # 1200+ lines, full API
    ├── smritidbFFI.h                      # C header
    └── smritidbFFI.modulemap              # Swift module map
```

## Build

```bash
# Build the cdylib + uniffi-bindgen binary
cargo build --release

# Generate Kotlin
cargo run --release --bin uniffi-bindgen -- \
    generate src/smritidb.udl --language kotlin --out-dir bindings/kotlin

# Generate Swift
cargo run --release --bin uniffi-bindgen -- \
    generate src/smritidb.udl --language swift --out-dir bindings/swift
```

Native library artifacts produced under `target/release/`:

| Platform | Artifact |
|---|---|
| macOS (current host) | `target/release/libsmritidb_ffi.dylib` |
| iOS / iOS Sim / catalyst | needs `cargo build --target <triple>` per platform |
| Linux x86_64 | `target/release/libsmritidb_ffi.so` |
| Android x86_64 / arm64 | needs `cargo-ndk` + `cargo build --target aarch64-linux-android` |
| Windows | `target/release/smritidb_ffi.dll` |

## Kotlin quickstart

```kotlin
import uniffi.smritidb.*

fun main() {
    val store = Store(dimension = 10000u)
    store.put(key = "the cat sat on the mat", value = "the cat sat on the mat".toByteArray(), tags = listOf("sentence"))

    val hits = store.recall(cue = "cat on mat", topK = 5u, minSimilarity = 0.0)
    hits.forEach {
        println("${String(it.value)}  ->  ${"%.4f".format(it.similarity)}")
    }
}
```

The Kotlin output loads `libsmritidb_ffi` via JNA at runtime; place it where the JVM can find it (e.g. `java.library.path` or alongside the jar on Android).

## Swift quickstart

```swift
import smritidb

let store = try Store(dimension: 10_000)
let _ = try store.put(
    key: "the cat sat on the mat",
    value: Data("the cat sat on the mat".utf8),
    tags: ["sentence"]
)

let hits = try store.recall(cue: "cat on mat", topK: 5, minSimilarity: 0.0)
for h in hits {
    print("\(String(decoding: h.value, as: UTF8.self))  ->  \(String(format: "%.4f", h.similarity))")
}
```

For an iOS app, build the static lib for `aarch64-apple-ios` + `aarch64-apple-ios-sim` + `x86_64-apple-ios-sim`, wrap into an `xcframework`, and pair with the generated `.swift` + `.modulemap`.

## Kotlin Multiplatform (Phase 3.5)

A pure-KMP wrapper consuming the UniFFI Kotlin output across Android, JVM, iOS, and JS targets lives in `packages/smritidb-kmp/` (scaffolded; full KMP packaging is a follow-up).

## Spec compliance

Every binding round-trips the [Phase 0 conformance properties](../../notebooks/phase0_hdc_validation.ipynb):
- `random_hv(seed, dim)` is deterministic
- `similarity(a, a) == 1.0`; random-pair similarity ≈ 0.5
- `bind` is self-inverse: `bind(bind(a, b), b) == a`
- Bundle preserves similarity to each component
- Compositional role/filler recovery works through the cleanup memory

## License

Apache-2.0. See the repo root [`LICENSE`](../../LICENSE) and [`NOTICE`](../../NOTICE).
