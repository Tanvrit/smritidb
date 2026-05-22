package com.tanvrit.smritidb

/**
 * Kotlin/Wasm `actual` for the primitives + `openStore` declared in
 * `commonMain/.../Smritidb.kt`.
 *
 * All forwarding goes through `@JsFun`-declared external bridges that
 * assume `globalThis.SmritidbWasm` has been seeded by the host before
 * any Kotlin code runs (see `wasmJsTest/.../Bootstrap.kt`).
 *
 * Kotlin/Wasm JS interop is stricter than Kotlin/JS: typed-array types
 * (`Uint8Array`, `Float32Array`) can't be retained as long-lived Kotlin
 * fields without an explicit `JsReference` wrapper. The primitives below
 * still ferry small byte payloads as hex strings (a 256-byte HV
 * round-trips in microseconds), while the snapshot path used by the
 * IndexedDB adapter (`jsStoreSnapshotBase64` /
 * `jsStoreRestoreFromSnapshotBase64` below) uses base64 — a single
 * `Buffer.from(b64, 'base64')` cross-boundary call regardless of payload
 * size, ~50× faster than hex for the 10 KB KMF snapshots IDB writes.
 *
 * Same Phase B restrictions as the JS target — text cues only, no tags,
 * no embedding encoding.
 */

public actual fun openStore(dimension: UInt): SmritidbStore = WasmJsStore(dimension.toInt())

private class WasmJsStore(private val dimInt: Int) : SmritidbStore {
    private val handle: Int = jsStoreOpenMemory(dimInt)
    private var closed = false

    override val dimension: UInt get() = dimInt.toUInt()
    override val specVersion: String get() = jsSpecVersion()

    override fun put(cue: Cue, value: ByteArray, tags: List<String>): String {
        require(!closed) { "store is closed" }
        val key = when (cue) {
            is Cue.Text -> cue.value
            is Cue.Vector -> error("Vector cues are not yet plumbed through the wasm bridge")
            is Cue.Embedding -> error("Embedding cues are not yet plumbed through the wasm bridge")
        }
        require(tags.isEmpty()) { "tags are not yet plumbed through the wasm bridge (Phase D)" }
        return jsStorePutHex(handle, key, value.toHex())
    }

    override fun recall(cue: Cue, topK: UInt, minSimilarity: Double): List<Match> {
        require(!closed) { "store is closed" }
        val key = when (cue) {
            is Cue.Text -> cue.value
            is Cue.Vector -> error("Vector cues are not yet plumbed through the wasm bridge")
            is Cue.Embedding -> error("Embedding cues are not yet plumbed through the wasm bridge")
        }
        val json = jsStoreRecallJson(handle, key, topK.toInt(), minSimilarity)
        return parseSimpleRecall(json).map { w ->
            Match(
                id = w.id,
                similarity = w.similarity,
                value = w.valueHex.fromHex(),
                tags = w.tags,
                accessCount = w.accessCount.toUInt(),
            )
        }
    }

    override fun delete(id: String): Boolean {
        require(!closed) { "store is closed" }
        return jsStoreDelete(handle, id)
    }

    override fun size(): UInt {
        require(!closed) { "store is closed" }
        return jsStoreSize(handle).toUInt()
    }

    override fun close() {
        if (closed) return
        jsStoreClose(handle)
        closed = true
    }
}

// --- primitives ---------------------------------------------------------
//
// Each `Hypervector` (= `ByteArray`) ferries across the wasm boundary as
// a hex string. Round-tripping a 10 000-byte HV through hex is on the
// order of 20 µs in Node — acceptable for smoke tests; a Phase D
// improvement would be to surface an ArrayBuffer or fixed-shape JsAny
// interop type.

public actual fun randomHv(seed: ByteArray, dim: UInt): Hypervector =
    jsRandomHvHex(seed.toHex(), dim.toInt()).fromHex()

