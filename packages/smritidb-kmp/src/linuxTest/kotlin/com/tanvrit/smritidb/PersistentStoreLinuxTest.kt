package com.tanvrit.smritidb

import kotlin.test.Test
import kotlin.test.assertEquals

/**
 * Linux-native smoke test for the [PersistentStore] Memory adapter.
 *
 * The shared bridge in `src/ffiNativeMain/.../` is identical for
 * Apple, Android Native, and Linux — the underlying `libsmritidb_ffi.a`
 * varies only by Rust cross-compile triple. This test exists so the
 * `linuxX64Test` and `linuxArm64Test` Gradle tasks have at least one
 * Linux-specific assertion to execute, complementing the broader
 * coverage shared via `ffiNativeTest`.
 */
class PersistentStoreLinuxTest {
    @Test
    fun memoryStoreRoundTripOnLinuxNative() {
        val store = PersistentStore.openMemory()
        try {
            store.put(key = "alpha", value = "hello".encodeToByteArray())
            assertEquals(1, store.size())
        } finally {
            store.close()
        }
    }
}
