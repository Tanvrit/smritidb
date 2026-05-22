@file:JsModule("smritidb-core")
@file:JsNonModule

package com.tanvrit.smritidb

import org.khronos.webgl.Uint8Array

/**
 * External declarations for the wasm-bindgen output in
 * `packages/core-rs/pkg-node/`. The `@JsModule("smritidb-core")`
 * resolves via the NPM dependency wired in `build.gradle.kts`
 * (`implementation(npm("smritidb-core", File("../core-rs/pkg-node").absolutePath))`).
 *
 * The Node target is CommonJS and synchronously loads its own `.wasm`
 * sibling at `require` time — there is no async init step. The browser
 * (web) target is asynchronous, but we don't currently exercise it from
 * `jsTest` (the gradle task runs under Node).
 *
 * Function names mirror the `js_name = ...` arguments in `wasm.rs`:
 * camelCase to match TypeScript convention while the Rust crate keeps
 * snake_case internally.
 */

external fun storeOpenMemory(dimension: Int?): Int

external fun storePut(handle: Int, key: String, value: Uint8Array): String

external fun storeRecall(
    handle: Int,
    cue: String,
    top_k: Int,
    min_sim: Double,
): dynamic

external fun storeDelete(handle: Int, id: String): Boolean

external fun storeSize(handle: Int): Int

external fun storeDimension(handle: Int): Int

external fun storeConsolidate(handle: Int)

external fun storeClose(handle: Int)

/** KMF snapshot of the substrate — Phase D persistence adapters write this verbatim. */
external fun storeSnapshot(handle: Int): Uint8Array

/** Restore the substrate from a KMF snapshot, preserving the existing handle. */
external fun storeRestoreFromSnapshot(handle: Int, bytes: Uint8Array)

external fun specVersion(): String

external fun randomHv(seed: Uint8Array, dim: Int): Uint8Array
external fun similarity(a: Uint8Array, b: Uint8Array): Double
external fun bind(a: Uint8Array, b: Uint8Array): Uint8Array
external fun unbind(a: Uint8Array, b: Uint8Array): Uint8Array
external fun bundle(hvs: Uint8Array, dim: Int): Uint8Array
external fun permute(hv: Uint8Array, k: Int): Uint8Array
external fun encodeString(s: String, dim: Int): Uint8Array
