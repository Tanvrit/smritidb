// Smritidb — Kotlin Multiplatform wrapper.
//
// Consumes the UniFFI-generated Kotlin (packages/smritidb-ffi/bindings/kotlin/)
// on JVM + Android-JVM, and wraps Rust statically via Cinterop on every
// Kotlin/Native target that links `libsmritidb_ffi.a` — Apple (macOS +
// iOS) and Android Native. The commonMain surface is a thin idiomatic
// Kotlin API that matches the TypeScript reference in shape and the
// Rust core in semantics.

plugins {
    kotlin("multiplatform") version "2.1.20"
    `maven-publish`
}

group = "com.tanvrit.smritidb"
version = "0.1.0"

// --- Pin gradle-managed Node to a version with `process.getBuiltinModule` ---
//
// The Kotlin/Wasm test runner emits an ESM bundle whose `@JsFun` arrow
// functions don't get a synchronous CommonJS `require()` in scope. The
// wasm bootstrap (`wasmJsTest/.../Bootstrap.kt`) reaches the wasm-bindgen
// Node module by calling `process.getBuiltinModule('node:module')` to get
// `createRequire(...)` — which was added in Node 22.1.0. The KMP plugin
// 2.1.20 defaults to Node 22.0.0, so we pin to 22.11.0 (current 22.x LTS)
// via the Kotlin 2.1 `NodeJsEnvSpec` Gradle extension.
//
// Both `js` and `wasmJs` targets share a single `NodeJsEnvSpec`, so this
// one knob covers `jsNodeTest` and `wasmJsNodeTest`.
plugins.withType<org.jetbrains.kotlin.gradle.targets.js.nodejs.NodeJsPlugin> {
    the<org.jetbrains.kotlin.gradle.targets.js.nodejs.NodeJsEnvSpec>().version.set("22.11.0")
}

