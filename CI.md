# CI

Smritidb runs three GitHub Actions workflows. All three live under [`.github/workflows/`](./.github/workflows/). Every job runs on Tanvrit's **self-hosted** runners (group `tanvrit`); no job may name a GitHub-hosted image, and `self-hosted-guard.yml` enforces that on every push.

The single most important invariant the CI defends is **bit-exactness across bindings**: a snapshot emitted from any binding must be byte-identical to one emitted from any other. The conformance workflow is the gate that locks this in.

## Workflows

### `ci.yml` — primary CI

Runs on every push to `main` and every pull request. Each binding is its own job so they execute in parallel where they don't share dependencies.

| Job | Runner | What it does |
|---|---|---|
| `rust-core` | Linux X64 | `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test --release` (50 tests) over `packages/core-rs`, plus a `cargo build --release --features wasm` smoke. |
| `rust-ffi` | Linux X64 | `cargo test --release` over `packages/smritidb-ffi` (6 tests) and builds the cdylib for downstream KMP. |
| `rust-c-abi` | Linux X64 | `cargo test --release` over `packages/smritidb-c` (2 tests) plus compiles and runs the C smoke harness under `packages/smritidb-c/tests/smoke.c` with the system `cc` (the fleet has no clang, and the harness is plain C99). |
| `python` | Linux X64 | Python 3.11 provisioned by `uv` (see *Toolchain version sources of truth*), then `maturin build --release` + `uv pip install dist/*.whl` + `pytest -q` over `packages/smritidb-py` (21 tests). |
| `kmp-jvm` | Linux X64 | Builds the UniFFI cdylib, stages it under `target/release-jna/linux-x86-64/`, then `gradle :jvmTest` (13 tests) on exactly the Gradle version the wrapper pins. |
| `kmp-apple` | macOS | Compile + JVM smoke on macOS. Cinterop wiring for the real Apple targets is deferred. |
| `typescript` | Linux X64 | `pnpm typecheck`, `pnpm lint`, `pnpm test` (73 tests), `pnpm build`. |
| `conformance-gate` | Linux X64 | Depends on `rust-core` + `typescript`. Asserts `tests/conformance/golden.json` and `tests/conformance/kmf_fixture.bin` are unchanged from HEAD, then re-emits the KMF fixture from the TS reference and `cmp`s the bytes. |
| `notebooks` | Linux X64 | Phase 0 HDC validation notebook smoke (Python 3.12 via `uv`). |
| `spec-lint` | Linux X64 | `markdownlint` over `SPEC.md`, `docs/`, `README.md`. |

The Linux runners are separate runner processes on ONE host that share `HOME=/root`, so anything a job installs under `$HOME` is visible to, and can be deleted by, a concurrent job. Every tool is therefore provisioned in-workflow and keyed per runner registration:
- pnpm: `pnpm/action-setup` with `dest: ~/setup-pnpm-${{ runner.name }}`. The default `~/setup-pnpm` is deleted and reinstalled on every run, which pulled pnpm out from under a concurrent job (Conformance run 35860816227).
- Python: `uv` with `UV_PYTHON_INSTALL_DIR` under the runner's own tool cache.
- Gradle: `GRADLE_USER_HOME` per runner, not the shared `/root/.gradle`.

Caches:
- `~/.cargo/{registry,git}` and each crate's `target/` via `actions/cache@v4`, keyed on the Cargo manifest hashes.
- `pnpm` store + `node_modules` via `actions/cache@v4`, keyed on `pnpm-lock.yaml`.
- Gradle via `gradle/actions/setup-gradle@v4`, given the wrapper's `gradle-version` explicitly.

Test artifacts upload only on failure (`if: failure()`, short `retention-days`).

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

# Rust C ABI — 2 tests + C smoke. The source file comes BEFORE -lsmritidb_c:
# linkers that default to --as-needed drop a library named before its user.
cd packages/smritidb-c && cargo test --release && \
  cc -I include tests/smoke.c -o tests/smoke \
    -L target/release -lsmritidb_c \
    -Wl,-rpath,'$ORIGIN/../target/release' && \
  LD_LIBRARY_PATH=target/release ./tests/smoke

