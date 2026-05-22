// Adds Dokka to packages/smritidb-kmp without modifying its
// build.gradle.kts. Driven by docs/regen.sh.
//
// Why an init script? The KMP module's gradle build is focused on the
// multiplatform wiring (Cinterop, wasm-pack tasks, source-set
// hierarchy). Documentation belongs to the repo's docs/ tree, not to
// the module's own gradle config, so we apply Dokka externally.
//
// Why explicit source roots? The Kotlin Multiplatform plugin does not
// expose a "main" Dokka source set automatically. We register a
// synthetic source set called "smritidb-kmp" and feed it every Kotlin
// source root in the module.
//
// JDK pin: Dokka 1.9.20 ships an IntelliJ utility (`JavaVersion.parse`)
// that throws on JDK 25's version string. Run with JDK 17 — see
// docs/CONTRIBUTING-API-DOCS.md.
initscript {
    repositories {
        gradlePluginPortal()
        mavenCentral()
    }
    dependencies {
        classpath("org.jetbrains.dokka:dokka-gradle-plugin:1.9.20")
    }
}

allprojects {
    apply<org.jetbrains.dokka.gradle.DokkaPlugin>()
    afterEvaluate {
        tasks.named<org.jetbrains.dokka.gradle.DokkaTask>("dokkaHtml") {
            // Resolve the docs/ tree relative to the smritidb-kmp module's
            // root (settings.gradle.kts there makes rootDir == the module).
            outputDirectory.set(
                project.rootDir.resolve("../../docs/api/kotlin")
            )
            dokkaSourceSets {
                register("smritidb-kmp") {
                    val srcRoots = listOf(
                        "src/commonMain/kotlin",
                        "src/jvmMain/kotlin",
                        "src/jsMain/kotlin",
                        "src/wasmJsMain/kotlin",
                        "src/webMain/kotlin",
                        "src/ffiNativeMain/kotlin",
                    )
                    srcRoots.forEach { p ->
                        val f = project.file(p)
                        if (f.exists()) {
                            sourceRoots.from(f)
                        }
                    }
                    displayName.set("smritidb-kmp")
                    includeNonPublic.set(false)
                    skipDeprecated.set(false)
                    reportUndocumented.set(false)
                }
            }
        }
    }
}