kotlin {
    jvmToolchain(17)

    // The UniFFI 0.28-generated Kotlin bindings emit a property named
    // `message` on exception subclasses that shadows `kotlin.Exception.message`.
    // Kotlin 2.1 surfaces this as an "overload resolution ambiguity" error
    // (it was a silent warning in 2.0). Pin the language and API level back
    // until UniFFI emits an `override val message`. Also opt in to
    // `expect`/`actual` classes (KT-61573) which our `PersistentStore`
    // surface uses.
    compilerOptions {
        languageVersion.set(org.jetbrains.kotlin.gradle.dsl.KotlinVersion.KOTLIN_2_0)
        apiVersion.set(org.jetbrains.kotlin.gradle.dsl.KotlinVersion.KOTLIN_2_0)
        freeCompilerArgs.add("-Xexpect-actual-classes")
    }

    jvm {
        testRuns["test"].executionTask.configure {
            useJUnitPlatform()
        }
    }

    // Kotlin/Native targets: Cinterop against the UniFFI-generated C
    // ABI of libsmritidb_ffi (a static archive built once per Rust
    // triple via `cargo build --release --target <triple>`). The
    // Cinterop def file — `src/nativeInterop/cinterop/smritidb_ffi.def`
    // — is target-independent; we inject the header path and the
    // per-target `-L<rust target dir>` flag here.
    //
    // The shared `actual` implementation in
    // `src/ffiNativeMain/.../PersistentStoreNative.kt` and
    // `SmritidbNative.kt` serialises arguments by hand into the UniFFI
    // binary `RustBuffer` format (Big-Endian, length-prefixed) —
    // UniFFI does not ship native-Kotlin bindings so we drive the
    // low-level C ABI directly. The same source set serves Apple
    // (macOS + iOS) and Android Native — both link the exact same
    // `libsmritidb_ffi.a` (just built with a different cross
    // toolchain).
    // `packages/smritidb-kmp` is a standalone Gradle build, so
    // `project.rootDir` resolves there — not at the repo root.
    // `../smritidb-ffi` walks up to `packages/smritidb-ffi`.
    val rustFfiDir = project.rootDir.resolve("../smritidb-ffi")
    val ffiHeader = rustFfiDir.resolve("bindings/swift/smritidbFFI.h")

    fun org.jetbrains.kotlin.gradle.plugin.mpp.KotlinNativeTarget.wireSmritidbFfi(rustTriple: String) {
        val libDir = rustFfiDir.resolve("target/$rustTriple/release")
        compilations.getByName("main").cinterops {
            val smritidb_ffi by creating {
                defFile(project.file("src/nativeInterop/cinterop/smritidb_ffi.def"))
                // Add the directory holding `smritidbFFI.h` to the C
                // preprocessor's include path so the def file's
                // `headers = smritidbFFI.h` line resolves.
                compilerOpts("-I${ffiHeader.parentFile.absolutePath}")
                // Make the cinterop's link-test (Kotlin/Native confirms
                // every undefined symbol it sees can be resolved) find
                // the per-triple static archive.
                extraOpts("-libraryPath", libDir.absolutePath)
            }
        }
        // The final binary link picks up the same archive. Without
        // this, downstream consumers' executables would fail to link
        // against `_uniffi_smritidb_ffi_*`. On Linux we use GNU ld's
        // `-l:filename` syntax to force the static `.a` rather than
        // letting the linker pick the `.so` that `cargo build` also
        // produces — the `.so` references newer glibc symbols
        // (`fcntl64@GLIBC_2.28` etc.) that the Kotlin/Native sysroot
        // does not provide. Apple's `ld` does not understand `-l:`,
        // but on macOS / iOS `cargo build` produces a `.dylib` (not a
        // `.so`) and `-lsmritidb_ffi` resolves to the `.a` by default.
        binaries.all {
            val isLinux = rustTriple.contains("unknown-linux-gnu")
            linkerOpts(
                "-L${libDir.absolutePath}",
                if (isLinux) "-l:libsmritidb_ffi.a" else "-lsmritidb_ffi",
            )
        }
    }

    iosArm64 { wireSmritidbFfi("aarch64-apple-ios") }
    iosX64 { wireSmritidbFfi("x86_64-apple-ios") }
    iosSimulatorArm64 { wireSmritidbFfi("aarch64-apple-ios-sim") }
    macosArm64 { wireSmritidbFfi("aarch64-apple-darwin") }
    macosX64 { wireSmritidbFfi("x86_64-apple-darwin") }

    // Android Native targets re-use the exact same Cinterop +
    // libsmritidb_ffi.a pattern as Apple. The Rust archives are built
    // via `cargo build --release --target <triple>` with an Android
    // NDK cross-compile toolchain — see README for the env setup.
    // The Kotlin actual code is source-shared with Apple via the
    // `ffiNativeMain` source set defined below.
    androidNativeArm64 { wireSmritidbFfi("aarch64-linux-android") }
    androidNativeX64 { wireSmritidbFfi("x86_64-linux-android") }

    // Linux native targets follow the same pattern. The Rust archives
    // for `x86_64-unknown-linux-gnu` and `aarch64-unknown-linux-gnu`
    // are cross-compiled from a Mac host via the
    // `messense/macos-cross-toolchains` Homebrew tap (see README) or
    // produced natively on a Linux CI runner. The Kotlin actual code
    // is source-shared with Apple + Android Native via `ffiNativeMain`;
    // the Cinterop klib produced here exposes the same
    // `smritidb_ffi.*` package, so the bridge code resolves the same
    // UniFFI symbols on every target.
    linuxX64 { wireSmritidbFfi("x86_64-unknown-linux-gnu") }
    linuxArm64 { wireSmritidbFfi("aarch64-unknown-linux-gnu") }

    // Kotlin/JS + Kotlin/Wasm consume the wasm-bindgen output from
    // `packages/core-rs/pkg-node/` (Node target — CommonJS, sync-loads
    // its own `.wasm` next to it) and `packages/core-rs/pkg/` (browser
    // target — ESM, async fetches the `.wasm`). The KMP module exposes
    // **memory-only** persistence on these platforms; `openSqlite` /
    // `openFile` throw `UnsupportedOperationException` because rusqlite
    // doesn't compile to `wasm32-unknown-unknown`. IndexedDB and
    // OPFS-sqlite-wasm adapters are tracked as Phase D.
    js(IR) {
        nodejs {
            testTask {
                useMocha {
                    timeout = "30s"
                }
            }
        }
        browser()
        binaries.executable()
    }

    @OptIn(org.jetbrains.kotlin.gradle.ExperimentalWasmDsl::class)
    wasmJs {
        nodejs()
        browser()
        binaries.executable()
    }

    // Kotlin/Wasm Node tests run against gradle-managed Node 22.11.0
    // (pinned above). The wasm bootstrap in `wasmJsTest/.../Bootstrap.kt`
    // pulls in the wasm-bindgen Node bundle via `process.getBuiltinModule`
    // (Node 22.1+), so the suite needs that pinned version to function.
    //
    // The browser variant (`wasmJsBrowserTest`) is still skipped — it
    // would need a wasm-pack browser bundle wired through karma, which
    // duplicates the Node-side coverage with no Phase D win.
    tasks.matching { it.name == "wasmJsBrowserTest" }.configureEach {
        enabled = false
    }

    sourceSets {
        val commonMain by getting {
            dependencies {
                implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.8.1")
            }
        }
        val commonTest by getting {
            dependencies {
                implementation(kotlin("test-common"))
                implementation(kotlin("test-annotations-common"))
                // `runTest` powers the suspend-aware coroutine smoke tests
                // (Phase D IndexedDB adapter on JS). The artifact is
                // multiplatform; KMP picks the per-target variant.
                implementation("org.jetbrains.kotlinx:kotlinx-coroutines-test:1.8.1")
            }
        }

        val jvmMain by getting {
            // The UniFFI-generated Kotlin bindings live under
            // `src/jvmMain/kotlin/uniffi/smritidb/smritidb.kt` — copied from
            // `packages/smritidb-ffi/bindings/kotlin/` and locally patched for
            // Kotlin 2.x source compatibility (UniFFI 0.28 emits clashing
            // `close()` overrides and a `message` property that shadows
            // `kotlin.Throwable.message` without an `override` modifier).
            // Regeneration steps live in `README.md`.
            //
            // Bundle the native library produced by `cargo build` so consumers
            // of the published jar don't need to set java.library.path. JNA's
            // platform-specific resource layout is `<jar>/<os-arch>/<libname>`.
            resources.srcDir("../smritidb-ffi/target/release-jna")
            dependencies {
                // The UniFFI-generated Kotlin uses JNA to load libsmritidb_ffi
                implementation("net.java.dev.jna:jna:5.14.0")
            }
        }

        val jvmTest by getting {
            dependencies {
                implementation(kotlin("test-junit5"))
                runtimeOnly("org.junit.jupiter:junit-jupiter-engine:5.10.2")
                runtimeOnly("org.junit.platform:junit-platform-launcher:1.10.2")
            }
        }

        // Every Kotlin/Native target that links libsmritidb_ffi.a shares
        // a single set of `actual` implementations driven by Cinterop +
        // a hand-rolled UniFFI lift/lower bridge. That covers Apple
        // (macOS + iOS), Android Native, and Linux (x64 + arm64).
        // Keeping the bridge in one source set avoids drift — the
        // Cinterop klib is named the same on every target
        // (`smritidb_ffi`, via the shared .def file) so imports
        // resolve without per-target branching.
        val ffiNativeMain by creating { dependsOn(commonMain) }
        val ffiNativeTest by creating {
            dependsOn(commonTest)
            dependencies {
                implementation(kotlin("test"))
            }
        }
        listOf(
            "iosArm64", "iosX64", "iosSimulatorArm64", "macosArm64", "macosX64",
            "androidNativeArm64", "androidNativeX64",
            "linuxX64", "linuxArm64",
        ).forEach { t ->
            getByName("${t}Main").dependsOn(ffiNativeMain)
            getByName("${t}Test").dependsOn(ffiNativeTest)
        }

        // Linux-only test source set. Sits between `linuxX64Test` /
        // `linuxArm64Test` and `ffiNativeTest`, so files dropped into
        // `src/linuxTest/kotlin/` get picked up by both Linux native
        // test compilations. Apple and Android Native do not depend on
        // it.
        val linuxTest by creating {
            dependsOn(ffiNativeTest)
        }
        listOf("linuxX64", "linuxArm64").forEach { t ->
            getByName("${t}Test").dependsOn(linuxTest)
        }

        // JS + WasmJs share the high-level "wasm-backed PersistentStore"
        // wiring — both use the wasm-bindgen output from
        // `packages/core-rs/pkg-node/`. The two targets diverge only in
        // how Kotlin reaches the JS module:
        //
        //   * `jsMain` uses `@JsModule("smritidb-core")` against the
        //     CommonJS package; the `npm(...)` dependency below tells
        //     the Kotlin/JS resolver where to find it.
        //   * `wasmJsMain` cannot use `@JsModule`; instead it declares
        //     a few `@JsFun(...)` external bridges and the host loader
        //     stashes the wasm-bindgen module on `globalThis.SmritidbWasm`
        //     before any Kotlin code runs.
        //
        // Both expose ONLY the memory adapter — see PersistentStoreJs.kt
        // / PersistentStoreWasmJs.kt for `openSqlite` / `openFile`
        // throwing `UnsupportedOperationException`. IndexedDB and
        // OPFS-sqlite-wasm are Phase D.
        val webMain by creating { dependsOn(commonMain) }

        val jsMain by getting {
            dependsOn(webMain)
            dependencies {
                // Pin to the local wasm-pack output. Re-runs of
                // `wasm-pack build --target nodejs ... --out-dir pkg-node`
                // refresh the package without touching the gradle build.
                implementation(npm("smritidb-core", File(projectDir, "../core-rs/pkg-node").absolutePath))
            }
        }
        val jsTest by getting {
            dependencies {
                implementation(kotlin("test-js"))
                // `fake-indexeddb` provides a Node-side polyfill for the
                // browser IndexedDB API so the Phase D IndexedDB adapter
                // can be smoke-tested from `jsNodeTest`. Importing
                // `fake-indexeddb/auto` installs the globals; the actual
                // adapter code does not depend on the package at runtime
                // in production (it uses the browser's native indexedDB).
                implementation(npm("fake-indexeddb", "6.0.0"))
            }
        }

        val wasmJsMain by getting {
            dependsOn(webMain)
        }
        val wasmJsTest by getting {
            dependencies {
                implementation(kotlin("test-wasm-js"))
                // Same Node-side IDB polyfill as `jsTest`; the wasmJs
                // IDB adapter doesn't depend on it at runtime in
                // production, but Node has no native `indexedDB` so we
                // install it before each suite that exercises the
                // adapter.
                implementation(npm("fake-indexeddb", "6.0.0"))
            }
        }
    }
}

