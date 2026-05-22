package com.tanvrit.smritidb

import uniffi.smritidb.PersistentStore as UniffiPersistentStore
import uniffi.smritidb.StoreOptions as UniffiStoreOptions

/**
 * JVM (and Android) `actual` for [PersistentStore].
 *
 * Delegates to the UniFFI-generated `uniffi.smritidb.PersistentStore`, which
 * in turn calls into the Rust core via JNA. The companion factories all
 * funnel into UniFFI's `openSqlite` / `openFile` / `openMemory` constructors
 * — the same surface exposed from the Swift / Python bindings.
 *
 * No `sqlite-jdbc`, no Room, no SQLDelight: SQLite is the bundled `rusqlite`
 * build linked into `libsmritidb_ffi`.
 */
public actual class PersistentStore internal constructor(
    private val inner: UniffiPersistentStore,
) {
    public actual companion object {
        public actual fun openSqlite(
            path: String,
            dimension: Int,
        ): PersistentStore =
            PersistentStore(UniffiPersistentStore.openSqlite(path, storeOptions(dimension)))

        public actual fun openFile(
            path: String,
            dimension: Int,
        ): PersistentStore =
            PersistentStore(UniffiPersistentStore.openFile(path, storeOptions(dimension)))

        public actual fun openMemory(
            dimension: Int,
        ): PersistentStore =
            PersistentStore(UniffiPersistentStore.openMemory(storeOptions(dimension)))

        private fun storeOptions(dimension: Int): UniffiStoreOptions =
            UniffiStoreOptions(
                dimension = dimension.toUInt(),
                // `valueCapBytes`, `defaultTopK`, `defaultMinSimilarity` of zero
                // tell the Rust shim to substitute the core defaults — see
                // `StoreOptions::to_core` in `packages/smritidb-ffi/src/lib.rs`.
                valueCapBytes = 0u,
                defaultTopK = 0u,
                defaultMinSimilarity = 0.0,
            )
    }

    public actual fun put(
        key: String,
        value: ByteArray,
        tags: List<String>,
    ): String = inner.put(key, value, tags, /* metadata = */ null)

    public actual fun recall(
        cue: String,
        topK: Int,
        minSim: Double,
    ): List<RecallMatch> =
        inner.recall(cue, topK.toUInt(), minSim).map { m ->
            RecallMatch(id = m.id, similarity = m.similarity, tags = m.tags)
        }

    public actual fun delete(id: String): Boolean = inner.delete(id)

    public actual fun size(): Int = inner.size().toInt()

    public actual fun dimension(): Int = inner.dimension().toInt()

    public actual fun consolidate() {
        inner.consolidate()
    }

    public actual fun persist() {
        inner.persist()
    }

    public actual fun close() {
        // `inner.close()` is the patched `AutoCloseable.close()` from the
        // UniFFI binding — it now also runs the FFI-side `closeFfi()` to
        // release the adapter before tearing down the handle.
        inner.close()
    }
}
