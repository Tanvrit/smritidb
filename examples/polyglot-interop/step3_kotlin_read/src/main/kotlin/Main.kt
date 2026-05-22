// Step 3 — Kotlin/JVM reader for the polyglot interop demo.
//
// Opens the SQLite file written by Step 1 (Python) and asserts that
// (a) the item count matches and (b) the top-1 recall value is
// byte-identical to what Python wrote.
//
// We open via the **same JAR** that backs `packages/smritidb-kmp` on JVM
// (`PersistentStore.openSqlite(path)` — see PersistentStoreJvm.kt) for the
// size and recall check; then we drop down to the underlying UniFFI
// `uniffi.smritidb.PersistentStore` (bundled in the same JAR) for the
// byte-identity assertion, because the lightweight `RecallMatch` returned
// by the KMP wrapper deliberately omits the value blob — the rich
// `Match` lives one layer below.

import com.tanvrit.smritidb.PersistentStore as KmpStore
import uniffi.smritidb.PersistentStore as UniffiStore
import uniffi.smritidb.StoreOptions as UniffiStoreOptions
import kotlin.system.exitProcess

private const val PATH = "/tmp/smritidb-polyglot.db"
private const val EXPECTED_ITEMS = 100
private val EXPECTED_TOP_VALUE = "item value 50".toByteArray(Charsets.US_ASCII)

fun main() {
    // --- 1. KMP-level open + size + top-k via the idiomatic Kotlin surface.
    val kmpStore = try {
        // Dimension argument is ignored at restore-time (the persisted dimension
        // wins), but we pass the value Python wrote to be honest about the
        // shared agreement.
        KmpStore.openSqlite(PATH, dimension = 8192)
    } catch (e: Throwable) {
        System.err.println("FAIL: cannot open $PATH via KMP wrapper: $e")
        exitProcess(1)
    }

    try {
        val size = kmpStore.size()
        if (size != EXPECTED_ITEMS) {
            System.err.println("FAIL: KMP reported $size items, expected $EXPECTED_ITEMS")
            exitProcess(1)
        }
        val kmpHits = kmpStore.recall(cue = "concept_50", topK = 5, minSim = 0.5)
        if (kmpHits.isEmpty()) {
            System.err.println("FAIL: KMP recall returned no hits")
            exitProcess(1)
        }
        if (kmpHits[0].similarity < 0.99) {
            System.err.println("FAIL: KMP top similarity ${kmpHits[0].similarity} < 0.99")
            exitProcess(1)
        }
    } finally {
        kmpStore.close()
    }

    // --- 2. UniFFI-level open + recall to get the value bytes for the
    // byte-identity check. Same dylib, same C ABI — just the layer that
    // returns the full `Match { id, similarity, value, tags, accessCount }`.
    val opts = UniffiStoreOptions(
        dimension = 8192u,
        valueCapBytes = 0u,
        defaultTopK = 0u,
        defaultMinSimilarity = 0.0,
    )
    val uniStore = UniffiStore.openSqlite(PATH, opts)
    try {
        val uniSize = uniStore.size().toInt()
        if (uniSize != EXPECTED_ITEMS) {
            System.err.println("FAIL: UniFFI reported $uniSize items, expected $EXPECTED_ITEMS")
            exitProcess(1)
        }
        val hits = uniStore.recall("concept_50", topK = 5u, minSimilarity = 0.5)
        if (hits.isEmpty()) {
            System.err.println("FAIL: UniFFI recall returned no hits")
            exitProcess(1)
        }
        val top = hits[0]
        if (!top.value.contentEquals(EXPECTED_TOP_VALUE)) {
            System.err.println(
                "FAIL: top value byte-mismatch: got ${top.value.toString(Charsets.US_ASCII)!!}, " +
                    "expected ${EXPECTED_TOP_VALUE.toString(Charsets.US_ASCII)}"
            )
            exitProcess(1)
        }
        if (top.similarity < 0.99) {
            System.err.println("FAIL: top similarity ${top.similarity} < 0.99")
            exitProcess(1)
        }

        println(
            "PASS: Kotlin read $uniSize items from $PATH " +
                "(top-1 value=${top.value.toString(Charsets.US_ASCII)} " +
                "@ sim=${"%.6f".format(top.similarity)})"
        )
    } finally {
        uniStore.close()
    }
}
