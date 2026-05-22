package com.tanvrit.smritidb

/**
 * A durable, biology-inspired associative store backed by one of the
 * persistence adapters in `smritidb_core::persist`:
 *
 *   * `PersistentStore.openSqlite(path)` — a SQLite database storing the
 *     KMF snapshot and the WAL inline.
 *   * `PersistentStore.openFile(path)` — a snapshot at `<path>` with a
 *     side-car WAL at `<path>.wal`.
 *   * `PersistentStore.openMemory()` — an ephemeral in-process store.
 *
 * Each platform's `actual` chooses a different substrate:
 *
 *   * **JVM / Android**: the UniFFI-generated Kotlin bindings calling into
 *     `libsmritidb_ffi` via JNA. SQLite is the `rusqlite` `bundled` build —
 *     no `sqlite-jdbc`, no Room, no SQLDelight.
 *   * **Apple / Linux / Android Native**: Cinterop against the same Rust
 *     static library `libsmritidb_ffi.a`. *(Wiring lands in Phase C; the
 *     `expect` is here so the surface is stable.)*
 *   * **JS / WasmJs**: the wasm-bindgen build from `packages/core-rs`,
 *     with a `Memory` adapter (every target) and an additional
 *     `openIndexedDb(dbName, dimension)` factory exposed **only** on the
 *     JS / WasmJs `actual` (see `PersistentStoreJs.kt`). The IDB factory
 *     is *not* on this common `expect class` — `expect` declarations
 *     cannot have per-target methods, so callers writing
 *     browser-specific code reach into the platform actual directly.
 *     OPFS-sqlite-wasm via OPFS is a Phase E concern.
 *
 * All three companion-object factories take an optional `dimension`. The
 * default of `10_000` matches `smritidb_core::store::DEFAULT_DIMENSION`
 * and SPEC.md §1.1.
 */
public expect class PersistentStore {
    public companion object {
        public fun openSqlite(
            path: String,
            dimension: Int = 10_000,
        ): PersistentStore

        public fun openFile(
            path: String,
            dimension: Int = 10_000,
        ): PersistentStore

        public fun openMemory(
            dimension: Int = 10_000,
        ): PersistentStore
    }

    /**
     * Insert (or replace) an item under a string cue. The cue is encoded
     * to a hypervector per SPEC.md §3.5. Returns the item's generated id.
     */
    public fun put(
        key: String,
        value: ByteArray,
        tags: List<String> = emptyList(),
    ): String

    /**
     * Top-k recall by similarity to `cue`. Hits with `similarity < minSim`
     * are dropped; ties are broken by ascending id per SPEC.md §4.2.
     */
    public fun recall(
        cue: String,
        topK: Int = 10,
        minSim: Double = 0.5,
    ): List<RecallMatch>

    /** Delete an item by id. Returns true iff an item was removed. */
    public fun delete(id: String): Boolean

    /** Total item count. */
    public fun size(): Int

    /** Configured dimension. */
    public fun dimension(): Int

    /** Run a consolidation pass. */
    public fun consolidate()

    /** Flush a snapshot through the adapter (truncates the WAL). */
    public fun persist()

    /** Release adapter resources. Idempotent. */
    public fun close()
}

/**
 * The lightweight result type for [PersistentStore.recall]. A trimmed-down
 * mirror of `smritidb_core::store::Match` — we only project the fields a
 * KMP caller typically needs in this layer; the rich `Match` (including
 * `value` and `accessCount`) is exposed via the lower-level `SmritidbStore`
 * surface in `Smritidb.kt`.
 */
public data class RecallMatch(
    public val id: String,
    public val similarity: Double,
    public val tags: List<String>,
)
