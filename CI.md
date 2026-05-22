# CI

Smritidb runs three GitHub Actions workflows. All three live under [`.github/workflows/`](./.github/workflows/).

The single most important invariant the CI defends is **bit-exactness across bindings**: a snapshot emitted from any binding must be byte-identical to one emitted from any other. The conformance workflow is the gate that locks this in.

## Workflows

### `ci.yml` — primary CI

Runs on every push to `main` and every pull request. Each binding is its own job so they execute in parallel where they don't share dependencies.

| Job | Runner | What it does |
|---|---|---|
| `rust-core` | ubuntu-latest | `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test --release` (50 tests) over `packages/core-rs`, plus a `cargo build --release --features wasm` smoke. |
| `rust-ffi` | ubuntu-latest | `cargo test --release` over `packages/smritidb-ffi` (6 tests) and builds the cdylib for downstream KMP. |
| `rust-c-abi` | ubuntu-latest | `cargo test --release` over `packages/smritidb-c` (2 tests) plus compiles and runs the C smoke harness under `packages/smritidb-c/tests/smoke.c`. |
| `python` | ubuntu-latest | `maturin develop --release` + `pytest -q` over `packages/smritidb-py` (21 tests). |
| `kmp-jvm` | ubuntu-latest | Builds the UniFFI cdylib, stages it under `target/release-jna/linux-x86-64/`, then `gradle :jvmTest` (12 tests). |
| `kmp-apple` | macos-latest | Compile + JVM smoke on macOS. Cinterop wiring for the real Apple targets is deferred. |
| `typescript` | ubuntu-latest | `pnpm typecheck`, `pnpm lint`, `pnpm test` (73 tests), `pnpm build`. |
| `conformance-gate` | ubuntu-latest | Depends on `rust-core` + `typescript`. Asserts `tests/conformance/golden.json` and `tests/conformance/kmf_fixture.bin` are unchanged from HEAD, then re-emits the KMF fixture from the TS reference and `cmp`s the bytes. |
| `notebooks` | ubuntu-latest | Phase 0 HDC validation notebook smoke. |
| `spec-lint` | ubuntu-latest | `markdownlint` over `SPEC.md`, `docs/`, `README.md`. |

Caches:
- `~/.cargo/{registry,git}` and each crate's `target/` via `actions/cache@v4`, keyed on the Cargo manifest hashes.
- `pnpm` store + `node_modules` via `actions/cache@v4`, keyed on `pnpm-lock.yaml`.
- Gradle via `gradle/actions/setup-gradle@v4` (manages its own layout).

Test artifacts upload only on failure.

### `conformance.yml` — bit-exactness gate

Runs on every PR, nightly at 07:00 UTC, and on `workflow_dispatch`. Smaller and tighter than `ci.yml`: only the corpus + KMF cross-impl tests in each binding plus the fixture re-emit. The final `conformance-summary` job is the one to require in branch protection.

| Job | What it asserts |
|---|---|
| `fixture-parity` | `golden.json` + `kmf_fixture.bin` unchanged from HEAD; re-emitted KMF fixture byte-identical to the checked-in one. |
| `ts-conformance` | TypeScript runs `conformance.test.ts` + `kmf.cross-impl.test.ts`. |
| `rust-conformance` | `cargo test --test conformance --test kmf_cross_impl` (10 tests). |
| `py-conformance` | `pytest` against the maturin-built extension. |
| `kmp-conformance` | `gradle :jvmTest` (UniFFI + JNA cdylib path). |
| `conformance-summary` | Single required check; green iff every binding passed. |

### `release.yml` — release artifact builds (scaffold)

Runs on tag pushes matching `v*` (e.g. `v0.1.0`) and on `workflow_dispatch`. Builds artifacts but does not push to any external registry — publish steps are commented out and require secrets that have not been provisioned yet.

