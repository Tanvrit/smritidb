package com.tanvrit.smritidb

import org.khronos.webgl.Uint8Array

/**
 * JS (Kotlin/JS IR) `actual` for the primitives + `openStore` declared in
 * `commonMain/.../Smritidb.kt`. All forwarding to the wasm-bindgen build
 * goes through the externals in `SmritidbCoreJs.kt`.
 *
 * `openStore` returns a `SmritidbStore` whose `put` / `recall` accept only
 * `Cue.Text` — `Cue.Vector` / `Cue.Embedding` would require a richer
 * wasm-side `store_put_hv` / `store_put_embedding` shim that we haven't
 * exposed yet. The JVM and Native bindings have the same restriction in
 * spirit (they delegate to the same `put_string` / `recall_string`).
 */

public actual fun openStore(dimension: UInt): SmritidbStore = JsStore(dimension.toInt())

private class JsStore(private val dimInt: Int) : SmritidbStore {
    private val handle: Int = storeOpenMemory(dimInt)
    private var closed = false

    override val dimension: UInt get() = dimInt.toUInt()
    override val specVersion: String get() = specVersion()

    override fun put(cue: Cue, value: ByteArray, tags: List<String>): String {
        require(!closed) { "store is closed" }
        val key = when (cue) {
            is Cue.Text -> cue.value
            is Cue.Vector -> error("Vector cues are not yet plumbed through the wasm bridge")
            is Cue.Embedding -> error("Embedding cues are not yet plumbed through the wasm bridge")
        }
        // Wasm shim doesn't carry tags yet (see PersistentStoreJs); enforce
        // here so the API surface fails loudly.
        require(tags.isEmpty()) { "tags are not yet plumbed through the wasm bridge (Phase D)" }
        return storePut(handle, key, value.toU8())
    }

    override fun recall(cue: Cue, topK: UInt, minSimilarity: Double): List<Match> {
        require(!closed) { "store is closed" }
        val key = when (cue) {
            is Cue.Text -> cue.value
            is Cue.Vector -> error("Vector cues are not yet plumbed through the wasm bridge")
            is Cue.Embedding -> error("Embedding cues are not yet plumbed through the wasm bridge")
        }
        val raw = storeRecall(handle, key, topK.toInt(), minSimilarity)
        @Suppress("UNCHECKED_CAST_TO_EXTERNAL_INTERFACE")
        val arr = raw as Array<dynamic>
        return arr.map { entry ->
            @Suppress("UnsafeCastFromDynamic")
            val tagsRaw = entry.tags as Array<String>
            // `entry.value` lifts through serde-wasm-bindgen as a
            // `Uint8Array`. Accept it via `dynamic` to dodge Kotlin/JS's
            // strict cast in `as Uint8Array` (it sometimes mistakes the
            // wasm-bindgen wrapper for a plain `Array`).
            val valueDyn = entry.value
            val valueArr = uint8ArrayFromDynamic(valueDyn)
            Match(
                id = entry.id as String,
                similarity = (entry.similarity as Number).toDouble(),
                value = valueArr.toByteArray(),
                tags = tagsRaw.toList(),
                accessCount = (entry.accessCount as Number).toInt().toUInt(),
            )
        }
    }

    override fun delete(id: String): Boolean {
        require(!closed) { "store is closed" }
        return storeDelete(handle, id)
    }

    override fun size(): UInt {
        require(!closed) { "store is closed" }
        return storeSize(handle).toUInt()
    }

    override fun close() {
        if (closed) return
        storeClose(handle)
        closed = true
    }
}

// --- primitives ---------------------------------------------------------

public actual fun randomHv(seed: ByteArray, dim: UInt): Hypervector =
    randomHv(seed.toU8(), dim.toInt()).toByteArray()

public actual fun similarity(a: Hypervector, b: Hypervector): Double =
    similarity(a.toU8(), b.toU8())

public actual fun bind(a: Hypervector, b: Hypervector): Hypervector =
    bind(a.toU8(), b.toU8()).toByteArray()

public actual fun unbind(a: Hypervector, b: Hypervector): Hypervector =
    unbind(a.toU8(), b.toU8()).toByteArray()

public actual fun bundle(hvs: List<Hypervector>): Hypervector {
    require(hvs.isNotEmpty()) { "bundle: hvs is empty" }
    val dim = hvs[0].size
    val flat = ByteArray(dim * hvs.size)
    for ((i, hv) in hvs.withIndex()) {
        require(hv.size == dim) { "bundle: ragged input" }
        hv.copyInto(flat, destinationOffset = i * dim)
    }
    return bundle(flat.toU8(), dim).toByteArray()
}

public actual fun permute(hv: Hypervector, k: Int): Hypervector =
    permute(hv.toU8(), k).toByteArray()

public actual fun encodeString(s: String, dim: UInt): Hypervector =
    encodeString(s, dim.toInt()).toByteArray()

public actual fun encodeEmbedding(embedding: FloatArray, dim: UInt): Hypervector {
    // `wasm.rs` does not yet expose `encodeEmbedding` (the TypeScript
    // reference encodes on the JS side and feeds the resulting HV in via
    // `put`). We synthesise the result locally so the API still works:
    // call `randomHv` once with the first embedding element to seed a
    // canonical key and let upstream callers refine the wiring in
    // Phase D. For now this is a stub that should not be hit by the
    // smoke tests — they only exercise text cues.
    throw NotImplementedError(
        "encodeEmbedding is not yet exposed via the wasm bridge; pre-encode " +
            "in JS or wait for Phase D.",
    )
}

private fun ByteArray.toU8(): Uint8Array {
    val out = Uint8Array(this.size)
    for (i in this.indices) {
        out.asDynamic()[i] = this[i].toInt() and 0xff
    }
    return out
}

// `Uint8Array.toByteArray()` lives in `PersistentStoreJs.kt` (internal,
// same package) — reuse it rather than duplicating.

/**
 * `serde-wasm-bindgen` sometimes lifts a `Vec<u8>` as a JS Array of
 * numbers rather than a Uint8Array, depending on host JS engine and
 * serializer config. Normalise either shape to `Uint8Array` so the
 * downstream `.toByteArray()` extension works.
 */
private fun uint8ArrayFromDynamic(v: dynamic): Uint8Array {
    if (v is Uint8Array) return v
    // `Array.isArray`-style check via dynamic — Kotlin can't pattern-
    // match against arbitrary JS arrays.
    val isJsArray = js("Array.isArray")(v) as Boolean
    if (isJsArray) {
        val len = (v.length as Number).toInt()
        val out = Uint8Array(len)
        for (i in 0 until len) {
            out.asDynamic()[i] = ((v[i] as Number).toInt() and 0xff)
        }
        return out
    }
    // Fall through — treat as Uint8Array-like (has .length + indexing).
    @Suppress("UNCHECKED_CAST_TO_EXTERNAL_INTERFACE")
    return v as Uint8Array
}
