# Changelog

All notable changes to Smritidb will be documented in this file. Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), versioning follows [SemVer](https://semver.org/).

## [Unreleased]

### Changed — TypeScript (core-ts)
- `@tanvrit/smritidb` now has a `browser` export condition pointing at a Node-free entry (`dist/index.browser.js`, with its own `.d.ts`): the full surface minus `fsAdapter` and `sqliteAdapter`. Web bundlers previously had to resolve `node:fs/promises` and failed (`UnhandledSchemeError`). The Node entry's surface is unchanged.
- `SqliteAdapterOptions` is now `object` instead of an empty interface, so a primitive (`sqliteAdapter(db, 0)`) no longer type-checks. Every options object accepted before is still accepted.

### Fixed — TypeScript (core-ts)
- `sqliteAdapter(path)` in a webpack-bundled server (e.g. Next.js) failed at runtime with `Cannot find module 'better-sqlite3'`: webpack compiled the dynamic import into an empty context module. It is now marked `webpackIgnore` and left to Node.

### Added — Infrastructure
- ESLint 9 (flat config, `typescript-eslint` recommended) for `packages/core-ts` and `web`. The `lint` scripts existed but ESLint was never installed, so `pnpm lint` could not run.

## [0.1.0] — 2026-05-22

### Added — Spec
- SPEC v0.1.0-draft locked.
- Bit-exact cross-implementation conformance corpus at `tests/conformance/golden.json`.
- KMF wire format with BLAKE3-per-block integrity and magic trailer.

### Added — Rust
- `smritidb-core` Rust crate (primitives + Store + KMF + persistence trait + Memory/FS/SQLite adapters).
- `smritidb-ffi` UniFFI bridge (Kotlin + Swift bindings).
- `smritidb-c` stable C ABI with cbindgen-generated header.

### Added — Bindings
- Python (PyO3) — primitives + PersistentStore + context manager.
- TypeScript (core-ts) — full reference implementation; SQLite adapter unified with Rust schema.
- Kotlin Multiplatform — 9 of 11 declared targets working (JVM, macOS arm64+x64, iOS arm64+sim arm64+sim x64, Linux x64+arm64, JS Node with IndexedDB).
- Swift bindings via UniFFI.
- Go (CGo) and .NET (P/Invoke) bindings via the C ABI.

### Added — Cross-language Interop
- Cross-binding KMF byte-identity verified via fixture `tests/conformance/kmf_fixture.bin`.
- Polyglot demo at `examples/polyglot-interop/` proving Python → Rust → Kotlin → TypeScript bit-exact recall.
- Browser persistence via IndexedDB on Kotlin/JS.

### Added — Infrastructure
- GitHub Actions CI (ci.yml, conformance.yml, release.yml).
- API docs for all bindings (rustdoc, pdoc, Dokka, TypeDoc).
- Example apps (agent-memory, semantic-notebook).

### Added — Patent
- IPO Form-2 Complete Specification at `patent/FORM-2-COMPLETE-SPECIFICATION.md` (~24,500 words, filing-ready).
- Agent engagement letter at `patent/AGENT-ENGAGEMENT-LETTER.md`.

### Notes
- 234+ tests passing across the monorepo.
- Zero third-party storage SDK dependencies — all persistence via libsqlite3 (rusqlite bundled).

[0.1.0]: https://github.com/Tanvrit/smritidb/releases/tag/v0.1.0
