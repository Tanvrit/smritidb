package com.tanvrit.smritidb

/**
 * Kotlin/Wasm needs `globalThis.SmritidbWasm` stashed before any
 * `@JsFun` bridge runs. Delegates to the idempotent loader in
 * `Bootstrap.kt`.
 */
internal actual fun setUpSmritidbTestFixture() {
    ensureWasmBootstrapped()
}
