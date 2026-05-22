@file:OptIn(kotlin.io.encoding.ExperimentalEncodingApi::class)

package com.tanvrit.smritidb

import kotlinx.coroutines.launch

/**
 * Kotlin/Wasm (`wasmJs` target) `actual` for [PersistentStore].
 *
 * Backed by the same wasm-bindgen output from `packages/core-rs/pkg/`
 * (browser ESM flavour) — Kotlin/Wasm can't use `@JsModule` so we drive
 * the binding through `@JsFun` calls that assume the host has set
 * `globalThis.SmritidbWasm` to the wasm-bindgen module before any Kotlin
 * code runs. The test harness in `wasmJsTest/.../Bootstrap.kt`
 * uses a small bootstrap helper to load the module via Node's
 * `require()` and assigns it to `globalThis.SmritidbWasm`.
 *
 * Kotlin/Wasm JS interop is stricter than Kotlin/JS — typed-array types
 * (`Uint8Array`) cannot be retained as long-lived Kotlin fields without
 * an explicit `JsReference` wrapper. We ferry `ByteArray` payloads as
 * base64 strings (one cross-boundary call per call site, regardless of
 * payload size — `SmritidbWasmJs.kt` and `IndexedDbAdapterWasmJs.kt`).
 * IDB itself still stores real `Uint8Array` instances, so DB files are
 * interoperable with the K/JS adapter.
 *
 * **Memory + IndexedDB adapters.** `openSqlite` / `openFile` throw
 * `UnsupportedOperationException`. OPFS-sqlite-wasm remains a Phase E
 * follow-up.
 */
