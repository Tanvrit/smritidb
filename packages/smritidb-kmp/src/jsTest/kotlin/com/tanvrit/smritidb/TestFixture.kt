package com.tanvrit.smritidb

/**
 * JS uses `@JsModule("smritidb-core")` so the wasm-bindgen module is
 * resolved by Webpack at compile time — no runtime bootstrap needed.
 */
internal actual fun setUpSmritidbTestFixture() {}
