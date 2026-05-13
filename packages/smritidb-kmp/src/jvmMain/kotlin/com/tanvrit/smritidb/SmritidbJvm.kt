package com.tanvrit.smritidb

import uniffi.smritidb.SmritidbException as UniffiException
import uniffi.smritidb.Store as UniffiStore
import uniffi.smritidb.bind as uniffiBind
import uniffi.smritidb.bundle as uniffiBundle
import uniffi.smritidb.encodeEmbedding as uniffiEncodeEmbedding
import uniffi.smritidb.encodeString as uniffiEncodeString
import uniffi.smritidb.permute as uniffiPermute
import uniffi.smritidb.randomHv as uniffiRandomHv
import uniffi.smritidb.similarity as uniffiSimilarity
import uniffi.smritidb.unbind as uniffiUnbind

/**
 * JVM `actual`: delegates to the UniFFI-generated Kotlin bindings.
 *
 * The UniFFI output (packages/smritidb-ffi/bindings/kotlin/) is checked into
 * `src/jvmMain/kotlin/uniffi/smritidb/smritidb.kt` as a generated artifact,
 * and the JNA-loadable native lib `libsmritidb_ffi.{dylib,so,dll}` must be
 * on `java.library.path` at runtime.
 *
 * On Android Native targets, the same UniFFI output is consumed via the
 * `androidNativeArm64Main` / `androidNativeX64Main` source sets.
 */

public actual fun openStore(dimension: UInt): SmritidbStore = JvmStore(UniffiStore(dimension))

private class JvmStore(private val inner: UniffiStore) : SmritidbStore {
    override val dimension: UInt get() = inner.dimension()
    override val specVersion: String get() = inner.specVersion()

    override fun put(cue: Cue, value: ByteArray, tags: List<String>): String =
        when (cue) {
            is Cue.Text -> inner.put(cue.value, value, tags)
            is Cue.Vector -> error("Vector cues require dimension match; use Text cue in this binding")
            is Cue.Embedding -> error("Embedding cues are routed through encodeEmbedding(); pre-encode and use Vector cue")
        }

    override fun recall(cue: Cue, topK: UInt, minSimilarity: Double): List<Match> {
        val key = when (cue) {
            is Cue.Text -> cue.value
            is Cue.Vector -> error("Vector cues not yet supported via Kotlin recall")
            is Cue.Embedding -> error("Embedding cues not yet supported via Kotlin recall")
        }
        return inner.recall(key, topK, minSimilarity).map { m ->
            Match(
                id = m.id,
                similarity = m.similarity,
                value = m.value,
                tags = m.tags,
                accessCount = m.accessCount,
            )
        }
    }

    override fun delete(id: String): Boolean = inner.delete(id)
    override fun size(): UInt = inner.size()

    override fun close() {
        // UniFFI handles destructor via the Cleaner; nothing to do explicitly.
    }
}

public actual fun randomHv(seed: ByteArray, dim: UInt): Hypervector =
    uniffiRandomHv(seed, dim)

public actual fun similarity(a: Hypervector, b: Hypervector): Double =
    uniffiSimilarity(a, b)

public actual fun bind(a: Hypervector, b: Hypervector): Hypervector =
    uniffiBind(a, b)

public actual fun unbind(a: Hypervector, b: Hypervector): Hypervector =
    uniffiUnbind(a, b)

public actual fun bundle(hvs: List<Hypervector>): Hypervector =
    uniffiBundle(hvs)

public actual fun permute(hv: Hypervector, k: Int): Hypervector =
    uniffiPermute(hv, k)

public actual fun encodeString(s: String, dim: UInt): Hypervector =
    uniffiEncodeString(s, dim)

public actual fun encodeEmbedding(embedding: FloatArray, dim: UInt): Hypervector {
    val list = ArrayList<Float>(embedding.size)
    for (f in embedding) list.add(f)
    return uniffiEncodeEmbedding(list, dim)
}

@Suppress("unused")
private fun translateUniffiException(e: UniffiException): SmritidbException = when (e) {
    is UniffiException.DimensionMismatch -> SmritidbException.DimensionMismatch()
    is UniffiException.InvalidConfig -> SmritidbException.InvalidConfig("see Rust core")
    is UniffiException.NotFound -> SmritidbException.NotFound("?")
    is UniffiException.ValueTooLarge -> SmritidbException.ValueTooLarge()
    is UniffiException.EmptyInput -> SmritidbException.EmptyInput()
}
