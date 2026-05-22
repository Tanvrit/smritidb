package com.tanvrit.smritidb

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

/**
 * Smoke test for the Android Native actual of [PersistentStore].
 *
 * Mirrors `PersistentStoreAppleTest` so the same Cinterop-driven
 * UniFFI bridge is exercised on `androidNativeArm64` and
 * `androidNativeX64`. The implementation is source-shared with Apple
 * via the `ffiNativeMain` source set; the only per-target moving
 * parts are the Cinterop klib and the static archive
 * (`libsmritidb_ffi.a` built for `aarch64-linux-android` /
 * `x86_64-linux-android`).
 *
 * Note: Android Native test binaries do not execute on the host —
 * `gradle :smritidb-kmp:androidNativeArm64Test` runs them through an
 * Android emulator / device, which is out of scope on plain build
 * hosts. The build only links the test executable; this file's job
 * is to give the link a non-empty test entry point so any missing
 * lift/lower symbol surfaces as a link error.
 */
class PersistentStoreAndroidTest {
    @Test
    fun memoryStoreRoundTripOnAndroidNative() {
        val store = PersistentStore.openMemory()
        try {
            assertEquals(10_000, store.dimension())
            assertEquals(0, store.size())

            val id = store.put(
                key = "alpha",
                value = "hello".encodeToByteArray(),
                tags = listOf("greeting"),
            )
            assertTrue(id.isNotEmpty())
            assertEquals(1, store.size())

            val matches = store.recall(cue = "alpha", topK = 5, minSim = 0.0)
            assertTrue(matches.isNotEmpty())
            assertEquals(id, matches.first().id)
            assertTrue(matches.first().similarity >= 0.5)
            assertEquals(listOf("greeting"), matches.first().tags)

            assertTrue(store.delete(id))
            assertEquals(0, store.size())
        } finally {
            store.close()
        }
    }

    @Test
    fun closeIsIdempotent() {
        val store = PersistentStore.openMemory()
        store.close()
        store.close() // no exception
    }
}
