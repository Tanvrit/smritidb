package com.tanvrit.smritidb

import kotlin.test.AfterTest
import kotlin.test.BeforeTest
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFails
import kotlin.test.assertFalse
import kotlin.test.assertTrue

/**
 * Kotlin/Wasm smoke tests — same shape as `PersistentStoreJsTest`, but
 * runs against the wasm-bindgen ESM bundle loaded via the test
 * bootstrap below.
 *
 * Loads `globalThis.SmritidbWasm` once per test class so each test can
 * call into the wasm bridge as if it were a normal Kotlin function.
 */
class PersistentStoreWasmJsTest {
    private val stores = mutableListOf<PersistentStore>()

    @BeforeTest
    fun bootstrap() {
        ensureWasmBootstrapped()
    }

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
        store.close()
    }
}
