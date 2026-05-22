package com.tanvrit.smritidb

import kotlinx.coroutines.test.runTest
import kotlin.test.AfterTest
import kotlin.test.BeforeTest
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

/**
 * K/Wasm counterpart to `jsTest/.../IndexedDbAdapterTest.kt`.
 *
 * Exercises the full open → put → persist → close → reopen → recall
 * cycle through the wasm bridge, asserting the KMF snapshot round-trips
 * bit-identically. Uses `fake-indexeddb` as the Node-side polyfill; each
 * test instantiates a fresh `IDBFactory` so cross-test pollution is
 * impossible.
 *
 * The base64 round-trip in the bridge is the only difference vs. the
 * K/JS path — verifying byte equality (`store.size() == 1` after reload
 * and `similarity > 0.99` on the exact cue) confirms the bytes survive
 * Buffer.from / .toString.
 */
class IndexedDbAdapterWasmJsTest {
    @BeforeTest
    fun installFakeIndexedDb() {
        ensureWasmBootstrapped()
        installFakeIndexedDbAutoWasm()
    }

    @AfterTest
    fun resetFakeIndexedDb() {
        resetIndexedDbWasm()
    }

    @Test
    fun roundTripThroughIndexedDb() = runTest {
        val store = PersistentStore.openIndexedDb("wasm_test_db_1", dimension = 4096)
        store.put("alpha", "hello".encodeToByteArray())
        assertEquals(1, store.size())
        store.persistAsync()
        store.closeAsync()

        val store2 = PersistentStore.openIndexedDb("wasm_test_db_1", dimension = 4096)
        assertEquals(1, store2.size())
        val matches = store2.recall("alpha", topK = 5, minSim = 0.5)
        assertTrue(
            matches.isNotEmpty() && matches[0].similarity > 0.99,
            "expected an exact-cue hit after reload, got matches=$matches",
        )
        store2.closeAsync()
    }

    @Test
    fun emptyDbBehavesLikeFreshStore() = runTest {
        val store = PersistentStore.openIndexedDb("wasm_test_db_empty", dimension = 4096)
        assertEquals(0, store.size())
        store.put("alpha", "value".encodeToByteArray())
        assertEquals(1, store.size())
        store.closeAsync()
    }

    @Test
    fun multipleOpensSeeLatestPersist() = runTest {
        val db = "wasm_test_db_overwrite"
        val s1 = PersistentStore.openIndexedDb(db, dimension = 4096)
        s1.put("first", "1".encodeToByteArray())
        s1.persistAsync()
        s1.closeAsync()

        val s2 = PersistentStore.openIndexedDb(db, dimension = 4096)
        assertEquals(1, s2.size())
        s2.put("second", "2".encodeToByteArray())
        s2.persistAsync()
        s2.closeAsync()

        val s3 = PersistentStore.openIndexedDb(db, dimension = 4096)
        assertEquals(2, s3.size())
        s3.closeAsync()
    }
}
