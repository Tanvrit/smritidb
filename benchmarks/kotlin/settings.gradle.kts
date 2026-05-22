// Standalone Gradle build for the Kotlin/JVM benchmark — we intentionally
// don't include this in the smritidb-kmp build to keep its multiplatform
// targets isolated. We depend on the already-built jvmJar via a file
// dependency and JNA loads libsmritidb_ffi from packages/smritidb-ffi/target/release.
rootProject.name = "smritidb-benchmarks-kotlin"
