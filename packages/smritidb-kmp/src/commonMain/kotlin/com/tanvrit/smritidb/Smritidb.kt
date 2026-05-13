package com.tanvrit.smritidb

/**
 * Smritidb — an open biology-inspired associative memory layer.
 *
 * This is the Kotlin Multiplatform public surface. The actual implementation
 * is provided per-target by an `actual` declaration that wraps either the
 * UniFFI-generated bindings (JVM / Android Native / Linux / Apple) or the
 * WebAssembly build (JS / WasmJs).
 *
 * The semantics mirror the SPEC.md §3 contract exactly — any divergence is
 * a binding bug, not a spec decision.
 */

public const val SPEC_VERSION: String = "0.1.0-draft"

/** A binary hypervector, packed one byte per bit. */
public typealias Hypervector = ByteArray

/** A semantic key — string, embedding, or pre-computed hypervector. */
public sealed interface Cue {
    public data class Text(val value: String) : Cue
    public data class Embedding(val floats: FloatArray) : Cue {
        override fun equals(other: Any?): Boolean =
            other is Embedding && floats.contentEquals(other.floats)
        override fun hashCode(): Int = floats.contentHashCode()
    }
    public data class Vector(val bytes: Hypervector) : Cue
}

/** A recall result: an item + its similarity to the cue. */
public data class Match(
    public val id: String,
    public val similarity: Double,
    public val value: ByteArray,
    public val tags: List<String>,
    public val accessCount: UInt,
) {
    override fun equals(other: Any?): Boolean =
        other is Match &&
            id == other.id &&
            similarity == other.similarity &&
            value.contentEquals(other.value) &&
            tags == other.tags &&
            accessCount == other.accessCount

    override fun hashCode(): Int {
        var r = id.hashCode()
        r = 31 * r + similarity.hashCode()
        r = 31 * r + value.contentHashCode()
        r = 31 * r + tags.hashCode()
        r = 31 * r + accessCount.hashCode()
        return r
    }
}

public sealed class SmritidbException(message: String) : RuntimeException(message) {
    public class DimensionMismatch : SmritidbException("dimension mismatch")
    public class InvalidConfig(reason: String) : SmritidbException("invalid config: $reason")
    public class NotFound(id: String) : SmritidbException("item not found: $id")
    public class ValueTooLarge : SmritidbException("value exceeds the configured cap")
    public class EmptyInput : SmritidbException("empty input")
}

/**
 * Open a Smritidb store. Each platform's `actual` opens the native
 * substrate (JVM/Android/Apple/Linux via UniFFI; JS/Wasm via the
 * smritidb_core wasm bundle).
 */
public expect fun openStore(dimension: UInt = 10_000u): SmritidbStore

public interface SmritidbStore : AutoCloseable {
    public val dimension: UInt
    public val specVersion: String

    public fun put(cue: Cue, value: ByteArray, tags: List<String> = emptyList()): String
    public fun recall(
        cue: Cue,
        topK: UInt = 10u,
        minSimilarity: Double = 0.5,
    ): List<Match>

    public fun delete(id: String): Boolean
    public fun size(): UInt
}

// Primitive operations — same across platforms via expect/actual.
public expect fun randomHv(seed: ByteArray, dim: UInt): Hypervector
public expect fun similarity(a: Hypervector, b: Hypervector): Double
public expect fun bind(a: Hypervector, b: Hypervector): Hypervector
public expect fun unbind(a: Hypervector, b: Hypervector): Hypervector
public expect fun bundle(hvs: List<Hypervector>): Hypervector
public expect fun permute(hv: Hypervector, k: Int): Hypervector
public expect fun encodeString(s: String, dim: UInt): Hypervector
public expect fun encodeEmbedding(embedding: FloatArray, dim: UInt): Hypervector
