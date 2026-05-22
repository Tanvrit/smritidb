@file:Suppress("NOTHING_TO_INLINE")

package com.tanvrit.smritidb

/**
 * Bootstrap helpers for `fake-indexeddb` — Node's stand-in for the
 * browser `indexedDB` global. Lives in `jsTest` so production builds
 * never see the polyfill.
 *
 * Kotlin's `js(...)` intrinsic only accepts a single expression, so the
 * install / reset routines are kept as one-liners that delegate to the
 * polyfill's published surface.
 */

internal inline fun installFakeIndexedDbAuto() {
    // `fake-indexeddb/auto` is side-effect-only; importing it installs
    // `indexedDB`, `IDBKeyRange`, etc. on `globalThis`. Safe to call
    // repeatedly.
    js("require('fake-indexeddb/auto')")
}

internal inline fun resetIndexedDb() {
    // The polyfill exposes an `IDBFactory` constructor; instantiating a
    // fresh one and assigning it to `globalThis.indexedDB` discards all
    // stored data without unloading the module. This is the canonical
    // reset pattern documented in `fake-indexeddb`'s README.
    js("globalThis.indexedDB = new (require('fake-indexeddb').IDBFactory)()")
}
