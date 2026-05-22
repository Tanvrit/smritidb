package com.tanvrit.smritidb

import kotlin.test.AfterTest
import kotlin.test.BeforeTest
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue

/**
 * Cross-platform smoke tests for the `PersistentStore` surface.
 *
 * Only exercises the `Memory` adapter — file and sqlite adapters are
 * platform-specific (rusqlite is bundled by the FFI on JVM/Apple/Native;
 * the JS/WasmJs `actual` is restricted to `MemoryAdapter` until Phase C).
 */
class PersistentStoreTest {
    private val stores = mutableListOf<PersistentStore>()

    @BeforeTest
    fun setUp() {
        // No-op on every target except Kotlin/Wasm, which lazy-loads
        // the wasm-bindgen Node module on first use.
        setUpSmritidbTestFixture()
    }

    @AfterTest
    fun cleanup() {
        for (s in stores) {
            runCatching { s.close() }
        }
        stores.clear()
    }

    private fun openMemory(dim: Int = 10_000): PersistentStore {
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
        val matches = store.recall("alpha", topK = 2, minSim = 0.5)
        assertTrue(matches.isNotEmpty(), "expected at least one recall hit")
        // The first hit re-uses the exact same cue string, so similarity must be 1.0.
        assertTrue(
            matches[0].similarity > 0.9,
            "expected top hit similarity > 0.9, got ${matches[0].similarity}",
        )
    }

    @Test
    fun deleteRemovesItem() {
        val store = openMemory()
        val id = store.put("alpha", "alpha".encodeToByteArray())
        assertEquals(1, store.size())
        assertTrue(store.delete(id))
        assertEquals(0, store.size())
        // Idempotent: deleting an unknown id is a no-op.
        assertFalse(store.delete(id))
    }

    @Test
    fun dimensionReportsConfiguredValue() {
        val store = openMemory(dim = 8192)
        assertEquals(8192, store.dimension())
    }

    @Test
    fun persistOnMemoryAdapterIsNoOp() {
        val store = openMemory()
        store.put("alpha", "a".encodeToByteArray())
        // Memory adapter persists into the in-process map; the call must
        // succeed but does not change observable behaviour.
        store.persist()
        assertEquals(1, store.size())
    }

    @Test
    fun consolidateRunsWithoutError() {
        val store = openMemory()
        // Multiple recalls so the co-activation tracker has something to do.
        store.put("alpha", "a".encodeToByteArray())
        store.put("beta", "b".encodeToByteArray())
        store.recall("alpha", topK = 2, minSim = 0.0)
        store.recall("alpha", topK = 2, minSim = 0.0)
        store.consolidate()
    }
}
