package com.tanvrit.smritidb

import kotlinx.coroutines.await
import kotlin.js.Promise

/**
 * Browser-storage persistence adapter for the **Kotlin/Wasm**
 * `PersistentStore`. Mirrors the Kotlin/JS adapter
 * (`jsMain/.../IndexedDbAdapter.kt`) — same v1 schema, same shape, same
 * `suspend` surface — but driven through `@JsFun` arrow-function bridges
 * because Kotlin/Wasm can't use `@JsModule`.
 *
 * ## Schema (matches the K/JS adapter; same DB files are interchangeable)
 *
 *   * **`smritidb_snapshot`** — out-of-line keyed store holding the KMF
 *     snapshot blob under the constant key `1`.
 *   * **`smritidb_wal`** — placeholder auto-increment store reserved
 *     for the future append-only WAL (Phase E).
 *
 * ## Bytes across the K/Wasm ↔ JS boundary
 *
 * Kotlin/Wasm cannot transfer typed-array views directly across the
 * `@JsFun` boundary as fields of long-lived Kotlin objects. We ferry
 * binary payloads (KMF snapshots, hypervector bytes) as base64 strings:
 * one cross-boundary call per direction regardless of payload size. The
 * decode/encode happens via Node's `Buffer` when available (~native
 * speed) and falls back to `atob` / `btoa` in browser hosts. A 10 KB
 * snapshot round-trips in ~0.1 ms each way, vs. ~5 ms for the prior hex
 * path.
 *
 * The `IDBObjectStore.put` / `IDBRequest.result` themselves traffic in
 * real `Uint8Array` instances — base64 is only the *wire format* between
 * Kotlin and the JS bridge. IDB sees and stores the same typed-array
 * bytes it would in the K/JS adapter, so a DB file written by one target
 * can be read by the other.
 *
 * ## Async wiring
 *
 * Every async IDB op exposes a `Promise<...>` to Kotlin; `Promise.await()`
 * from `kotlinx.coroutines` resumes the suspend function when JS resolves
 * the promise from `req.onsuccess` / `tx.oncomplete`. We avoid
 * `@JsExport`-based callbacks because exported Kotlin/Wasm symbols live
 * on the module's `exports` object, not `globalThis` — promise return
 * values cross the `@JsFun` boundary natively and need no special wiring.
 */
internal class IndexedDbAdapterWasmJs private constructor(
    private val dbHandle: Int,
) {
    /** Read the snapshot blob at key `1`, or `null` if never persisted. */
    internal suspend fun read(): ByteArray? {
        val resultB64 = idbReadAsPromise(dbHandle, SNAPSHOT_STORE, SNAPSHOT_KEY).await<JsString?>()
        return resultB64?.toString()?.let { decodeBase64(it) }
    }

    /** Write (or overwrite) the snapshot blob at key `1`. Idempotent. */
    internal suspend fun write(bytes: ByteArray) {
        idbWriteAsPromise(dbHandle, SNAPSHOT_STORE, SNAPSHOT_KEY, encodeBase64(bytes)).await<JsBoolean>()
    }

    /** Delete the snapshot row and any WAL entries. Tests-only convenience. */
    @Suppress("unused")
    internal suspend fun clear() {
        idbClearAsPromise(dbHandle, SNAPSHOT_STORE, WAL_STORE).await<JsBoolean>()
    }

    /** Close the underlying `IDBDatabase`. Idempotent. */
    internal fun close() {
        idbCloseDatabase(dbHandle)
    }

    internal companion object {
        internal const val SCHEMA_VERSION: Int = 1
        internal const val SNAPSHOT_STORE: String = "smritidb_snapshot"
        internal const val WAL_STORE: String = "smritidb_wal"
        internal const val SNAPSHOT_KEY: Int = 1

        /** Open (or create) `dbName` at v1; resume with a new adapter. */
        internal suspend fun open(dbName: String): IndexedDbAdapterWasmJs {
            idbEnsureBridge()
            val dbId = idbOpenAsPromise(dbName, SCHEMA_VERSION, SNAPSHOT_STORE, WAL_STORE)
                .await<JsNumber>().toInt()
            return IndexedDbAdapterWasmJs(dbId)
        }
    }
}

// --- IDB JS bridge -----------------------------------------------------
//
// Each `@JsFun` is self-contained — Kotlin/Wasm can't share local JS
// helpers across separate arrow-function closures. We stash mutable
// state (the open-database map, base64 helpers) on
// `globalThis.SmritidbIdbBridge`, set up lazily by `idbEnsureBridge`.
// The wiring is structurally the same as wasm-bindgen's handle protocol:
// integer handles in, Promise<integer/string?> out.

@JsFun(
    """() => {
        if (globalThis.SmritidbIdbBridge) return;
        const bridge = {
            dbs: new Map(),
            nextDbId: 1,
        };
        // Base64 helpers — used for the wasm ↔ JS byte ferry. `Buffer`
        // is native in Node; in browsers we fall back to `atob`/`btoa`.
        bridge.b64Encode = (u8) => {
            if (typeof Buffer !== 'undefined') {
                return Buffer.from(u8).toString('base64');
            }
            let out = '';
            const CHUNK = 0x8000;
            for (let i = 0; i < u8.length; i += CHUNK) {
                out += String.fromCharCode.apply(null, u8.subarray(i, i + CHUNK));
            }
            return btoa(out);
        };
        bridge.b64Decode = (s) => {
            if (typeof Buffer !== 'undefined') {
                const b = Buffer.from(s, 'base64');
                // Buffer is a Uint8Array subclass; copy into a plain
                // Uint8Array so IDB structured-clone doesn't trip on it.
                return new Uint8Array(b.buffer, b.byteOffset, b.byteLength).slice();
            }
            const bin = atob(s);
            const u8 = new Uint8Array(bin.length);
            for (let i = 0; i < bin.length; i++) u8[i] = bin.charCodeAt(i);
            return u8;
        };
        globalThis.SmritidbIdbBridge = bridge;
    }""",
)
internal external fun idbEnsureBridge()

