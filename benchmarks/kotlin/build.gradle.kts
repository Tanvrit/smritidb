// Kotlin/JVM benchmark — runs the canonical workload through the
// smritidb-kmp JVM jar (which itself delegates to the UniFFI-generated
// bindings + libsmritidb_ffi.dylib).
//
// We do a hand-rolled benchmark loop with warm-up + steady-state timing
// rather than JMH, because:
//   1. The JMH Gradle plugin pulls in a ton of transitive deps for what
//      is ultimately a wall-clock measurement.
//   2. We want a single JSON output schema across all four bindings, and
//      JMH's output format is harder to adapt than reading System.nanoTime().
//
// 5-second JIT warmup is hand-coded in Benchmarks.kt.

plugins {
    kotlin("jvm") version "2.4.20"
    application
}

repositories {
    mavenCentral()
}

dependencies {
    // The smritidb-kmp JVM jar was produced by `gradle :smritidb-kmp:jvmJar`
    // ahead of running this benchmark.
    implementation(files(rootDir.resolve("../../packages/smritidb-kmp/build/libs/smritidb-kmp-jvm-0.1.0.jar")))
    // smritidb-kmp's JVM publication depends on JNA + kotlinx-coroutines.
    // We pull them in directly since we're consuming a fat jar rather than
    // a published POM.
    implementation("net.java.dev.jna:jna:5.14.0")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.8.1")
    // Pin the same Kotlin stdlib version smritidb-kmp was compiled against
    // to avoid the runtime warning the launcher otherwise emits.
    implementation("org.jetbrains.kotlin:kotlin-stdlib:2.4.20")
}

application {
    mainClass.set("com.tanvrit.smritidb.bench.BenchmarksKt")
    // JNA needs to find libsmritidb_ffi at runtime. The kmp project copies
    // it under build/libs but for our flat-file dependency we point JNA
    // straight at the cargo output dir.
    val ffiDir = rootDir.resolve("../../packages/smritidb-ffi/target/release-jna/darwin-aarch64")
    val ffiFallback = rootDir.resolve("../../packages/smritidb-ffi/target/release")
    applicationDefaultJvmArgs = listOf(
        "-Djna.library.path=${ffiDir.absolutePath}:${ffiFallback.absolutePath}",
        "-Djna.boot.library.path=${ffiDir.absolutePath}:${ffiFallback.absolutePath}",
    )
}

kotlin {
    jvmToolchain(17)
}

// Custom task with a stable name so run-all.sh can invoke it.
tasks.register<JavaExec>("benchmark") {
    group = "verification"
    description = "Run the cross-binding Kotlin/JVM benchmark."
    classpath = sourceSets.main.get().runtimeClasspath
    mainClass.set("com.tanvrit.smritidb.bench.BenchmarksKt")

    val ffiDir = rootDir.resolve("../../packages/smritidb-ffi/target/release-jna/darwin-aarch64")
    val ffiFallback = rootDir.resolve("../../packages/smritidb-ffi/target/release")
    jvmArgs = listOf(
        "-Djna.library.path=${ffiDir.absolutePath}:${ffiFallback.absolutePath}",
        "-Djna.boot.library.path=${ffiDir.absolutePath}:${ffiFallback.absolutePath}",
        // -Xss2m gives the JIT a comfortable stack for the deep encode_string
        // call chain.
        "-Xss2m",
    )
    // Result path passed as the first arg.
    args = listOf(rootDir.resolve("../results/kotlin.json").absolutePath)
}