// Make sure the wasm-pack output exists before Kotlin tries to resolve
// the `smritidb-core` NPM dependency. `wasmPackBuild` is a no-op when
// the Rust sources haven't changed (cargo's incremental build handles
// that), so wiring it up unconditionally costs nothing.
val wasmPackBuildWeb by tasks.registering(Exec::class) {
    description = "Build packages/core-rs as a wasm-bindgen package for the browser (ESM)."
    group = "build"
    workingDir = File(projectDir, "../core-rs")
    // Prepend the rustup toolchain dir so we pick the rustup-managed
    // rustc (which has the wasm32 stdlib component) instead of any
    // Homebrew rust on PATH that does not.
    val rustupBin = File(System.getProperty("user.home"), ".rustup/toolchains/stable-aarch64-apple-darwin/bin")
    if (rustupBin.exists()) {
        environment("PATH", "${rustupBin.absolutePath}:${System.getenv("PATH")}")
    }
    commandLine(
        "wasm-pack", "build", "--target", "web", "--release",
        "--", "--no-default-features", "--features", "wasm",
    )
}

val wasmPackBuildNode by tasks.registering(Exec::class) {
    description = "Build packages/core-rs as a wasm-bindgen package for Node (CommonJS)."
    group = "build"
    workingDir = File(projectDir, "../core-rs")
    val rustupBin = File(System.getProperty("user.home"), ".rustup/toolchains/stable-aarch64-apple-darwin/bin")
    if (rustupBin.exists()) {
        environment("PATH", "${rustupBin.absolutePath}:${System.getenv("PATH")}")
    }
    commandLine(
        "wasm-pack", "build", "--target", "nodejs", "--release",
        "--out-dir", "pkg-node",
        "--", "--no-default-features", "--features", "wasm",
    )
}

