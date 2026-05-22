package com.tanvrit.smritidb

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

/**
 * Smoke test for the Apple actual of [PersistentStore].
 *
 * Exercises only the `Memory` adapter so the test is identical across
 * `iosArm64`, `iosX64`, `iosSimulatorArm64`, `macosArm64`, and
 * `macosX64` — no filesystem layout to worry about. The JVM
 * `PersistentStoreTest` covers the SQLite + snapshot adapters; if the
 * memory round-trip works on Apple, the rusqlite-bundled adapters
 * built into the same `libsmritidb_ffi.a` work too.
 */
class PersistentStoreAppleTest {
    @Test
    fun memoryStoreRoundTripOnApple() {
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