# Python — 21 tests, in a uv-managed venv (the same interpreter CI uses).
# `maturin develop` needs an ACTIVE virtualenv; CI builds and installs the wheel.
uv python install 3.11 && uv venv --managed-python --python 3.11 .venv && . .venv/bin/activate && \
  uv pip install maturin pytest && \
  cd packages/smritidb-py && maturin build --release --out dist && \
  uv pip install --force-reinstall dist/*.whl && pytest -q

# KMP JVM — 13 tests (requires the FFI cdylib staged under release-jna/)
cd packages/smritidb-ffi && cargo build --release && \
  mkdir -p target/release-jna/$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m) && \
  cp target/release/libsmritidb_ffi.* target/release-jna/$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m)/ ; \
  cd ../smritidb-kmp && ./gradlew :jvmTest

# TypeScript — 73 tests
pnpm install && pnpm -C packages/core-ts test

# Conformance fixture re-emit
pnpm -C packages/core-ts build && \
  node tests/conformance/emit_kmf_fixture.mjs && \
  git diff --exit-code -- tests/conformance/kmf_fixture.bin
```

## Action pinning policy

All third-party actions are pinned to a major version tag (e.g. `@v4`). The pin is deliberate — `@latest` is never used, and we accept the trade-off that minor/patch upgrades land automatically. When a major version of a critical action ships breaking changes, bump the pin in a focused PR rather than letting it drift.

The one exception is `astral-sh/setup-uv`, which publishes no moving major tag (only `vX.Y.Z`), so it is pinned by commit SHA with the version in a trailing comment.

## Toolchain version sources of truth

Each toolchain version is declared in exactly one place, and the workflows read it from there rather than restating it:

| Toolchain | Source of truth | How CI reads it |
|---|---|---|
| Node | `.nvmrc` | `actions/setup-node` with `node-version-file: .nvmrc` |
| pnpm | `packageManager` in the root `package.json` | `pnpm/action-setup` with **no** `version:` input |
| Python | the `uv python install <minor>` step in each Python job (3.11 for the binding, 3.12 for the notebooks) | `astral-sh/setup-uv` (uv itself pinned by `version:`), then `uv python install` + `uv venv --managed-python`, and a check that the interpreter is that minor. `actions/setup-python` is not used: it only ships Ubuntu builds and fails on the fleet's Debian 13 ("version '3.11' ... not found for Debian 13"). |
| Gradle | `distributionUrl` in `packages/smritidb-kmp/gradle/wrapper/gradle-wrapper.properties` (8.12.1, the newest Gradle Kotlin 2.1.20 fully supports; `distributionSha256Sum` pinned) | Each KMP job reads the version from that file, passes it to `setup-gradle` as `gradle-version`, and fails with `::error::` if `gradle --version` disagrees. Before this, `setup-gradle` got no version and the job ran whatever `gradle` the host had: nothing on the Linux fleet (exit 127), Homebrew's on the Mac. |

`pnpm/action-setup@v4` hard-errors (`Multiple versions of pnpm specified`) when it is given a `version:` input *and* finds `packageManager` in `package.json`. `packageManager` is the one to keep: Corepack reads it, so it also governs local dev and `pnpm install` on a contributor's machine — a workflow input governs only CI, and drifting from it is exactly the failure the error is warning about. Bumping pnpm therefore means editing `package.json` alone.

## Deferred concerns

- **Windows runner for Rust + Python + KMP.** `rusqlite` (bundled) and JNA both have historical pitfalls on Windows — the MSVC toolchain interaction with `bundled` and JNA's library-loading semantics are easy to misconfigure — and CI may only use a self-hosted Windows runner. We build a Windows wheel in `release.yml`, but the full test matrix is Linux + macOS only. Adding Windows tests is a tracked follow-up.
- **KMP Apple targets** (`iosArm64`, `iosX64`, `iosSimulatorArm64`, `macosArm64`, `macosX64`). Declared in `packages/smritidb-kmp/build.gradle.kts`, but the per-target `actual` and the Cinterop wiring against `libsmritidb_ffi.a` ship in the next phase. CI currently runs only `:jvmTest` on a self-hosted macOS runner as a smoke. That job pins `[self-hosted, macOS]` with no architecture label while it stages a darwin-aarch64 library, so it would fail to load JNA on the Intel mac runner; adding `ARM64` is a known follow-up. The full `gradle build` on macOS will be enabled once the Apple `actual`s exist.
- **KMP JS / WasmJs.** Declared but with no real `actual` implementation yet — left out of CI to keep the gate honest.
- **Android Native and Linux Native KMP.** Declared with `nativeMain` shared sources, but no `actual` shipped — left out of CI.
- **Publish-to-registry steps** for npm / PyPI / Maven Central / NuGet. Scaffolded in `release.yml` but commented out; each needs a secret (`NPM_TOKEN`, `PYPI_API_TOKEN`, Sonatype credentials + GPG, `NUGET_API_KEY`) before being enabled.
- **`smritidb-dotnet` / `smritidb-go` / `smritidb-dart` bindings.** Crates exist but are early — once they have real test suites the conformance workflow should gain a job per binding.

## Expected CI runtime

Cold runs (no warm caches) take roughly 12–18 minutes end-to-end, dominated by `rust-core` (Rust release build + LTO) and `kmp-jvm` (Gradle bootstrap + FFI cdylib build). Warm cache runs (most PR pushes after the first) come in around 5–8 minutes. The `conformance-gate` job adds 2–3 minutes on top of the longest of its two dependencies.