val wasmPackBuild by tasks.registering {
    description = "Builds both wasm-pack outputs consumed by jsMain / wasmJsMain."
    group = "build"
    dependsOn(wasmPackBuildWeb, wasmPackBuildNode)
}

publishing {
    repositories {
        maven {
            name = "GitHubPackages"
            url = uri("https://maven.pkg.github.com/kanervalabs/smritidb")
            credentials {
                username = System.getenv("GITHUB_USERNAME") ?: project.findProperty("gpr.user") as String?
                password = System.getenv("GITHUB_TOKEN") ?: project.findProperty("gpr.key") as String?
            }
        }
    }
    publications.withType<MavenPublication> {
        pom {
            name.set("Smritidb")
            description.set("Kotlin Multiplatform bindings for Smritidb binary HDC associative memory.")
            url.set("https://kanervalabs.com/smritidb")
            licenses {
                license {
                    name.set("Apache-2.0")
                    url.set("https://www.apache.org/licenses/LICENSE-2.0.txt")
                    distribution.set("repo")
                }
            }
            developers {
                developer {
                    id.set("vivek-singh")
                    name.set("Vivek Singh")
                    email.set("ervivek40@gmail.com")
                    organization.set("Tanvrit Private Limited")
                    organizationUrl.set("https://kanervalabs.com")
                }
            }
            scm {
                url.set("https://github.com/kanervalabs/smritidb")
                connection.set("scm:git:git://github.com/kanervalabs/smritidb.git")
                developerConnection.set("scm:git:ssh://github.com/kanervalabs/smritidb.git")
            }
        }
    }
}
