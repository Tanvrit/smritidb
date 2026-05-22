package com.tanvrit.smritidb

/**
 * Per-test bootstrap hook used by `commonTest` suites that exercise the
 * wasm-bindgen bridge.
 *
 * Most targets don't need anything — JVM / Native link the Rust core
 * statically, JS uses `@JsModule`. The Kotlin/Wasm target is the
 * exception: it can't use `@JsModule`, so the host has to stash the
 * wasm-bindgen Node module on `globalThis.SmritidbWasm` before any
 * `@JsFun` bridge runs. Under `wasmJsNodeTest`, the `actual` calls
 * `ensureWasmBootstrapped()` from `wasmJsTest/.../Bootstrap.kt` which
 * uses `process.getBuiltinModule('node:module')` + `createRequire` to
 * load the wasm bundle synchronously.
 *
 * Every other test target gets a no-op `actual`.
 */
internal expect fun setUpSmritidbTestFixture()