public actual fun similarity(a: Hypervector, b: Hypervector): Double =
    jsSimilarityHex(a.toHex(), b.toHex())

public actual fun bind(a: Hypervector, b: Hypervector): Hypervector =
    jsBindHex(a.toHex(), b.toHex()).fromHex()

public actual fun unbind(a: Hypervector, b: Hypervector): Hypervector =
    jsUnbindHex(a.toHex(), b.toHex()).fromHex()

public actual fun bundle(hvs: List<Hypervector>): Hypervector {
    require(hvs.isNotEmpty()) { "bundle: hvs is empty" }
    val dim = hvs[0].size
    val flat = ByteArray(dim * hvs.size)
    for ((i, hv) in hvs.withIndex()) {
        require(hv.size == dim) { "bundle: ragged input" }
        hv.copyInto(flat, destinationOffset = i * dim)
    }
    return jsBundleHex(flat.toHex(), dim).fromHex()
}

public actual fun permute(hv: Hypervector, k: Int): Hypervector =
    jsPermuteHex(hv.toHex(), k).fromHex()

public actual fun encodeString(s: String, dim: UInt): Hypervector =
    jsEncodeStringHex(s, dim.toInt()).fromHex()

public actual fun encodeEmbedding(embedding: FloatArray, dim: UInt): Hypervector {
    throw NotImplementedError(
        "encodeEmbedding is not yet exposed via the wasm bridge; pre-encode " +
            "in JS or wait for Phase D.",
    )
}

// --- external bridges --------------------------------------------------
//
// The `hexToU8` / `u8ToHex` helpers in JS are inlined into each bridge so
// we can keep the wasmJs surface to a single arrow function per call.

@JsFun("() => globalThis.SmritidbWasm.specVersion()")
internal external fun jsSpecVersion(): String

@JsFun("(dimension) => globalThis.SmritidbWasm.storeOpenMemory(dimension)")
internal external fun jsStoreOpenMemory(dimension: Int): Int

@JsFun(
    """(handle, key, hex) => {
        const u8 = Uint8Array.from(hex.match(/../g) || [], h => parseInt(h, 16));
        return globalThis.SmritidbWasm.storePut(handle, key, u8);
    }""",
)
internal external fun jsStorePutHex(handle: Int, key: String, valueHex: String): String

@JsFun(
    """(handle, cue, topK, minSim) => {
        const raw = globalThis.SmritidbWasm.storeRecall(handle, cue, topK, minSim);
        // Convert each entry's `value: Uint8Array` to a hex string so we
        // can ferry the bytes across the wasm boundary without losing
        // them to `JSON.stringify`'s object-coercion of typed arrays.
        const out = raw.map(m => ({
            id: m.id,
            similarity: m.similarity,
            valueHex: Array.from(m.value, b => b.toString(16).padStart(2, '0')).join(''),
            tags: m.tags,
            accessCount: m.accessCount,
        }));
        return JSON.stringify(out);
    }""",
)
internal external fun jsStoreRecallJson(
    handle: Int,
    cue: String,
    topK: Int,
    minSim: Double,
): String

@JsFun("(handle, id) => globalThis.SmritidbWasm.storeDelete(handle, id)")
internal external fun jsStoreDelete(handle: Int, id: String): Boolean

@JsFun("(handle) => globalThis.SmritidbWasm.storeSize(handle)")
internal external fun jsStoreSize(handle: Int): Int

@JsFun("(handle) => globalThis.SmritidbWasm.storeDimension(handle)")
internal external fun jsStoreDimension(handle: Int): Int

@JsFun("(handle) => globalThis.SmritidbWasm.storeConsolidate(handle)")
internal external fun jsStoreConsolidate(handle: Int)

@JsFun("(handle) => globalThis.SmritidbWasm.storeClose(handle)")
internal external fun jsStoreClose(handle: Int)

