@file:OptIn(kotlinx.cinterop.ExperimentalForeignApi::class)

package com.tanvrit.smritidb

import kotlinx.cinterop.COpaquePointer
import smritidb_ffi.uniffi_smritidb_ffi_fn_clone_store
import smritidb_ffi.uniffi_smritidb_ffi_fn_constructor_store_new
import smritidb_ffi.uniffi_smritidb_ffi_fn_free_store
import smritidb_ffi.uniffi_smritidb_ffi_fn_func_bind
import smritidb_ffi.uniffi_smritidb_ffi_fn_func_bundle
import smritidb_ffi.uniffi_smritidb_ffi_fn_func_encode_embedding
import smritidb_ffi.uniffi_smritidb_ffi_fn_func_encode_string
import smritidb_ffi.uniffi_smritidb_ffi_fn_func_permute
import smritidb_ffi.uniffi_smritidb_ffi_fn_func_random_hv
import smritidb_ffi.uniffi_smritidb_ffi_fn_func_similarity
import smritidb_ffi.uniffi_smritidb_ffi_fn_func_unbind
import smritidb_ffi.uniffi_smritidb_ffi_fn_method_store_delete
import smritidb_ffi.uniffi_smritidb_ffi_fn_method_store_dimension
import smritidb_ffi.uniffi_smritidb_ffi_fn_method_store_put
import smritidb_ffi.uniffi_smritidb_ffi_fn_method_store_recall
import smritidb_ffi.uniffi_smritidb_ffi_fn_method_store_size
import smritidb_ffi.uniffi_smritidb_ffi_fn_method_store_spec_version

/**
 * Kotlin/Native actuals for the rest of the commonMain surface in
 * `Smritidb.kt` — the in-memory `SmritidbStore` plus the hypervector
 * primitive functions. All of them call into `libsmritidb_ffi`'s
 * UniFFI C ABI; the shared lift/lower helpers live alongside the
 * `PersistentStore` actual.
 *
 * Source-shared across Apple (`iosArm64`, `iosX64`, `iosSimulatorArm64`,
 * `macosArm64`, `macosX64`), Android Native (`androidNativeArm64`,
 * `androidNativeX64`), and Linux (`linuxX64`, `linuxArm64`) — every
 * target whose `*Main` depends on `ffiNativeMain`.
 *
 * The functions named here mirror the `uniffi_smritidb_ffi_fn_func_*`
 * symbol family — they're free functions in the Rust core. The
 * `SmritidbStore` returned by [openStore] wraps the
 * `uniffi_smritidb_ffi_fn_constructor_store_new` constructor and the
 * matching `fn_method_store_*` methods.
 */

public actual fun openStore(dimension: UInt): SmritidbStore =
    NativeStore(
        callPointer { status ->
            uniffi_smritidb_ffi_fn_constructor_store_new(dimension, status)
        },
        dimension,
    )

