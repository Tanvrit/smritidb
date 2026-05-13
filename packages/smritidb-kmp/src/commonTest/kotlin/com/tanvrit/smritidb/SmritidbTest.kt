package com.tanvrit.smritidb

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotEquals
import kotlin.test.assertTrue

class SmritidbTest {
    private val dim: UInt = 8192u

    @Test
    fun similarity_self_equals_one() {
        val a = randomHv("hello".encodeToByteArray(), dim)
        assertEquals(1.0, similarity(a, a))
    }

    @Test
    fun similarity_random_pair_near_half() {
        val a = randomHv("a".encodeToByteArray(), dim)
        val b = randomHv("b".encodeToByteArray(), dim)
        val s = similarity(a, b)
        assertTrue(s in 0.45..0.55, "expected ~0.5, got $s")
    }

    @Test
    fun bind_is_self_inverse() {
        val a = randomHv("a".encodeToByteArray(), dim)
        val b = randomHv("b".encodeToByteArray(), dim)
        val round = unbind(bind(a, b), b)
        assertEquals(1.0, similarity(a, round))
    }

    @Test
    fun randomHv_is_deterministic() {
        val a = randomHv("seed".encodeToByteArray(), dim)
        val b = randomHv("seed".encodeToByteArray(), dim)
        val c = randomHv("other".encodeToByteArray(), dim)
        assertEquals(a.toList(), b.toList())
        assertNotEquals(a.toList(), c.toList())
    }

    @Test
    fun store_put_and_recall_round_trip() {
        val store = openStore(dim)
        store.put(Cue.Text("the cat sat on the mat"), "the cat sat on the mat".encodeToByteArray())
        val hits = store.recall(Cue.Text("the cat sat on the mat"), topK = 1u, minSimilarity = 0.9)
        assertEquals(1, hits.size)
        assertEquals("the cat sat on the mat", hits[0].value.decodeToString())
        assertEquals(1.0, hits[0].similarity)
    }

    @Test
    fun store_delete_removes_item() {
        val store = openStore(dim)
        val id = store.put(Cue.Text("alpha"), "alpha".encodeToByteArray())
        assertEquals(1u, store.size())
        assertEquals(true, store.delete(id))
        assertEquals(0u, store.size())
    }

    @Test
    fun spec_version_is_reachable() {
        val store = openStore(dim)
        assertTrue(store.specVersion.startsWith("0."))
    }
}
