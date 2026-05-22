package com.tanvrit.smritidb

import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.launch
import org.khronos.webgl.Uint8Array
import org.khronos.webgl.get

/**
 * JS (Kotlin/JS IR) `actual` for [PersistentStore].
 *
 * Backed by the wasm-bindgen build from `packages/core-rs/pkg-node/`
 * (Node-flavoured CommonJS) — same Rust core as JVM / Native, just
 * exercised through `wasm-bindgen` glue rather than UniFFI.
 *
 * **Memory + IndexedDB adapters.** `openSqlite` and `openFile` throw
 * `UnsupportedOperationException`: rusqlite (and `std::fs`) do not
 * compile to `wasm32-unknown-unknown`. The Phase D `openIndexedDb`
 * factory (added as a JS-only `suspend` companion method below) writes
 * KMF snapshots to IndexedDB via `IndexedDbAdapter` for durable
 * browser-side storage. OPFS-sqlite-wasm is the Phase E follow-up — see
 * `README.md` and the project's `roadmap.md`.
 *
 * ## Handle protocol
 *
 * `wasm-bindgen` can't ferry an owned `&mut Store` across the boundary,
 * so the Rust crate's `wasm.rs` exposes an integer-handle API: the
 * `Store` lives in a process-global `HashMap<u32, Store>` and every
 * call passes its handle as the first argument. The Kotlin `actual`
 * here is a thin façade that holds the handle and forwards calls.
 *
 * The handle is single-use: once `close()` is called the store entry
 * is dropped from the Rust-side map and subsequent operations fail
 * with a `JsError` ("invalid store handle: …"). Callers should wrap
 * their use in `try { ... } finally { store.close() }`.
 */
public actual class PersistentStore internal constructor(
    private var handle: Int,
    private val dim: Int,
    private val idb: IndexedDbAdapter? = null,
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
            val h = storeOpenMemory(dimension)
            return PersistentStore(h, dimension)
        }

        /**
         * Open a `PersistentStore` whose KMF snapshot is stored in
         * IndexedDB under the given `dbName`. The store itself lives in
         * wasm (memory adapter); each `persist()` snapshots the wasm
         * substrate and writes it to IndexedDB. Each new
         * `openIndexedDb(dbName, ...)` reads the existing snapshot back
         * in, so the contents survive page reloads.
         *
         * **Browser-only.** This factory is part of the JS / WasmJs
         * surface; the JVM / Native targets do not expose it (they have
         * `openSqlite` / `openFile` instead). Common code that needs the
         * shared surface should depend on `openMemory` (every target) or
         * on the platform-specific factories directly.
         */
        public suspend fun openIndexedDb(
            dbName: String,
            dimension: Int = 10_000,
        ): PersistentStore {
            val adapter = IndexedDbAdapter.open(dbName)
            val handle = storeOpenMemory(dimension)
            val existing = adapter.read()
            // `existing` is `null` on a fresh database. On a re-open we
            // restore the wasm substrate from the prior snapshot — the
            // restored KMF's dimension overrides the configured one, so
            // a `dimension` mismatch (e.g. opening a 4096-dim db with
            // `dimension = 8192`) silently adopts the snapshot's value.
            if (existing != null) {
                storeRestoreFromSnapshot(handle, existing)
            }
            return PersistentStore(handle, dimension, adapter)
        }
    }

    public actual fun put(key: String, value: ByteArray, tags: List<String>): String {
        require(tags.isEmpty()) {
            // The wasm `store_put` shim doesn't yet expose tags — the same
            // simplification the TypeScript reference makes. When the
            // browser-storage adapters land we'll route through a richer
            // FFI surface; for now, keep the call site honest.
            "tags are not yet plumbed through the wasm bridge (Phase D)"
        }
        return storePut(handle, key, value.toUint8Array())
    }

    public actual fun recall(cue: String, topK: Int, minSim: Double): List<RecallMatch> {
        val raw = storeRecall(handle, cue, topK, minSim)
        // `raw` is a JS array of `{ id, similarity, tags }` objects.
        val arr = raw as Array<dynamic>
        return arr.map { entry ->
            @Suppress("UnsafeCastFromDynamic")
            val tagsRaw = entry.tags as Array<String>
            RecallMatch(
                id = entry.id as String,
                similarity = (entry.similarity as Number).toDouble(),
                tags = tagsRaw.toList(),
            )
        }
    }

    public actual fun delete(id: String): Boolean = storeDelete(handle, id)

    public actual fun size(): Int = storeSize(handle)

    public actual fun dimension(): Int = dim

    public actual fun consolidate() {
        storeConsolidate(handle)
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
        val snapshot = storeSnapshot(handle)
        // Fire-and-forget. Tests and durability-sensitive callers should
        // prefer `persistAsync()` so the IDB transaction is observably
        // committed before the next read.
        kotlinx.coroutines.GlobalScope.launch {
            idb.write(snapshot)
        }
    }

    /**
     * IndexedDB-aware variant of [persist] that awaits the underlying
     * `IDBTransaction`'s commit. Equivalent to [persist] on a memory-only
     * store. **JS-only** — not on the common surface.
     */
    public suspend fun persistAsync() {
        val adapter = idb ?: return
        val snapshot = storeSnapshot(handle)
        adapter.write(snapshot)
    }

    public actual fun close() {
        if (handle == 0) return
        storeClose(handle)
        handle = 0
        idb?.close()
    }

    /**
     * Snapshot through the adapter (if any) and then [close]. Use this
     * over the synchronous [close] when working against IndexedDB —
     * `close()` alone does not await the final write. **JS-only.**
     */
    public suspend fun closeAsync() {
        if (handle == 0) return
        if (idb != null) {
            val snapshot = storeSnapshot(handle)
            idb.write(snapshot)
        }
        storeClose(handle)
        handle = 0
        idb?.close()
    }
}

private fun ByteArray.toUint8Array(): Uint8Array {
    val out = Uint8Array(this.size)
    for (i in this.indices) {
        out.asDynamic()[i] = this[i].toInt() and 0xff
    }
    return out
}

internal fun Uint8Array.toByteArray(): ByteArray {
    val out = ByteArray(this.length)
    for (i in 0 until this.length) {
        out[i] = this[i]
    }
    return out
}
