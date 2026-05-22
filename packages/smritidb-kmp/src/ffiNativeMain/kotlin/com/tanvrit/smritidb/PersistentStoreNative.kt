@file:OptIn(kotlinx.cinterop.ExperimentalForeignApi::class)

package com.tanvrit.smritidb

import kotlinx.cinterop.COpaquePointer
import smritidb_ffi.uniffi_smritidb_ffi_fn_clone_persistentstore
import smritidb_ffi.uniffi_smritidb_ffi_fn_constructor_persistentstore_open_file
import smritidb_ffi.uniffi_smritidb_ffi_fn_constructor_persistentstore_open_memory
import smritidb_ffi.uniffi_smritidb_ffi_fn_constructor_persistentstore_open_sqlite
import smritidb_ffi.uniffi_smritidb_ffi_fn_free_persistentstore
import smritidb_ffi.uniffi_smritidb_ffi_fn_method_persistentstore_close
import smritidb_ffi.uniffi_smritidb_ffi_fn_method_persistentstore_consolidate
import smritidb_ffi.uniffi_smritidb_ffi_fn_method_persistentstore_delete
import smritidb_ffi.uniffi_smritidb_ffi_fn_method_persistentstore_dimension
import smritidb_ffi.uniffi_smritidb_ffi_fn_method_persistentstore_persist
import smritidb_ffi.uniffi_smritidb_ffi_fn_method_persistentstore_put
import smritidb_ffi.uniffi_smritidb_ffi_fn_method_persistentstore_recall
import smritidb_ffi.uniffi_smritidb_ffi_fn_method_persistentstore_size

/**
 * Kotlin/Native actual for [PersistentStore], shared across every
 * Kotlin/Native target that links `libsmritidb_ffi.a`:
 *
 *   * Apple — `iosArm64`, `iosX64`, `iosSimulatorArm64`, `macosArm64`,
 *     `macosX64`.
 *   * Android Native — `androidNativeArm64`, `androidNativeX64`.
 *   * Linux — `linuxX64`, `linuxArm64` (compile + cinterop wired;
 *     linking the test executable needs a Rust archive for the
 *     Linux triple, which today is built only on a Linux CI runner).
 *
 * UniFFI 0.28 generates a Swift binding but no Kotlin/Native binding.
 * We cinterop against the generated C ABI (`smritidbFFI.h`) directly
 * and reimplement the bits of UniFFI's lift/lower protocol the
 * `PersistentStore` surface needs. The shared helpers live in
 * `UniffiBridge.kt` alongside this file.
 *
 * The constructed [PersistentStore] owns the opaque Rust handle. We do
 * NOT take an extra `uniffi_*_fn_clone_persistentstore` reference —
 * `close()` calls `uniffi_*_fn_free_persistentstore` exactly once. If
 * the caller forgets to `close()` we leak the handle (no JVM-style
 * cleaner here); the test harness always closes explicitly via
 * `try { ... } finally { store.close() }`.
 */
public actual class PersistentStore internal constructor(
    private var handle: COpaquePointer?,
) {
    public actual companion object {
        public actual fun openSqlite(path: String, dimension: Int): PersistentStore =
            PersistentStore(
                callPointer { status ->
                    uniffi_smritidb_ffi_fn_constructor_persistentstore_open_sqlite(
                        rustBufferFromUtf8(path),
                        lowerStoreOptions(dimension),
                        status,
                    )
                },
            )

        public actual fun openFile(path: String, dimension: Int): PersistentStore =
            PersistentStore(
                callPointer { status ->
                    uniffi_smritidb_ffi_fn_constructor_persistentstore_open_file(
                        rustBufferFromUtf8(path),
                        lowerStoreOptions(dimension),
                        status,
                    )
                },
            )

        public actual fun openMemory(dimension: Int): PersistentStore =
            PersistentStore(
                callPointer { status ->
                    uniffi_smritidb_ffi_fn_constructor_persistentstore_open_memory(
                        lowerStoreOptions(dimension),
                        status,
                    )
                },
            )
    }

    public actual fun put(key: String, value: ByteArray, tags: List<String>): String {
        val buf = callRustBuffer { status ->
            uniffi_smritidb_ffi_fn_method_persistentstore_put(
                cloneHandle(),
                rustBufferFromUtf8(key),
                rustBufferFromBytes(value),
                lowerStringList(tags),
                // metadata is Option<String> — lower as None.
                lowerNoneString(),
                status,
            )
        }
        return liftString(buf)
    }

    public actual fun recall(cue: String, topK: Int, minSim: Double): List<RecallMatch> {
        val buf = callRustBuffer { status ->
            uniffi_smritidb_ffi_fn_method_persistentstore_recall(
                cloneHandle(),
                rustBufferFromUtf8(cue),
                topK.toUInt(),
                minSim,
                status,
            )
        }
        return liftMatchList(buf)
    }

    public actual fun delete(id: String): Boolean =
        callByte { status ->
            uniffi_smritidb_ffi_fn_method_persistentstore_delete(
                cloneHandle(),
                rustBufferFromUtf8(id),
                status,
            )
        } != 0.toByte()

    public actual fun size(): Int =
        callUInt { status ->
            uniffi_smritidb_ffi_fn_method_persistentstore_size(cloneHandle(), status)
        }.toInt()

    public actual fun dimension(): Int =
        callUInt { status ->
            uniffi_smritidb_ffi_fn_method_persistentstore_dimension(cloneHandle(), status)
        }.toInt()

    public actual fun consolidate() {
        callUInt { status ->
            uniffi_smritidb_ffi_fn_method_persistentstore_consolidate(cloneHandle(), status)
        }
    }

    public actual fun persist() {
        callVoid { status ->
            uniffi_smritidb_ffi_fn_method_persistentstore_persist(cloneHandle(), status)
        }
    }

    /**
     * Releases adapter resources (closes SQLite, flushes the WAL, …)
     * and frees the Rust-side handle. Idempotent — second and
     * subsequent calls are no-ops, matching JVM `Closeable` semantics.
     */
    public actual fun close() {
        val ptr = handle ?: return
        try {
            // Run the FFI `close()` method first — adapters release
            // resources (closes SQLite, flushes the WAL, …) here.
            // The method takes a cloned pointer and decrements its
            // Arc; the original `ptr` we cached at construction
            // remains alive until `_fn_free_persistentstore` below.
            callVoid { status ->
                uniffi_smritidb_ffi_fn_method_persistentstore_close(cloneOf(ptr), status)
            }
        } finally {
            callVoid { status ->
                uniffi_smritidb_ffi_fn_free_persistentstore(ptr, status)
            }
            handle = null
        }
    }

    private fun requireHandle(): COpaquePointer =
        handle ?: throw PersistentStoreException(
            "PersistentStore has already been closed",
        )

    /**
     * UniFFI 0.28 methods take ownership of the pointer they receive
     * (they decrement the Arc when the call returns). To preserve the
     * cached `handle` across calls, every method must pass a freshly
     * cloned pointer. This mirrors `uniffiClonePointer()` in the
     * Swift binding.
     */
    private fun cloneHandle(): COpaquePointer = cloneOf(requireHandle())

    private fun cloneOf(ptr: COpaquePointer): COpaquePointer =
        callPointer { status ->
            uniffi_smritidb_ffi_fn_clone_persistentstore(ptr, status)
        }
}
