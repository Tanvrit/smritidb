# Regenerating the API Docs

The HTML under `docs/api/<lang>/` is auto-generated. After changing a
public surface, regenerate the affected target and commit the refreshed
output.

The driver script `docs/regen.sh` runs every step in order. Run it from
the repository root.

## All targets

```sh
./docs/regen.sh
```

## Per-target

### Rust (`core-rs`, `smritidb-ffi`, `smritidb-c`)

No workspace root — each crate is a standalone Cargo project — so doc
generation is per-crate. They share an output directory so the
cross-links between crates resolve.

```sh
cd packages/core-rs && \
  cargo doc --no-deps --target-dir ../../docs/api/rust --no-default-features
cd packages/smritidb-ffi && \
  cargo doc --no-deps --target-dir ../../docs/api/rust
cd packages/smritidb-c && \
  cargo doc --no-deps --target-dir ../../docs/api/rust
```

The `--no-default-features` flag on `core-rs` keeps `rusqlite` out of
the doc build; including it adds ~12 MB of bundled-libsqlite3 sources
to the doc database for no reader benefit. If you want the SQLite
adapter's docs visible, drop the flag and accept the heavier output.

`smritidb-c`'s rustdoc covers the Rust side of the C ABI; the raw
header surface is in [`packages/smritidb-c/include/smritidb.h`](../packages/smritidb-c/include/smritidb.h).

### Python (`smritidb-py`)

```sh
source .venv/bin/activate         # or whatever your venv path is
pip install pdoc                  # one-time
cd packages/smritidb-py && pdoc smritidb -o ../../docs/api/python
```

`pdoc` introspects the installed `smritidb` package (i.e., it needs
`maturin develop` to have run so `_native.abi3.so` exists in the venv).
The HTML rooted at `docs/api/python/smritidb.html` is the entry point.

If `pdoc` is unavailable in the consumer environment, regenerate from
the `.pyi` type stubs at `packages/smritidb-py/python/smritidb/_native.pyi`
and the package docstring at `python/smritidb/__init__.py`.

### Kotlin (`smritidb-kmp`)

Dokka is **not declared** in `packages/smritidb-kmp/build.gradle.kts` —
we apply it via a Gradle init script so the build file stays focused on
the multiplatform wiring. The init script (`docs/dokka-init.gradle.kts`,
shipped alongside this file) declares the Dokka plugin classpath and
configures source roots from the multiplatform layout.

```sh
# Dokka 1.9.20 requires JDK 17. JDK 25 (the Homebrew default at time of
# writing) crashes Dokka's bundled IntelliJ utilities with
# `IllegalArgumentException: 25.0.2`. Always pin JDK 17.
export JAVA_HOME=/Library/Java/JavaVirtualMachines/zulu-17.jdk/Contents/Home

cd packages/smritidb-kmp && \
  gradle --init-script ../../docs/dokka-init.gradle.kts \
         dokkaHtml --no-daemon \
         -x wasmPackBuildWeb -x wasmPackBuildNode
```

We skip `wasmPackBuildWeb` / `wasmPackBuildNode` (they need
`wasm-pack` on PATH and aren't relevant to documentation).

Output goes directly into `docs/api/kotlin/` (configured in the init
script via `outputDirectory.set(...)`).

### TypeScript (`core-ts`)

```sh
cd packages/core-ts && \
  npm install --save-dev typedoc && \
  npx typedoc src/index.ts --out ../../docs/api/typescript --skipErrorChecking
```

`--skipErrorChecking` keeps TypeDoc from re-running the `tsc` checker;
the package's own `pnpm typecheck` is the canonical check.

## When to regenerate

- After changing any public function signature, type, or class.
- After changing any docstring (Rustdoc, JSDoc, Kdoc, Python docstring).
- After bumping `SPEC_VERSION` or `KMF_SPEC_VERSION`.
- Before cutting a release.

It is safe to run `./docs/regen.sh` on a clean checkout to confirm no
generated files have drifted from source.

## Output footprint

A full regeneration produces roughly:

| Target          | Size  |
|-----------------|-------|
| `api/rust/doc`  | 4 MB  |
| `api/python`    | 200 KB|
| `api/kotlin`    | 3 MB  |
| `api/typescript`| 800 KB|

These are checked in. They will be picked up by a future `.gitignore`
review.