private class NativeStore(
    private var handle: COpaquePointer?,
    private val dimensionConfig: UInt,
) : SmritidbStore {
    override val dimension: UInt
        get() = callUInt { status ->
            uniffi_smritidb_ffi_fn_method_store_dimension(cloneHandle(), status)
        }

    override val specVersion: String
        get() = liftString(
            callRustBuffer { status ->
                uniffi_smritidb_ffi_fn_method_store_spec_version(cloneHandle(), status)
            },
        )

    override fun put(cue: Cue, value: ByteArray, tags: List<String>): String {
        val keyString = when (cue) {
            is Cue.Text -> cue.value
            is Cue.Vector ->
                throw SmritidbException.InvalidConfig(
                    "Vector cues require dimension match; use Text cue in this binding",
                )
            is Cue.Embedding ->
                throw SmritidbException.InvalidConfig(
                    "Embedding cues are routed through encodeEmbedding(); " +
                        "pre-encode and use Vector cue",
                )
        }
        val buf = callRustBuffer { status ->
            uniffi_smritidb_ffi_fn_method_store_put(
                cloneHandle(),
                rustBufferFromUtf8(keyString),
                rustBufferFromBytes(value),
                lowerStringList(tags),
                status,
            )
        }
        return liftString(buf)
    }

    override fun recall(cue: Cue, topK: UInt, minSimilarity: Double): List<Match> {
        val keyString = when (cue) {
            is Cue.Text -> cue.value
            is Cue.Vector ->
                throw SmritidbException.InvalidConfig("Vector cues not yet supported")
            is Cue.Embedding ->
                throw SmritidbException.InvalidConfig("Embedding cues not yet supported")
        }
        val buf = callRustBuffer { status ->
            uniffi_smritidb_ffi_fn_method_store_recall(
                cloneHandle(),
                rustBufferFromUtf8(keyString),
                topK,
                minSimilarity,
                status,
            )
        }
        return liftMatchListFull(buf)
    }

    override fun delete(id: String): Boolean {
        val ptr = cloneHandle()
        return callByte { status ->
            uniffi_smritidb_ffi_fn_method_store_delete(
                ptr,
                rustBufferFromUtf8(id),
                status,
            )
        } != 0.toByte()
    }

    override fun size(): UInt = callUInt { status ->
        uniffi_smritidb_ffi_fn_method_store_size(cloneHandle(), status)
    }

    override fun close() {
        val ptr = handle ?: return
        try {
            callVoid { status ->
                uniffi_smritidb_ffi_fn_free_store(ptr, status)
            }
        } finally {
            handle = null
        }
        // dimensionConfig is just a debug crutch — not used post-close.
        @Suppress("UNUSED_EXPRESSION")
        dimensionConfig
    }

    /**
     * UniFFI methods take ownership of the pointer they receive. We
     * clone the cached `handle` before every call so the cached
     * pointer survives across calls — matching `uniffiClonePointer()`
     * in the Swift binding.
     */
    private fun cloneHandle(): COpaquePointer {
        val ptr = handle
            ?: throw SmritidbException.InvalidConfig("Store has already been closed")
        return callPointer { status ->
            uniffi_smritidb_ffi_fn_clone_store(ptr, status)
        }
    }
}

// ---------------------------------------------------------------------------
// Hypervector primitives (top-level UniFFI functions).
//
// Each takes/returns `Vec<u8>` (a top-level `RustBuffer` of raw bytes)
// or a primitive scalar; sequences require the framed binary protocol
// just like in `PersistentStoreApple.kt`.
// ---------------------------------------------------------------------------

public actual fun randomHv(seed: ByteArray, dim: UInt): Hypervector {
    val buf = callRustBuffer { status ->
        uniffi_smritidb_ffi_fn_func_random_hv(
            rustBufferFromBytes(seed),
            dim,
            status,
        )
    }
    return liftBytes(buf)
}

public actual fun similarity(a: Hypervector, b: Hypervector): Double =
    callDouble { status ->
        uniffi_smritidb_ffi_fn_func_similarity(
            rustBufferFromBytes(a),
            rustBufferFromBytes(b),
            status,
        )
    }

public actual fun bind(a: Hypervector, b: Hypervector): Hypervector =
    liftBytes(
        callRustBuffer { status ->
            uniffi_smritidb_ffi_fn_func_bind(
                rustBufferFromBytes(a),
                rustBufferFromBytes(b),
                status,
            )
        },
    )

public actual fun unbind(a: Hypervector, b: Hypervector): Hypervector =
    liftBytes(
        callRustBuffer { status ->
            uniffi_smritidb_ffi_fn_func_unbind(
                rustBufferFromBytes(a),
                rustBufferFromBytes(b),
                status,
            )
        },
    )

public actual fun bundle(hvs: List<Hypervector>): Hypervector =
    liftBytes(
        callRustBuffer { status ->
            uniffi_smritidb_ffi_fn_func_bundle(
                lowerByteArrayList(hvs),
                status,
            )
        },
    )

public actual fun permute(hv: Hypervector, k: Int): Hypervector =
    liftBytes(
        callRustBuffer { status ->
            uniffi_smritidb_ffi_fn_func_permute(
                rustBufferFromBytes(hv),
                k,
                status,
            )
        },
    )

public actual fun encodeString(s: String, dim: UInt): Hypervector =
    liftBytes(
        callRustBuffer { status ->
            uniffi_smritidb_ffi_fn_func_encode_string(
                rustBufferFromUtf8(s),
                dim,
                status,
            )
        },
    )

public actual fun encodeEmbedding(embedding: FloatArray, dim: UInt): Hypervector =
    liftBytes(
        callRustBuffer { status ->
            uniffi_smritidb_ffi_fn_func_encode_embedding(
                lowerFloatArray(embedding),
                dim,
                status,
            )
        },
    )
