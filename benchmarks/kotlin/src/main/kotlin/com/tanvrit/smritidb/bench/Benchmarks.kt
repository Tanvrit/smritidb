package com.tanvrit.smritidb.bench

// Cross-binding benchmark — Kotlin/JVM. Runs the canonical workload
// defined in benchmarks/WORKLOAD.md against the smritidb-kmp JVM
// surface (which itself delegates to the UniFFI bindings + Rust).
//
// Emits JSON to the path given as argv[0], matching the schema in
// WORKLOAD.md. We hand-roll the timing loop rather than depending on
// kotlinx-benchmark / JMH: the goal here is a single JSON file across
// all four bindings, not a full statistical bench framework.

import com.tanvrit.smritidb.Cue
import com.tanvrit.smritidb.SPEC_VERSION
import com.tanvrit.smritidb.bind
import com.tanvrit.smritidb.bundle
import com.tanvrit.smritidb.encodeString
import com.tanvrit.smritidb.openStore
import com.tanvrit.smritidb.randomHv
import com.tanvrit.smritidb.similarity
import java.io.File
import java.io.PrintWriter
import kotlin.system.measureNanoTime

private const val D = 10_000u
private const val RANDOM_HV_BATCH = 1_000
private const val BUNDLE_BATCH = 100
private const val BIND_BATCH = 100
private const val SIMILARITY_BATCH = 10_000
private const val ENCODE_STRING_BATCH = 10_000
private val RECALL_NS = intArrayOf(100, 1_000, 10_000)
private const val RECALL_QUERIES = 1_000
private const val PERSIST_N = 10_000
private const val WAL_APPENDS = 1_000
private const val WARMUP_ITERS = 3

private data class PrimRes(val opsPerSec: Double, val iters: Int, val wallSeconds: Double)

private fun measure(name: String, iters: Int, body: (Int) -> Unit): PrimRes {
    // Warm-up: defeats JIT cold-start and lets the inline cache settle.
    repeat(WARMUP_ITERS) { body(0) }
    val nanos = measureNanoTime { for (i in 0 until iters) body(i) }
    val wall = nanos / 1e9
    val ops = iters / wall
    println("  %-15s %12.0f ops/s (%.3fs for %d iters)".format(name, ops, wall, iters))
    return PrimRes(ops, iters, wall)
}

private fun percentile(samples: DoubleArray, p: Double): Double {
    if (samples.isEmpty()) return 0.0
    val sorted = samples.copyOf()
    java.util.Arrays.sort(sorted)
    val idx = ((sorted.size - 1) * p).toInt().coerceIn(0, sorted.size - 1)
    return sorted[idx]
}

// -- JSON emit (no dep) ---------------------------------------------------

private fun n(v: Any?): String = when (v) {
    is Double -> {
        if (v.isFinite()) v.toString() else "null"
    }
    is Number, is Boolean -> v.toString()
    is String -> "\"" + v.replace("\\", "\\\\").replace("\"", "\\\"") + "\""
    null -> "null"
    else -> throw IllegalArgumentException("can't json: $v")
}

private fun rawObj(vararg pairs: Pair<String, String>): String =
    pairs.joinToString(",", prefix = "{", postfix = "}") { (k, v) -> "\"$k\":$v" }

private fun rawArr(items: List<String>): String =
    items.joinToString(",", prefix = "[", postfix = "]")

private fun primJson(r: PrimRes): String = rawObj(
    "ops_per_sec" to n(r.opsPerSec),
    "iters" to n(r.iters),
    "wall_seconds" to n(r.wallSeconds),
)

// -- primitives -----------------------------------------------------------

private data class Primitives(
    val randomHv: PrimRes,
    val bundle: PrimRes,
    val bind: PrimRes,
    val similarity: PrimRes,
    val encodeString: PrimRes,
)