/**
 * Returns the KMF snapshot for `handle` as a base64 string. The
 * wasm-bindgen `storeSnapshot` already returns a `Uint8Array`; we
 * encode JS-side so the bytes cross the K/Wasm boundary in a single
 * call (vs. one per byte for the per-index Uint8Array path).
 */
@JsFun(
    """(handle) => {
        const u8 = globalThis.SmritidbWasm.storeSnapshot(handle);
        if (typeof Buffer !== 'undefined') {
            return Buffer.from(u8).toString('base64');
        }
        let out = '';
        const CHUNK = 0x8000;
        for (let i = 0; i < u8.length; i += CHUNK) {
            out += String.fromCharCode.apply(null, u8.subarray(i, i + CHUNK));
        }
        return btoa(out);
    }""",
)
internal external fun jsStoreSnapshotBase64(handle: Int): String

/**
 * Restore the substrate from a KMF snapshot delivered as a base64
 * string. Decoded JS-side and handed to wasm-bindgen as a plain
 * `Uint8Array`.
 */
@JsFun(
    """(handle, b64) => {
        let u8;
        if (typeof Buffer !== 'undefined') {
            const b = Buffer.from(b64, 'base64');
            u8 = new Uint8Array(b.buffer, b.byteOffset, b.byteLength).slice();
        } else {
            const bin = atob(b64);
            u8 = new Uint8Array(bin.length);
            for (let i = 0; i < bin.length; i++) u8[i] = bin.charCodeAt(i);
        }
        globalThis.SmritidbWasm.storeRestoreFromSnapshot(handle, u8);
    }""",
)
internal external fun jsStoreRestoreFromSnapshotBase64(handle: Int, b64: String)

@JsFun(
    """(seedHex, dim) => {
        const u8 = Uint8Array.from(seedHex.match(/../g) || [], h => parseInt(h, 16));
        const out = globalThis.SmritidbWasm.randomHv(u8, dim);
        return Array.from(out, b => b.toString(16).padStart(2, '0')).join('');
    }""",
)
internal external fun jsRandomHvHex(seedHex: String, dim: Int): String

@JsFun(
    """(aHex, bHex) => {
        const a = Uint8Array.from(aHex.match(/../g) || [], h => parseInt(h, 16));
        const b = Uint8Array.from(bHex.match(/../g) || [], h => parseInt(h, 16));
        return globalThis.SmritidbWasm.similarity(a, b);
    }""",
)
internal external fun jsSimilarityHex(aHex: String, bHex: String): Double

@JsFun(
    """(aHex, bHex) => {
        const a = Uint8Array.from(aHex.match(/../g) || [], h => parseInt(h, 16));
        const b = Uint8Array.from(bHex.match(/../g) || [], h => parseInt(h, 16));
        const out = globalThis.SmritidbWasm.bind(a, b);
        return Array.from(out, x => x.toString(16).padStart(2, '0')).join('');
    }""",
)
internal external fun jsBindHex(aHex: String, bHex: String): String

@JsFun(
    """(aHex, bHex) => {
        const a = Uint8Array.from(aHex.match(/../g) || [], h => parseInt(h, 16));
        const b = Uint8Array.from(bHex.match(/../g) || [], h => parseInt(h, 16));
        const out = globalThis.SmritidbWasm.unbind(a, b);
        return Array.from(out, x => x.toString(16).padStart(2, '0')).join('');
    }""",
)
internal external fun jsUnbindHex(aHex: String, bHex: String): String

@JsFun(
    """(hvsHex, dim) => {
        const hvs = Uint8Array.from(hvsHex.match(/../g) || [], h => parseInt(h, 16));
        const out = globalThis.SmritidbWasm.bundle(hvs, dim);
        return Array.from(out, x => x.toString(16).padStart(2, '0')).join('');
    }""",
)
internal external fun jsBundleHex(hvsHex: String, dim: Int): String