| Job | Artifact |
|---|---|
| `python-wheels` | `manylinux2014` (x86_64, aarch64), macOS (arm64, x86_64), Windows (x86_64) abi3 wheels via `maturin`. |
| `npm-package` | `@tanvrit/smritidb` tarball via `pnpm pack`. |
| `kmp-jvm` | Maven `publishJvmPublicationToMavenLocal` layout. |
| `dotnet-nuget` | Scaffold only (disabled — `smritidb-dotnet` binding isn't ready). |
| `github-release` | Collects all of the above and attaches to the GitHub Release for the tag. |

## Running locally

Everything CI runs can be run locally with the same commands. Run from the repo root unless noted.

```bash
# Rust core — 50 tests
cd packages/core-rs && cargo test --release

# Rust FFI — 6 tests
cd packages/smritidb-ffi && cargo test --release

# Rust C ABI — 2 tests + C smoke
cd packages/smritidb-c && cargo test --release && \
  clang -I include -L target/release -lsmritidb_c \
    -Wl,-rpath,'$ORIGIN/../target/release' \
    tests/smoke.c -o tests/smoke && \
  LD_LIBRARY_PATH=target/release ./tests/smoke

# Python — 21 tests
cd packages/smritidb-py && maturin develop --release && pytest -q

# KMP JVM — 12 tests (requires the FFI cdylib staged under release-jna/)
cd packages/smritidb-ffi && cargo build --release && \
  mkdir -p target/release-jna/$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m) && \
  cp target/release/libsmritidb_ffi.* target/release-jna/$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m)/ ; \
  cd ../smritidb-kmp && gradle :jvmTest

# TypeScript — 73 tests
pnpm install && pnpm -C packages/core-ts test

# Conformance fixture re-emit
pnpm -C packages/core-ts build && \
  node tests/conformance/emit_kmf_fixture.mjs && \
  git diff --exit-code -- tests/conformance/kmf_fixture.bin
```

## Action pinning policy

All third-party actions are pinned to a major version tag (e.g. `@v4`). The pin is deliberate — `@latest` is never used, and we accept the trade-off that minor/patch upgrades land automatically. When a major version of a critical action ships breaking changes, bump the pin in a focused PR rather than letting it drift.

## Deferred concerns

- **Windows runner for Rust + Python + KMP.** `rusqlite` (bundled) and JNA both have historical pitfalls on the GitHub-hosted `windows-latest` runners — the MSVC toolchain interaction with `bundled` and JNA's library-loading semantics are easy to misconfigure. We build a Windows wheel in `release.yml`, but the full test matrix is Linux + macOS only. Adding Windows tests is a tracked follow-up.
- **KMP Apple targets** (`iosArm64`, `iosX64`, `iosSimulatorArm64`, `macosArm64`, `macosX64`). Declared in `packages/smritidb-kmp/build.gradle.kts`, but the per-target `actual` and the Cinterop wiring against `libsmritidb_ffi.a` ship in the next phase. CI currently runs only `:jvmTest` on `macos-latest` as a smoke. The full `gradle build` on macOS will be enabled once the Apple `actual`s exist.
- **KMP JS / WasmJs.** Declared but with no real `actual` implementation yet — left out of CI to keep the gate honest.
- **Android Native and Linux Native KMP.** Declared with `nativeMain` shared sources, but no `actual` shipped — left out of CI.
- **Publish-to-registry steps** for npm / PyPI / Maven Central / NuGet. Scaffolded in `release.yml` but commented out; each needs a secret (`NPM_TOKEN`, `PYPI_API_TOKEN`, Sonatype credentials + GPG, `NUGET_API_KEY`) before being enabled.
- **`smritidb-dotnet` / `smritidb-go` / `smritidb-dart` bindings.** Crates exist but are early — once they have real test suites the conformance workflow should gain a job per binding.

## Expected CI runtime

Cold runs (no warm caches) take roughly 12–18 minutes end-to-end, dominated by `rust-core` (Rust release build + LTO) and `kmp-jvm` (Gradle bootstrap + FFI cdylib build). Warm cache runs (most PR pushes after the first) come in around 5–8 minutes. The `conformance-gate` job adds 2–3 minutes on top of the longest of its two dependencies.