private fun runPrimitives(): Primitives {
    println("[1/4] primitives (Kotlin/JVM)")

    val seeds = Array(RANDOM_HV_BATCH) { "hv_$it".toByteArray() }
    val randomHvR = measure("random_hv", RANDOM_HV_BATCH) { i ->
        randomHv(seeds[i % seeds.size], D)
    }

    val aPool = Array(SIMILARITY_BATCH) { randomHv("a$it".toByteArray(), D) }
    val bPool = Array(SIMILARITY_BATCH) { randomHv("b$it".toByteArray(), D) }

    val bundleR = measure("bundle", BUNDLE_BATCH) { i ->
        bundle(listOf(aPool[i % aPool.size], bPool[i % bPool.size]))
    }
    val bindR = measure("bind", BIND_BATCH) { i ->
        bind(aPool[i % aPool.size], bPool[i % bPool.size])
    }
    val simR = measure("similarity", SIMILARITY_BATCH) { i ->
        similarity(aPool[i % aPool.size], bPool[i % bPool.size])
    }
    val inputs = Array(ENCODE_STRING_BATCH) { "item_$it" }
    val encR = measure("encode_string", ENCODE_STRING_BATCH) { i ->
        encodeString(inputs[i], D)
    }
    return Primitives(randomHvR, bundleR, bindR, simR, encR)
}

// -- recall ---------------------------------------------------------------

private data class RecallRes(
    val n: Int,
    val queries: Int,
    val p50: Double,
    val p99: Double,
    val p999: Double,
    val insertSeconds: Double,
)

private fun runRecall(): List<RecallRes> {
    println("[2/4] recall (Kotlin/JVM)")
    val out = mutableListOf<RecallRes>()
    for (n in RECALL_NS) {
        val store = openStore(D)
        val insNs = measureNanoTime {
            for (i in 0 until n) {
                store.put(Cue.Text("item_$i"), "value for item $i".toByteArray())
            }
        }
        val insertSec = insNs / 1e9

        // Warm-up.
        repeat(WARMUP_ITERS) { store.recall(Cue.Text("item_0"), 10u, 0.5) }

        val samplesMs = DoubleArray(RECALL_QUERIES)
        for (i in 0 until RECALL_QUERIES) {
            val t = System.nanoTime()
            store.recall(Cue.Text("item_${i % n}"), 10u, 0.5)
            samplesMs[i] = (System.nanoTime() - t) / 1e6
        }
        val p50 = percentile(samplesMs, 0.5)
        val p99 = percentile(samplesMs, 0.99)
        val p999 = percentile(samplesMs, 0.999)
        println(
            "  N=%5d insert=%.3fs p50=%.3fms p99=%.3fms p999=%.3fms".format(
                n, insertSec, p50, p99, p999,
            ),
        )
        out.add(RecallRes(n, RECALL_QUERIES, p50, p99, p999, insertSec))
        store.close()
    }
    return out
}

// -- persistence & memory -------------------------------------------------
//
// Note: the smritidb-kmp commonMain surface does not yet expose
// PersistentStore on JVM. The UniFFI-generated jvm bindings include a
// Store (in-memory) only. We measure what we can — primitives, recall,
// memory — and mark persistence "N/A (binding gap: no JVM
// PersistentStore on the kmp surface today)" in the output.

private data class PersistenceRes(
    val available: Boolean,
    val note: String,
)

private fun runPersistence(): PersistenceRes {
    println("[3/4] persistence (Kotlin/JVM): SKIPPED")
    println("  Reason: smritidb-kmp commonMain does not yet expose PersistentStore on JVM.")
    return PersistenceRes(
        false,
        "smritidb-kmp commonMain has only an in-memory Store on the JVM today; persistence is wired via UniFFI on iOS / Apple native but the JVM `openStore` API does not yet have an `openSqlite` actual.",
    )
}

private data class MemoryRes(
    val nItems: Int,
    val rssBefore: Long,
    val rssAfter: Long,
    val perItem: Double,
)