@JsFun(
    """(hvHex, k) => {
        const hv = Uint8Array.from(hvHex.match(/../g) || [], h => parseInt(h, 16));
        const out = globalThis.SmritidbWasm.permute(hv, k);
        return Array.from(out, x => x.toString(16).padStart(2, '0')).join('');
    }""",
)
internal external fun jsPermuteHex(hvHex: String, k: Int): String

@JsFun(
    """(s, dim) => {
        const out = globalThis.SmritidbWasm.encodeString(s, dim);
        return Array.from(out, x => x.toString(16).padStart(2, '0')).join('');
    }""",
)
internal external fun jsEncodeStringHex(s: String, dim: Int): String

// --- helpers -----------------------------------------------------------

private val HEX = "0123456789abcdef".toCharArray()

internal fun ByteArray.toHex(): String {
    val sb = StringBuilder(this.size * 2)
    for (b in this) {
        val v = b.toInt() and 0xff
        sb.append(HEX[v ushr 4])
        sb.append(HEX[v and 0x0f])
    }
    return sb.toString()
}

internal fun String.fromHex(): ByteArray {
    require(this.length % 2 == 0) { "fromHex: odd length" }
    val out = ByteArray(this.length / 2)
    var i = 0
    while (i < this.length) {
        val hi = hexDigit(this[i])
        val lo = hexDigit(this[i + 1])
        out[i / 2] = ((hi shl 4) or lo).toByte()
        i += 2
    }
    return out
}

private fun hexDigit(c: Char): Int = when (c) {
    in '0'..'9' -> c.code - '0'.code
    in 'a'..'f' -> c.code - 'a'.code + 10
    in 'A'..'F' -> c.code - 'A'.code + 10
    else -> throw IllegalArgumentException("not a hex digit: '$c'")
}

/**
 * Lightweight JSON parser for `[{id, similarity, valueHex, tags,
 * accessCount}, ...]` — the projection `jsStoreRecallJson` produces.
 */
internal data class WireMatch(
    val id: String,
    val similarity: Double,
    val valueHex: String,
    val tags: List<String>,
    val accessCount: Int,
)

internal fun parseSimpleRecall(s: String): List<WireMatch> {
    val trimmed = s.trim()
    if (trimmed == "[]") return emptyList()
    val inner = trimmed.removePrefix("[").removeSuffix("]")
    val results = mutableListOf<WireMatch>()
    var depth = 0
    var start = 0
    for (i in inner.indices) {
        when (inner[i]) {
            '{' -> {
                if (depth == 0) start = i
                depth++
            }
            '}' -> {
                depth--
                if (depth == 0) results += parseOne(inner.substring(start, i + 1))
            }
        }
    }
    return results
}

private fun parseOne(rec: String): WireMatch {
    val idMatch = Regex("\"id\"\\s*:\\s*\"([^\"]*)\"").find(rec)
    val simMatch = Regex("\"similarity\"\\s*:\\s*([0-9eE.+-]+)").find(rec)
    val valueMatch = Regex("\"valueHex\"\\s*:\\s*\"([^\"]*)\"").find(rec)
    val tagsMatch = Regex("\"tags\"\\s*:\\s*\\[([^\\]]*)\\]").find(rec)
    val countMatch = Regex("\"accessCount\"\\s*:\\s*([0-9]+)").find(rec)
    val id = idMatch?.groupValues?.get(1) ?: ""
    val sim = simMatch?.groupValues?.get(1)?.toDouble() ?: 0.0
    val valueHex = valueMatch?.groupValues?.get(1) ?: ""
    val tagsRaw = tagsMatch?.groupValues?.get(1)?.trim().orEmpty()
    val tags = if (tagsRaw.isEmpty()) emptyList()
    else tagsRaw.split(",").map { it.trim().removePrefix("\"").removeSuffix("\"") }
    val count = countMatch?.groupValues?.get(1)?.toInt() ?: 0
    return WireMatch(id, sim, valueHex, tags, count)
}
