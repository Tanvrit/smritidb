package com.tanvrit.smritidb

import kotlinx.coroutines.suspendCancellableCoroutine
import org.khronos.webgl.Uint8Array
import kotlin.coroutines.resume
import kotlin.coroutines.resumeWithException

/**
 * Browser-storage persistence adapter for the Kotlin/JS `PersistentStore`.
 *
 * Backs Phase D's `PersistentStore.openIndexedDb(dbName, dimension)`. The
 * shape is deliberately minimal:
 *
 *   * a single object store, **`smritidb_snapshot`**, holding the KMF
 *     snapshot blob under the constant key `1`;
 *   * a v2 object store, **`smritidb_wal`**, reserved for future
 *     append-only WAL entries — created at upgrade time so we don't have
 *     to bump the schema version when we start writing to it, but
 *     unused in v1 (snapshot-only is enough for a first cut and matches
 *     the in-memory adapter's semantics from the Rust core's tests).
 *
 * Why a hand-rolled binding rather than `kotlinx-browser`?
 * `kotlinx-browser` is a prerelease wrapper that does not yet expose
 * IDB v3 ergonomically across Kotlin/JS and Kotlin/Wasm; rolling a thin
 * `external` surface keeps the dep count at zero and lets us share the
 * exact same shape with the Kotlin/Wasm side later (Phase E).
 *
 * The IDB API is callback-based; each suspend method wraps a single
 * `IDBRequest` in `suspendCancellableCoroutine` so callers see a clean
 * coroutine API.
 *
 * ## Architecture
 *
 * The adapter lives **entirely on the Kotlin/JS side** — it does NOT
 * push through wasm. The flow is:
 *
 *   1. The memory-only `Store` runs in wasm (existing handle protocol).
 *   2. On `persist()`, Kotlin calls `storeSnapshot(handle) -> Uint8Array`
 *      to get the KMF blob, then writes it to IndexedDB via this
 *      adapter's `write()`.
 *   3. On `openIndexedDb(dbName, dimension)`, Kotlin reads the existing
 *      snapshot via `read()`, calls `storeRestoreFromSnapshot(handle,
 *      bytes)` to populate the wasm Store, then returns a
 *      `PersistentStore` whose `persist()` writes back.
 */
