// Step 3 — Kotlin/JVM reader for the polyglot interop demo.
//
// Consumes the already-built `smritidb-kmp-jvm` JAR from the sibling
// `packages/smritidb-kmp` project. The JAR bundles the UniFFI-generated
// Kotlin under the `uniffi.smritidb` package, plus the JNA-loadable
// `libuniffi_smritidb.dylib` (or `.so`) under `darwin-aarch64/` / `linux-x86-64/`.
//
// We add the sibling FFI's `release-jna/` directory as an extra resource
// root so JNA's `Native.load("uniffi_smritidb")` resolves the native
// library on first call without LD_LIBRARY_PATH gymnastics.

plugins {
    kotlin("jvm") version "2.1.20"
    application
}

kotlin {
    jvmToolchain(17)
    compilerOptions {
        // UniFFI 0.28 emits a `message` property that clashes with
        // `kotlin.Throwable.message` on Kotlin 2.1, so the upstream KMP
        // project pins back to language 2.0 — we mirror that.
        languageVersion.set(org.jetbrains.kotlin.gradle.dsl.KotlinVersion.KOTLIN_2_0)
        apiVersion.set(org.jetbrains.kotlin.gradle.dsl.KotlinVersion.KOTLIN_2_0)
    }
}

repositories {
    mavenCentral()
    // Pick up the pre-built `smritidb-kmp-jvm` JAR sitting next to us.
    flatDir { dirs(rootProject.file("../../../packages/smritidb-kmp/build/libs").absolutePath) }
}

dependencies {
    // The published JAR carries `com.tanvrit.smritidb.PersistentStore` AND
    // the embedded UniFFI bindings AND the native dylib (under the
    // `darwin-aarch64/` resource path JNA expects). We use a flatDir
    // dependency because the JAR is not in any local Maven repo.
    implementation(files(rootProject.file("../../../packages/smritidb-kmp/build/libs/smritidb-kmp-jvm-0.1.0.jar")))

    // The JAR was compiled against these but they're not bundled.
    implementation("net.java.dev.jna:jna:5.14.0")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.8.1")
}

application {
    mainClass.set("MainKt")
}
