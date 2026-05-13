// Smritidb — Kotlin Multiplatform wrapper.
//
// Consumes the UniFFI-generated Kotlin (packages/smritidb-ffi/bindings/kotlin/)
// on JVM + Android, and wraps Rust statically via Cinterop on Apple targets.
// The commonMain surface is a thin idiomatic Kotlin API that matches the
// TypeScript reference in shape and the Rust core in semantics.

plugins {
    kotlin("multiplatform") version "2.0.21"
    `maven-publish`
}

group = "com.tanvrit"
version = "0.1.0"

kotlin {
    jvmToolchain(17)

    jvm()

    androidNativeArm64()
    androidNativeX64()

    iosArm64()
    iosX64()
    iosSimulatorArm64()
    macosArm64()
    macosX64()

    linuxX64()
    linuxArm64()

    js(IR) {
        browser()
        nodejs()
    }

    @OptIn(org.jetbrains.kotlin.gradle.ExperimentalWasmDsl::class)
    wasmJs {
        browser()
        nodejs()
    }

    sourceSets {
        val commonMain by getting {
            dependencies {
                implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.8.1")
            }
        }
        val commonTest by getting {
            dependencies {
                implementation(kotlin("test"))
            }
        }

        val jvmMain by getting {
            dependencies {
                // The UniFFI-generated Kotlin uses JNA to load libsmritidb_ffi
                implementation("net.java.dev.jna:jna:5.14.0")
            }
        }

        val jvmTest by getting {
            dependencies {
                implementation(kotlin("test-junit5"))
            }
        }

        // Apple targets share an intermediate source set that uses Cinterop
        // against the Rust static library (libsmritidb_ffi.a).
        val appleMain by creating { dependsOn(commonMain) }
        listOf("iosArm64", "iosX64", "iosSimulatorArm64", "macosArm64", "macosX64").forEach { t ->
            getByName("${t}Main").dependsOn(appleMain)
        }

        // Linux + Android Native share another intermediate set.
        val nativeMain by creating { dependsOn(commonMain) }
        listOf(
            "androidNativeArm64", "androidNativeX64",
            "linuxX64", "linuxArm64",
        ).forEach { t ->
            getByName("${t}Main").dependsOn(nativeMain)
        }

        // JS + WasmJs use the WebAssembly build from packages/core-rs/pkg.
        val webMain by creating { dependsOn(commonMain) }
        getByName("jsMain").dependsOn(webMain)
        getByName("wasmJsMain").dependsOn(webMain)
    }
}

publishing {
    repositories {
        maven {
            name = "GitHubPackages"
            url = uri("https://maven.pkg.github.com/Tanvrit/smritidb")
            credentials {
                username = System.getenv("GITHUB_USERNAME") ?: project.findProperty("gpr.user") as String?
                password = System.getenv("GITHUB_TOKEN") ?: project.findProperty("gpr.key") as String?
            }
        }
    }
    publications.withType<MavenPublication> {
        pom {
            name.set("smritidb-kmp")
            description.set("Smritidb — an open biology-inspired associative memory layer. Kotlin Multiplatform bindings.")
            url.set("https://smritidb.com")
            licenses {
                license {
                    name.set("Apache-2.0")
                    url.set("https://www.apache.org/licenses/LICENSE-2.0.txt")
                }
            }
            developers {
                developer {
                    id.set("tanvrit")
                    name.set("Tanvrit Private Limited")
                    email.set("opensource@tanvrit.com")
                }
            }
            scm {
                url.set("https://github.com/Tanvrit/smritidb")
            }
        }
    }
}