internal class IndexedDbAdapter private constructor(
    private val db: IDBDatabase,
) {
    /**
     * Read the snapshot blob at key `1`, or `null` if the store has
     * never been persisted to before.
     */
    internal suspend fun read(): Uint8Array? = suspendCancellableCoroutine { cont ->
        val tx = db.transaction(arrayOf(SNAPSHOT_STORE), "readonly")
        val store = tx.objectStore(SNAPSHOT_STORE)
        val req = store.get(SNAPSHOT_KEY)
        req.onsuccess = {
            // `req.result` is `undefined` when the key is missing.
            val raw: dynamic = req.result
            if (raw == null || raw == undefined) {
                cont.resume(null)
            } else {
                @Suppress("UnsafeCastFromDynamic")
                cont.resume(raw as Uint8Array)
            }
        }
        req.onerror = {
            cont.resumeWithException(
                IllegalStateException("IndexedDB read failed: ${req.error?.message ?: "unknown"}"),
            )
        }
    }

    /**
     * Write (or overwrite) the snapshot blob at key `1`. Idempotent —
     * subsequent calls replace the prior value.
     */
    internal suspend fun write(bytes: Uint8Array): Unit = suspendCancellableCoroutine { cont ->
        val tx = db.transaction(arrayOf(SNAPSHOT_STORE), "readwrite")
        val store = tx.objectStore(SNAPSHOT_STORE)
        val req = store.put(bytes, SNAPSHOT_KEY)
        req.onsuccess = { cont.resume(Unit) }
        req.onerror = {
            cont.resumeWithException(
                IllegalStateException("IndexedDB write failed: ${req.error?.message ?: "unknown"}"),
            )
        }
    }

    /**
     * Delete the snapshot row and any WAL entries. Used by tests; not on
     * the production hot path.
     */
    @Suppress("unused")
    internal suspend fun clear(): Unit = suspendCancellableCoroutine { cont ->
        val tx = db.transaction(arrayOf(SNAPSHOT_STORE, WAL_STORE), "readwrite")
        val snapStore = tx.objectStore(SNAPSHOT_STORE)
        val walStore = tx.objectStore(WAL_STORE)
        snapStore.clear()
        val req = walStore.clear()
        req.onsuccess = { cont.resume(Unit) }
        req.onerror = {
            cont.resumeWithException(
                IllegalStateException("IndexedDB clear failed: ${req.error?.message ?: "unknown"}"),
            )
        }
    }

    /** Close the underlying `IDBDatabase`. Idempotent. */
    internal fun close() {
        db.close()
    }

    internal companion object {
        internal const val SCHEMA_VERSION: Int = 1
        internal const val SNAPSHOT_STORE: String = "smritidb_snapshot"
        internal const val WAL_STORE: String = "smritidb_wal"
        internal const val SNAPSHOT_KEY: Int = 1

        /**
         * Open (or create) the database `dbName` with the v1 schema —
         * an out-of-line keyed snapshot store plus a placeholder
         * autoincrement WAL store. Caches no global state; callers are
         * responsible for `close()`.
         */
        internal suspend fun open(dbName: String): IndexedDbAdapter =
            suspendCancellableCoroutine { cont ->
                val req = indexedDB.open(dbName, SCHEMA_VERSION)
                req.onupgradeneeded = {
                    val db = req.result
                    val storeNames = db.objectStoreNames
                    if (!containsName(storeNames, SNAPSHOT_STORE)) {
                        // No `keyPath` — keys are supplied out-of-line at
                        // `put` time. We use a single constant key (1) so
                        // there's at most one row per database.
                        db.createObjectStore(SNAPSHOT_STORE)
                    }
                    if (!containsName(storeNames, WAL_STORE)) {
                        // Reserved for Phase E append-only WAL. We create
                        // it at v1 so the schema bump is a no-op when WAL
                        // writes land.
                        val opts = js("({ autoIncrement: true })")
                        db.createObjectStore(WAL_STORE, opts)
                    }
                }
                req.onsuccess = {
                    cont.resume(IndexedDbAdapter(req.result))
                }
                req.onerror = {
                    cont.resumeWithException(
                        IllegalStateException(
                            "IndexedDB open failed for '$dbName': ${req.error?.message ?: "unknown"}",
                        ),
                    )
                }
                req.onblocked = {
                    cont.resumeWithException(
                        IllegalStateException(
                            "IndexedDB open blocked for '$dbName' (another tab holds an older schema)",
                        ),
                    )
                }
            }

        /**
         * `DOMStringList` doesn't have a Kotlin-friendly `contains`;
         * iterate by index. The list is short (max one or two entries)
         * so the linear scan is irrelevant.
         */
        private fun containsName(list: DOMStringList, name: String): Boolean {
            for (i in 0 until list.length) {
                if (list.item(i) == name) return true
            }
            return false
        }
    }
}

// --- external IndexedDB surface ----------------------------------------
//
// These mirror the standard
// [`IndexedDB` API](https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API).
// Kotlin's `kotlinx-browser` is a moving target right now; the surface
// we need is small enough that a hand-rolled `external` is easier to
// audit. Kotlin/JS resolves these against the host `globalThis`
// (browser native, or `fake-indexeddb/auto`'s polyfill installed at
// test time).

@Suppress("ClassName")
internal external object indexedDB {
    fun open(name: String, version: Int): IDBOpenDBRequest
    fun deleteDatabase(name: String): IDBOpenDBRequest
}

internal external interface IDBRequest {
    val result: dynamic
    val error: DomException?
    var onsuccess: (dynamic) -> Unit
    var onerror: (dynamic) -> Unit
}

internal external interface IDBOpenDBRequest : IDBRequest {
    override var onsuccess: (dynamic) -> Unit
    override var onerror: (dynamic) -> Unit
    var onupgradeneeded: (dynamic) -> Unit
    var onblocked: (dynamic) -> Unit
    override val result: IDBDatabase
}

internal external interface IDBDatabase {
    val objectStoreNames: DOMStringList
    fun transaction(storeNames: Array<String>, mode: String): IDBTransaction
    fun createObjectStore(name: String): IDBObjectStore
    fun createObjectStore(name: String, options: dynamic): IDBObjectStore
    fun close()
}

internal external interface IDBTransaction {
    fun objectStore(name: String): IDBObjectStore
}

internal external interface IDBObjectStore {
    fun get(key: dynamic): IDBRequest
    fun put(value: dynamic, key: dynamic): IDBRequest
    fun clear(): IDBRequest
}

internal external interface DOMStringList {
    val length: Int
    fun item(index: Int): String?
}

internal external interface DomException {
    val message: String
}
