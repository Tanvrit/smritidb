package com.tanvrit.smritidb

import kotlin.test.AfterTest
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFails
import kotlin.test.assertFalse
import kotlin.test.assertTrue

/**
 * JS-specific smoke tests for the wasm-backed [PersistentStore].
 *
 * Mirrors `commonTest/.../PersistentStoreTest.kt`, but additionally
 * asserts that the SQLite / file-system factories throw
 * `UnsupportedOperationException` — the documented Phase B contract.
 *
 * Runs under Node via the `jsNodeTest` Gradle task. The wasm module is
 * resolved via the NPM dep `smritidb-core` wired to `../core-rs/pkg-node`.
 */
class PersistentStoreJsTest {
    private val stores = mutableListOf<PersistentStore>()

    @AfterTest
    fun cleanup() {
        for (s in stores) {
            runCatching { s.close() }
        }
        stores.clear()
    }

    private fun openMemory(dim: Int = 4096): PersistentStore {
        val s = PersistentStore.openMemory(dim)
        stores += s
        return s
    }

    @Test
    fun memoryStoreRoundTrip() {
        val store = openMemory()
        store.put("alpha", "hello".encodeToByteArray())
        store.put("beta", "world".encodeToByteArray())
        assertEquals(2, store.size())

        val hits = store.recall("alpha", topK = 2, minSim = 0.5)
        assertTrue(hits.isNotEmpty(), "expected at least one recall hit")
        assertTrue(
            hits[0].similarity > 0.9,
            "expected top hit similarity > 0.9, got ${hits[0].similarity}",
        )
    }

    @Test
    fun deleteRemovesItem() {
        val store = openMemory()
        val id = store.put("alpha", "a".encodeToByteArray())
        assertEquals(1, store.size())
        assertTrue(store.delete(id))
        assertEquals(0, store.size())
        assertFalse(store.delete(id))
    }

    @Test
    fun dimensionReportsConfiguredValue() {
        val store = openMemory(dim = 4096)
        assertEquals(4096, store.dimension())
    }

    @Test
    fun consolidateAndPersistAreSafeNoOps() {
        val store = openMemory()
        store.put("alpha", "a".encodeToByteArray())
        store.put("beta", "b".encodeToByteArray())
        store.recall("alpha", topK = 2, minSim = 0.0)
        // Both should run without throwing — memory adapter has nothing
        // to persist and the consolidation pass is exercised on a small
        // substrate purely for smoke coverage.
        store.consolidate()
        store.persist()
        assertEquals(2, store.size())
    }

    @Test
    fun sqliteFactoryThrowsOnBrowser() {
        assertFails {
            PersistentStore.openSqlite("/tmp/unused.smr", 4096)
        }
    }

    @Test
    fun fileFactoryThrowsOnBrowser() {
        assertFails {
            PersistentStore.openFile("/tmp/unused.smr", 4096)
        }
    }

    @Test
    fun closeIsIdempotent() {
        val store = openMemory()
        store.close()
        // Second close must not throw.
        store.close()
    }
}