private fun rssBytes(): Long {
    // Use OS RSS via /proc on Linux; on macOS shell out to `ps` like the
    // Rust runner. The JVM's own `Runtime.totalMemory()` reports heap
    // only, which excludes JIT code cache and native libs — we want the
    // full picture for the bytes-per-item comparison.
    val pid = ProcessHandle.current().pid()
    val os = System.getProperty("os.name").lowercase()
    if (os.contains("linux")) {
        val status = File("/proc/self/status").takeIf { it.exists() }?.readText() ?: return 0
        val line = status.lines().firstOrNull { it.startsWith("VmRSS:") } ?: return 0
        val kb = line.substringAfter(":").trim().split(Regex("\\s+")).firstOrNull()?.toLongOrNull() ?: 0
        return kb * 1024
    }
    if (os.contains("mac") || os.contains("darwin")) {
        return try {
            val proc = ProcessBuilder("ps", "-o", "rss=", "-p", pid.toString()).redirectErrorStream(true).start()
            val text = proc.inputStream.bufferedReader().readText().trim()
            proc.waitFor()
            (text.toLongOrNull() ?: 0) * 1024L
        } catch (_: Exception) {
            0L
        }
    }
    return 0
}

private fun runMemory(): MemoryRes {
    println("[4/4] memory (Kotlin/JVM)")
    System.gc()
    val before = rssBytes()
    val store = openStore(D)
    for (i in 0 until PERSIST_N) {
        store.put(Cue.Text("item_$i"), "value for item $i".toByteArray())
    }
    System.gc()
    Thread.sleep(50) // let the GC settle
    val after = rssBytes()
    val delta = (after - before).coerceAtLeast(0)
    val perItem = delta.toDouble() / PERSIST_N
    println("  RSS before=${before}B after=${after}B delta=${delta}B = %.1f bytes/item".format(perItem))
    // Keep store alive past sample.
    @Suppress("UNUSED_EXPRESSION") store.size()
    store.close()
    return MemoryRes(PERSIST_N, before, after, perItem)
}

// -- main -----------------------------------------------------------------

fun main(args: Array<String>) {
    val outPath = args.getOrNull(0) ?: "../results/kotlin.json"
    println("Smritidb benchmark — Kotlin/JVM binding")
    println("  Java ${System.getProperty("java.version")} on ${System.getProperty("os.name")}")
    println("  spec=$SPEC_VERSION")

    val primitives = runPrimitives()
    val recall = runRecall()
    val persistence = runPersistence()
    val memory = runMemory()

    val os = System.getProperty("os.name").lowercase()
    val arch = System.getProperty("os.arch").lowercase()

    val json = rawObj(
        "binding" to "\"kotlin\"",
        "version" to "\"0.1.0\"",
        "spec_version" to "\"$SPEC_VERSION\"",
        "host" to rawObj("os" to "\"$os\"", "arch" to "\"$arch\""),
        "primitives" to rawObj(
            "random_hv" to primJson(primitives.randomHv),
            "bundle" to primJson(primitives.bundle),
            "bind" to primJson(primitives.bind),
            "similarity" to primJson(primitives.similarity),
            "encode_string" to primJson(primitives.encodeString),
        ),
        "recall" to rawArr(recall.map { r ->
            rawObj(
                "n" to n(r.n),
                "queries" to n(r.queries),
                "p50_ms" to n(r.p50),
                "p99_ms" to n(r.p99),
                "p999_ms" to n(r.p999),
                "insert_seconds" to n(r.insertSeconds),
            )
        }),
        "persistence" to rawObj(
            "available" to n(persistence.available),
            "note" to n(persistence.note),
        ),
        "memory" to rawObj(
            "n_items" to n(memory.nItems),
            "rss_before_bytes" to n(memory.rssBefore),
            "rss_after_bytes" to n(memory.rssAfter),
            "bytes_per_item" to n(memory.perItem),
            "theoretical_floor_bytes_per_item" to n((D / 8u).toInt()),
        ),
    )

    val file = File(outPath)
    file.parentFile?.mkdirs()
    PrintWriter(file).use { it.print(json) }
    println("wrote ${file.absolutePath}")
}