public actual class PersistentStore internal constructor(
    private var handle: Int,
    private val dim: Int,
    private val idb: IndexedDbAdapterWasmJs? = null,
) {
    public actual companion object {
        public actual fun openSqlite(path: String, dimension: Int): PersistentStore {
            throw UnsupportedOperationException(
                "SQLite persistence is not available in browser builds. " +
                    "Use openMemory() for in-process storage, or openIndexedDb() " +
                    "for durable browser storage. OPFS-sqlite-wasm is Phase E.",
            )
        }

        public actual fun openFile(path: String, dimension: Int): PersistentStore {
            throw UnsupportedOperationException(
                "File-system persistence is not available in browser builds. " +
                    "Use openMemory() for in-process storage, or openIndexedDb() " +
                    "for durable browser storage. OPFS-sqlite-wasm is Phase E.",
            )
        }

        public actual fun openMemory(dimension: Int): PersistentStore {
            val h = jsStoreOpenMemory(dimension)
            return PersistentStore(h, dimension)
        }

        /**
         * Open a `PersistentStore` whose KMF snapshot is stored in
         * IndexedDB under the given `dbName`. The store itself lives in
         * wasm (memory adapter); each `persistAsync()` snapshots the
         * wasm substrate and writes it to IndexedDB. Each new
         * `openIndexedDb(dbName, ...)` reads the existing snapshot back
         * in, so the contents survive page reloads.
         *
         * **Browser-only.** This factory is part of the WasmJs (and
         * JS) surface; the JVM / Native targets do not expose it (they
         * have `openSqlite` / `openFile` instead).
         */
        public suspend fun openIndexedDb(
            dbName: String,
            dimension: Int = 10_000,
        ): PersistentStore {
            val adapter = IndexedDbAdapterWasmJs.open(dbName)
            val handle = jsStoreOpenMemory(dimension)
            val existing = adapter.read()
            // `existing` is null on a fresh database. On re-open we
            // restore the wasm substrate from the prior snapshot — the
            // restored KMF's dimension overrides the configured one, so
            // a `dimension` mismatch silently adopts the snapshot's
            // value (matches the K/JS adapter semantics).
            if (existing != null) {
                jsStoreRestoreFromSnapshotBase64(handle, kotlin.io.encoding.Base64.encode(existing))
            }
            return PersistentStore(handle, dimension, adapter)
        }
    }

    public actual fun put(key: String, value: ByteArray, tags: List<String>): String {
        require(tags.isEmpty()) {
            "tags are not yet plumbed through the wasm bridge (Phase D)"
        }
        return jsStorePutHex(handle, key, value.toHex())
    }

    public actual fun recall(cue: String, topK: Int, minSim: Double): List<RecallMatch> {
        val json = jsStoreRecallJson(handle, cue, topK, minSim)
        return parseRecallJson(json)
    }

    public actual fun delete(id: String): Boolean = jsStoreDelete(handle, id)

    public actual fun size(): Int = jsStoreSize(handle)

    public actual fun dimension(): Int = dim

    public actual fun consolidate() {
        jsStoreConsolidate(handle)
    }

    @OptIn(kotlinx.coroutines.DelicateCoroutinesApi::class)
    public actual fun persist() {
        // The non-suspend surface can't await an IndexedDB transaction.
        // For a pure in-memory store there's nothing to do; for an IDB-
        // backed store, callers must drive the async write via
        // `persistAsync()` and `closeAsync()` below — the sync `persist`
        // is retained for source-compatibility with the JVM / Native
        // actuals and is a best-effort fire-and-forget on the IDB path.
        if (idb == null) return
        val snapshotB64 = jsStoreSnapshotBase64(handle)
        val snapshot = kotlin.io.encoding.Base64.decode(snapshotB64)
        kotlinx.coroutines.GlobalScope.launch {
            idb.write(snapshot)
        }
    }

    /**
     * IndexedDB-aware variant of [persist] that awaits the underlying
     * `IDBTransaction`'s commit. Equivalent to [persist] on a memory-only
     * store. **WasmJs-only** — not on the common surface.
     */
    public suspend fun persistAsync() {
        val adapter = idb ?: return
        val snapshotB64 = jsStoreSnapshotBase64(handle)
        adapter.write(kotlin.io.encoding.Base64.decode(snapshotB64))
    }

    public actual fun close() {
        if (handle == 0) return
        jsStoreClose(handle)
        handle = 0
        idb?.close()
    }

    /**
     * Snapshot through the adapter (if any) and then [close]. Use this
     * over the synchronous [close] when working against IndexedDB —
     * `close()` alone does not await the final write. **WasmJs-only.**
     */
    public suspend fun closeAsync() {
        if (handle == 0) return
        if (idb != null) {
            val snapshotB64 = jsStoreSnapshotBase64(handle)
            idb.write(kotlin.io.encoding.Base64.decode(snapshotB64))
        }
        jsStoreClose(handle)
        handle = 0
        idb?.close()
    }
}

/** Parses the JSON returned by `jsStoreRecallJson` into Kotlin objects. */
private fun parseRecallJson(s: String): List<RecallMatch> {
    val trimmed = s.trim()
    if (trimmed == "[]") return emptyList()
    val inner = trimmed.removePrefix("[").removeSuffix("]")
    val results = mutableListOf<RecallMatch>()
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
                if (depth == 0) results += parseSingleRecord(inner.substring(start, i + 1))
            }
        }
    }
    return results
}

private fun parseSingleRecord(rec: String): RecallMatch {
    val idMatch = Regex("\"id\"\\s*:\\s*\"([^\"]*)\"").find(rec)
    val simMatch = Regex("\"similarity\"\\s*:\\s*([0-9eE.+-]+)").find(rec)
    val tagsMatch = Regex("\"tags\"\\s*:\\s*\\[([^\\]]*)\\]").find(rec)
    val id = idMatch?.groupValues?.get(1) ?: ""
    val sim = simMatch?.groupValues?.get(1)?.toDouble() ?: 0.0
    val tagsRaw = tagsMatch?.groupValues?.get(1)?.trim().orEmpty()
    val tags = if (tagsRaw.isEmpty()) emptyList()
    else tagsRaw.split(",").map { it.trim().removePrefix("\"").removeSuffix("\"") }
    return RecallMatch(id = id, similarity = sim, tags = tags)
}
