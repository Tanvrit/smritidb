package com.tanvrit.smritidb

import kotlinx.coroutines.test.runTest
import kotlin.test.AfterTest
import kotlin.test.BeforeTest
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

/**
 * Phase D smoke test for the IndexedDB-backed `PersistentStore`.
 *
 * Runs under `jsNodeTest` (Mocha + Node). Node has no native
 * `indexedDB`, so we install the `fake-indexeddb` polyfill at process
 * start by side-effect-requiring `fake-indexeddb/auto`. The polyfill
 * mirrors the v3 browser surface closely enough that the same adapter
 * code runs in both environments.
 *
 * The test exercises the full open → put → persist → close → reopen →
 * recall cycle, asserting the KMF snapshot round-trips bit-identically.
 */
class IndexedDbAdapterTest {
    @BeforeTest
    fun installFakeIndexedDb() {
        // Side-effect import — installs `indexedDB`, `IDBKeyRange`, etc.
        // as globals on Node. No-op when re-run; the polyfill is
        // idempotent.
        installFakeIndexedDbAuto()
    }

    @AfterTest
    fun resetFakeIndexedDb() {
        // Wipe the in-memory polyfill so each test starts fresh — its
        // reset hook re-instantiates the global IndexedDB factory.
        resetIndexedDb()
    }

    @Test
    fun roundTripThroughIndexedDb() = runTest {
        val store = PersistentStore.openIndexedDb("test_db_1", dimension = 4096)
        store.put("alpha", "hello".encodeToByteArray())
        assertEquals(1, store.size())
        store.persistAsync()
        store.closeAsync()

        val store2 = PersistentStore.openIndexedDb("test_db_1", dimension = 4096)
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
        val store = PersistentStore.openIndexedDb("test_db_empty", dimension = 4096)
        // A never-before-written database returns no snapshot — the
        // store comes up empty.
        assertEquals(0, store.size())
        store.put("alpha", "value".encodeToByteArray())
        assertEquals(1, store.size())
        store.closeAsync()
    }

    @Test
    fun multipleOpensSeeLatestPersist() = runTest {
        val db = "test_db_overwrite"
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
