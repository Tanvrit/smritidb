package com.tanvrit.smritidb

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.io.TempDir
import java.nio.file.Path
import kotlin.test.assertEquals
import kotlin.test.assertTrue

/**
 * JVM-only round-trip for [PersistentStore.openSqlite].
 *
 * Pins the JVM `actual` to the UniFFI -> Rust `SqliteAdapter` path: write a
 * batch, close, reopen, and confirm both the count and a recall hit survive
 * across the process-lifetime boundary the SqliteAdapter is meant to defend.
 * The cross-platform smoke suite in `commonTest` only covers the Memory
 * adapter, so without this test the JVM SQLite wiring (re)introduced after
 * the in-memory fallback would silently regress.
 */
class PersistentStoreJvmSqliteTest {
    @Test
    fun sqliteRoundTrip(@TempDir tempDir: Path) {
        val path = tempDir.resolve("test.db").toString()
        val store = PersistentStore.openSqlite(path)
        for (i in 0 until 100) store.put("k$i", "v$i".encodeToByteArray())
        store.persist()
        assertEquals(100, store.size())
        store.close()

        val store2 = PersistentStore.openSqlite(path)
        assertEquals(100, store2.size())
        val matches = store2.recall("k50", topK = 5, minSim = 0.5)
        assertTrue(matches.isNotEmpty())
        assertTrue(matches[0].similarity > 0.99)
        store2.close()
    }
}
