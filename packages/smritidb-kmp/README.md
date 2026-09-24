# smritidb-kmp

Kotlin Multiplatform wrapper for [Smritidb](https://smritidb.com).

## What it is

A single Kotlin API for the Smritidb associative store, consumable from Android, JVM, iOS, macOS, Linux, Android Native, JS, and WasmJs. Bit-exact with the Rust, Python, and TypeScript bindings — a KMF snapshot written by any of them reads back identically here.

Backed by:

- **JVM** (including Android-JVM): the UniFFI-generated Kotlin (`packages/smritidb-ffi/bindings/kotlin/`) loading `libsmritidb_ffi` at runtime via JNA.
- **Apple targets** (`iosArm64`, `iosX64`, `iosSimulatorArm64`, `macosArm64`, `macosX64`), **Android Native** (`androidNativeArm64`, `androidNativeX64`), **and Linux Native** (`linuxX64`, `linuxArm64`): Cinterop against the static lib `libsmritidb_ffi.a`. All three families share the same Kotlin source set (`src/ffiNativeMain`) — only the cross-toolchain (Apple SDKs vs the Android NDK vs the glibc Linux cross-gcc) differs.
- **JS / WasmJs**: the WebAssembly build from `packages/core-rs/pkg/`.

## Install

When the Maven Central publish lands:

```kotlin
// build.gradle.kts
dependencies {
    implementation("com.tanvrit.smritidb:smritidb-kmp:0.1.0")
}
```

For local consumption today, see "Build" below — `gradle publishToMavenLocal` produces the JARs and KLibs and any consumer Gradle project can resolve them.

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
3. The native lib has been built for each target — for JVM dev, `cargo build --release` is enough; for iOS, build for `aarch64-apple-ios` + simulator triples and wrap into `libsmritidb_ffi.xcframework`. For Android Native, see below.

```bash
cd packages/smritidb-kmp
gradle build              # all targets
gradle jvmTest            # JVM-only quick test
gradle macosArm64Test     # Apple smoke test (links libsmritidb_ffi.a)
gradle publishToMavenLocal  # for local consumption
```

### Android Native cross-compile

The `androidNativeArm64` and `androidNativeX64` Kotlin/Native targets link the same `libsmritidb_ffi.a` archive as the Apple targets, just built with the Android NDK cross toolchain instead of the Apple SDKs. From a macOS host:

```bash
brew install --cask android-ndk          # one-time
rustup target add aarch64-linux-android x86_64-linux-android

export ANDROID_NDK_HOME=/opt/homebrew/share/android-ndk
export PATH="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin:$PATH"

cd packages/smritidb-ffi
cargo build --release --target aarch64-linux-android
cargo build --release --target x86_64-linux-android

cd ../smritidb-kmp
gradle linkDebugTestAndroidNativeArm64   # compile + link
gradle linkDebugTestAndroidNativeX64     # compile + link
# Running the test binary itself needs an Android emulator / device.
```

`packages/smritidb-ffi/.cargo/config.toml` pins the linker / archiver names; the toolchain dir on `PATH` is what makes them resolve.

### Linux Native cross-compile

The `linuxX64` and `linuxArm64` Kotlin/Native targets link the same `libsmritidb_ffi.a` archive, just built with a Linux GNU cross toolchain instead of the Apple SDKs / Android NDK. Two glibc versions are in play: the messense cross-gcc ships glibc 2.39 headers, while the Kotlin/Native sysroot pins glibc 2.19 (x64) / 2.25 (arm64). Compiling the Rust crate with the older sysroot keeps the two sides ABI-compatible. From a macOS host:

```bash
# one-time
brew tap messense/macos-cross-toolchains
brew install messense/macos-cross-toolchains/x86_64-unknown-linux-gnu
brew install messense/macos-cross-toolchains/aarch64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu

# warm Kotlin/Native's bundled sysroot (first run downloads the tarball)
cd packages/smritidb-kmp
gradle :compileKotlinLinuxX64 :compileKotlinLinuxArm64

# build the Rust archives against the Kotlin/Native sysroot
cd ../smritidb-ffi
SYSROOT_X64=$HOME/.konan/dependencies/x86_64-unknown-linux-gnu-gcc-8.3.0-glibc-2.19-kernel-4.9-2/x86_64-unknown-linux-gnu/sysroot
SYSROOT_ARM64=$HOME/.konan/dependencies/aarch64-unknown-linux-gnu-gcc-8.3.0-glibc-2.25-kernel-4.9-2/aarch64-unknown-linux-gnu/sysroot
CFLAGS_x86_64_unknown_linux_gnu="--sysroot=$SYSROOT_X64" \
  CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc \
  CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_AR=x86_64-unknown-linux-gnu-ar \
  CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS="-C link-arg=--sysroot=$SYSROOT_X64" \
  cargo build --release --target x86_64-unknown-linux-gnu
CFLAGS_aarch64_unknown_linux_gnu="--sysroot=$SYSROOT_ARM64" \
  CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-unknown-linux-gnu-gcc \
  CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_AR=aarch64-unknown-linux-gnu-ar \
  CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUSTFLAGS="-C link-arg=--sysroot=$SYSROOT_ARM64" \
  cargo build --release --target aarch64-unknown-linux-gnu

cd ../smritidb-kmp
gradle linkDebugTestLinuxX64    # compile + link
gradle linkDebugTestLinuxArm64  # compile + link
# Running the test binary on a Mac host needs docker / qemu;
# the `linuxX64Test` / `linuxArm64Test` gradle tasks SKIP on macOS:
docker run --rm --platform linux/amd64 \
  -v $(pwd)/build/bin/linuxX64/debugTest:/test \
  debian:bookworm-slim /test/test.kexe
```

The two `CARGO_TARGET_*_LINKER` / `_AR` vars are passed on the command line rather than declared in `packages/smritidb-ffi/.cargo/config.toml`, and that is deliberate: a `[target.x86_64-unknown-linux-gnu]` block in that file also applies when the triple is the **host**, so on a Linux runner it hijacks the plain `cargo test` and fails with a "linker `x86_64-unknown-linux-gnu-gcc` not found" error before compiling anything. That is what kept the `rust-ffi` and `kmp-jvm` CI jobs from ever running a test.

On a native Linux CI runner the `brew install` step is unnecessary — cargo finds the host `cc` directly and `gradle :linuxX64Test` (or `:linuxArm64Test`) actually executes the test binary. The build.gradle.kts wires `-l:libsmritidb_ffi.a` on Linux so the link picks the static archive deterministically even when `cargo build` has also produced `libsmritidb_ffi.so` next to it.

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

### JS / WasmJs (wasm-bindgen)

The browser / Node bindings load the `wasm-bindgen` build from
`packages/core-rs/pkg-node/` (CommonJS, used by `jsNodeTest`) and
`packages/core-rs/pkg/` (ESM, used by `wasmJs` and browser bundlers).
Both bundles are produced by `wasm-pack`:

```bash
# one-time
cargo install wasm-pack
rustup target add wasm32-unknown-unknown

# Build both bundles (the gradle wrapper task `wasmPackBuild` does the
# same; invoke it directly to skip running the Kotlin compile).
cd packages/core-rs
wasm-pack build --target web    --release            -- --no-default-features --features wasm
wasm-pack build --target nodejs --release --out-dir pkg-node -- --no-default-features --features wasm

cd ../smritidb-kmp
./gradlew :compileKotlinJs     # Kotlin/JS — IR backend, consumes pkg-node/
./gradlew :compileKotlinWasmJs # Kotlin/Wasm — consumes pkg/
./gradlew :jsNodeTest          # runs the smoke tests under Node
./gradlew :wasmJsNodeTest      # same suite on Kotlin/Wasm
```

The test npm dependencies are installed with npm, not Yarn
(`kotlin.js.yarn=false` in `gradle.properties`): the Yarn 1.22.22 that
Kotlin 2.4.20 ships refuses to run under the monorepo root's
`"packageManager": "pnpm@…"`.

`Cargo.toml` gates SQLite behind the `persist-sqlite` feature (default-on
for native, off for wasm) so the wasm build does NOT pull `rusqlite` —
`libsqlite3` doesn't compile to `wasm32-unknown-unknown`. The
`PersistentStore.openSqlite` / `openFile` factories on JS / WasmJs throw
`UnsupportedOperationException` for the same reason.

### Browser persistence

| Factory | Target | Storage | Durable across reloads? |
| --- | --- | --- | --- |
| `PersistentStore.openMemory()` | `js`, `wasmJs` | wasm-side `HashMap` | No |
| `PersistentStore.openIndexedDb(dbName)` | `js` only (Phase D) | KMF blob in IndexedDB | **Yes** |
| OPFS-sqlite-wasm | — | — | Phase E |

The Phase D **`openIndexedDb`** factory (Kotlin/JS only — see
`PersistentStoreJs.kt`) hosts the wasm memory store inside the page
and writes a KMF snapshot to IndexedDB on every `persistAsync()` /
`closeAsync()`. Reopening the same `dbName` reads the snapshot and
calls `storeRestoreFromSnapshot` over the wasm boundary so the rehydrated
substrate continues to work against the same handle protocol.

```kotlin
// Kotlin/JS — browser or Node (with `fake-indexeddb/auto` in tests)
val store = PersistentStore.openIndexedDb("my_app", dimension = 10_000)
store.put("the cat sat on the mat", "the cat sat on the mat".encodeToByteArray())
store.persistAsync()   // awaits the IDB transaction commit
store.closeAsync()     // snapshots-then-closes
```

The synchronous `persist()` / `close()` actuals are kept for source
compatibility with the JVM / Native bindings; on the IDB path they
issue a best-effort fire-and-forget write. Durability-sensitive code
should always prefer the `*Async` variants.

The IDB schema is a single out-of-line object store `smritidb_snapshot`
keyed by the constant `1`, with the KMF blob as the value. A second
object store `smritidb_wal` (autoincrement) is created at schema-v1
upgrade time but reserved for the Phase E append-only WAL — v1 is
snapshot-only, which is enough for a first cut and matches the
in-memory adapter's semantics from the Rust core.

**Kotlin/Wasm note.** Kotlin/Wasm JS interop ferries `ByteArray`s as hex strings
which is wasteful for a 10 kB snapshot — Phase E migrates the bridge
to an `ArrayBuffer` round-trip and clones the `IndexedDbAdapter` shape
verbatim.

`fake-indexeddb` is wired as a `jsTest`-only npm dev dep so the
`jsNodeTest` task exercises the adapter under Node; production code
imports nothing — it uses the browser-native `indexedDB` global.

`wasm-pack`'s bundled `wasm-opt` predates the `--enable-bulk-memory`
flag default; Rust 1.95's LLVM emits bulk-memory ops unconditionally.
`Cargo.toml` opts out with `[package.metadata.wasm-pack.profile.release] wasm-opt = false`.

The Kotlin/Wasm Node test runner needs `process.getBuiltinModule`
(Node 22.1+): without it `@JsFun` closures cannot resolve a synchronous
`require()` to load the wasm bundle. The gradle-managed Node that Kotlin
2.4.20 provisions by default (24.16.0 for `js`, 26.2.0 for `wasmJs`)
clears that floor, so `wasmJsNodeTest` runs; only `wasmJsBrowserTest`
stays disabled in `build.gradle.kts`.

## Docs

- API reference (dokka): [`docs/api/kotlin/index.html`](../../docs/api/kotlin/index.html)
- Persistence architecture: [`docs/guides/persistence.md`](../../docs/guides/persistence.md)
- Cross-language interop demo: [`examples/polyglot-interop/`](../../examples/polyglot-interop/)

## Tests

**100+ tests passing** across the live targets. Kotlin 2.4.20 / Gradle 9.7.1, verified 2026-09-24: JVM 13, macosArm64 14, jsNode 22, wasmJsNode 22. Earlier toolchain: linuxX64 15, linuxArm64 15 (under Docker). Android Native compiles + links; its test runner needs an emulator.

## Status

- Common API surface defined in `src/commonMain` — `expect` declarations for all primitives + the `SmritidbStore` interface.
- JVM `actual` complete (`src/jvmMain`) — delegates to UniFFI. **12 tests passing.**
- Kotlin/Native `actual` complete (`src/ffiNativeMain`) — hand-rolled UniFFI lift/lower against the C ABI in `smritidbFFI.h`; shared across Apple (macOS + iOS), Android Native, and Linux Native targets. **macosArm64: 14 tests passing; linuxX64 + linuxArm64: 15 tests each under Docker.**
- Kotlin/JS `actual` complete (`src/jsMain`) — delegates to `wasm-bindgen` via `@JsModule("smritidb-core")`. **22 tests passing under jsNodeTest** (Phase D adds the IndexedDB adapter; 3 new smoke tests in `IndexedDbAdapterTest`).
- Kotlin/Wasm `actual` complete (`src/wasmJsMain`). **22 tests passing under wasmJsNodeTest** (see "JS / WasmJs" above). IndexedDB adapter is JS-only for now — the Kotlin/Wasm port lands in Phase E.
- Common tests use `kotlin.test` and run on each target's runtime; Android Native test execution requires an emulator / device, Linux Native execution needs docker / qemu on a Mac host.

## License

Apache-2.0.