@JsFun(
    """(dbName, version, snapshotStore, walStore) => {
        const bridge = globalThis.SmritidbIdbBridge;
        return new Promise((resolve, reject) => {
            const req = globalThis.indexedDB.open(dbName, version);
            req.onupgradeneeded = () => {
                const db = req.result;
                const names = db.objectStoreNames;
                let hasSnapshot = false;
                let hasWal = false;
                for (let i = 0; i < names.length; i++) {
                    if (names.item(i) === snapshotStore) hasSnapshot = true;
                    if (names.item(i) === walStore) hasWal = true;
                }
                if (!hasSnapshot) db.createObjectStore(snapshotStore);
                if (!hasWal) db.createObjectStore(walStore, { autoIncrement: true });
            };
            req.onsuccess = () => {
                const dbId = bridge.nextDbId++;
                bridge.dbs.set(dbId, req.result);
                resolve(dbId);
            };
            req.onerror = () => {
                reject(new Error('IndexedDB open failed: ' + ((req.error && req.error.message) || 'unknown')));
            };
            req.onblocked = () => {
                reject(new Error('IndexedDB open blocked (older schema held by another tab)'));
            };
        });
    }""",
)
internal external fun idbOpenAsPromise(
    dbName: String,
    version: Int,
    snapshotStore: String,
    walStore: String,
): Promise<JsNumber>

@JsFun(
    """(dbId, storeName, key) => {
        const bridge = globalThis.SmritidbIdbBridge;
        return new Promise((resolve, reject) => {
            const db = bridge.dbs.get(dbId);
            if (!db) {
                reject(new Error('database handle ' + dbId + ' is closed'));
                return;
            }
            const tx = db.transaction([storeName], 'readonly');
            const store = tx.objectStore(storeName);
            const req = store.get(key);
            req.onsuccess = () => {
                const raw = req.result;
                if (raw == null) resolve(null);
                // IDB returns a Uint8Array; encode to base64 so we
                // pay one cross-boundary call regardless of size.
                else resolve(bridge.b64Encode(raw));
            };
            req.onerror = () => {
                reject(new Error('IndexedDB read failed: ' + ((req.error && req.error.message) || 'unknown')));
            };
        });
    }""",
)
internal external fun idbReadAsPromise(
    dbId: Int,
    storeName: String,
    key: Int,
): Promise<JsString?>

@JsFun(
    """(dbId, storeName, key, payloadBase64) => {
        const bridge = globalThis.SmritidbIdbBridge;
        return new Promise((resolve, reject) => {
            const db = bridge.dbs.get(dbId);
            if (!db) {
                reject(new Error('database handle ' + dbId + ' is closed'));
                return;
            }
            const tx = db.transaction([storeName], 'readwrite');
            const store = tx.objectStore(storeName);
            const u8 = bridge.b64Decode(payloadBase64);
            const req = store.put(u8, key);
            // Resolve on tx.oncomplete so callers see a durably committed
            // write — same semantics the K/JS adapter has.
            tx.oncomplete = () => resolve(true);
            const onErr = () => reject(new Error(
                'IndexedDB write failed: ' +
                ((req.error && req.error.message) || (tx.error && tx.error.message) || 'unknown'),
            ));
            req.onerror = onErr;
            tx.onerror = onErr;
            tx.onabort = onErr;
        });
    }""",
)
internal external fun idbWriteAsPromise(
    dbId: Int,
    storeName: String,
    key: Int,
    payloadBase64: String,
): Promise<JsBoolean>

@JsFun(
    """(dbId, snapshotStore, walStore) => {
        const bridge = globalThis.SmritidbIdbBridge;
        return new Promise((resolve, reject) => {
            const db = bridge.dbs.get(dbId);
            if (!db) {
                reject(new Error('database handle ' + dbId + ' is closed'));
                return;
            }
            const tx = db.transaction([snapshotStore, walStore], 'readwrite');
            tx.objectStore(snapshotStore).clear();
            const r = tx.objectStore(walStore).clear();
            tx.oncomplete = () => resolve(true);
            const onErr = () => reject(new Error(
                'IndexedDB clear failed: ' +
                ((r.error && r.error.message) || (tx.error && tx.error.message) || 'unknown'),
            ));
            r.onerror = onErr;
            tx.onerror = onErr;
            tx.onabort = onErr;
        });
    }""",
)
internal external fun idbClearAsPromise(
    dbId: Int,
    snapshotStore: String,
    walStore: String,
): Promise<JsBoolean>

@JsFun(
    """(dbId) => {
        const bridge = globalThis.SmritidbIdbBridge;
        const db = bridge.dbs.get(dbId);
        if (db) {
            db.close();
            bridge.dbs.delete(dbId);
        }
    }""",
)
internal external fun idbCloseDatabase(dbId: Int)

// --- base64 helpers (Kotlin side) --------------------------------------

@OptIn(kotlin.io.encoding.ExperimentalEncodingApi::class)
private fun encodeBase64(bytes: ByteArray): String =
    kotlin.io.encoding.Base64.encode(bytes)

@OptIn(kotlin.io.encoding.ExperimentalEncodingApi::class)
private fun decodeBase64(s: String): ByteArray = kotlin.io.encoding.Base64.decode(s)
