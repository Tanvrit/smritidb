# smritidb-kmp

Kotlin Multiplatform wrapper for [Smritidb](https://smritidb.com).

Provides a single Kotlin API consumable from Android, JVM, iOS, macOS, Linux, JS, and WasmJs. Backed by:

- **JVM / Android Native / Linux**: the UniFFI-generated Kotlin (`packages/smritidb-ffi/bindings/kotlin/`) loading `libsmritidb_ffi` at runtime via JNA.
- **Apple targets** (`iosArm64`, `iosX64`, `iosSimulatorArm64`, `macosArm64`, `macosX64`): Cinterop against the static lib `libsmritidb_ffi.a`.
- **JS / WasmJs**: the WebAssembly build from `packages/core-rs/pkg/`.

Same bit-exact semantics as the TypeScript, Python, and Rust references — a KMF snapshot written by any binding reads back identically in any other.

## Targets

```
jvm
androidNativeArm64, androidNativeX64
iosArm64, iosX64, iosSimulatorArm64
macosArm64, macosX64
linuxX64, linuxArm64
js (browser + nodejs)
wasmJs (browser + nodejs)
```

## Build

This package is a Gradle module. It assumes:

1. The Rust toolchain is installed (see `packages/core-rs/README.md`).
2. The UniFFI bindings have been generated: `cargo run --release --bin uniffi-bindgen -- generate src/smritidb.udl --language kotlin --out-dir bindings/kotlin` from `packages/smritidb-ffi/`.
3. The native lib has been built for each target — for JVM dev, `cargo build --release` is enough; for iOS, build for `aarch64-apple-ios` + simulator triples and wrap into `libsmritidb_ffi.xcframework`.

```bash
cd packages/smritidb-kmp
./gradlew build           # all targets
./gradlew jvmTest         # JVM-only quick test
./gradlew publishToMavenLocal  # for local consumption
```

## Quick example

```kotlin
import com.tanvrit.smritidb.*

fun main() {
    val store = openStore(dimension = 10_000u)
    store.put(Cue.Text("the cat sat on the mat"), "the cat sat on the mat".encodeToByteArray())

    val hits = store.recall(Cue.Text("cat on mat"), topK = 5u, minSimilarity = 0.0)
    hits.forEach { println("${it.value.decodeToString()} -> ${it.similarity}") }
}
```

## Status

- Common API surface defined in `src/commonMain` — `expect` declarations for all primitives + the `SmritidbStore` interface.
- JVM `actual` complete (`src/jvmMain`) — delegates to UniFFI.
- Other targets currently use the JVM `actual` pattern as the reference; per-target `actual` implementations (especially for Apple and JS/Wasm) ship in the next checkpoint.
- Common tests use `kotlin.test` and run on each target's runtime; passing on JVM today.

## License

Apache-2.0.
