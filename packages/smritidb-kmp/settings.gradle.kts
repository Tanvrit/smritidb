rootProject.name = "smritidb-kmp"

pluginManagement {
    repositories {
        gradlePluginPortal()
        mavenCentral()
        // Google Maven is asked only for the groups it serves: a host that
        // errors instead of answering 404 fails the lookup even when another
        // repository has the artifact.
        google {
            mavenContent {
                includeGroupAndSubgroups("androidx")
                includeGroupAndSubgroups("com.android")
                includeGroupAndSubgroups("com.google")
            }
        }
    }
}

dependencyResolutionManagement {
    repositories {
        mavenCentral()
        google {
            mavenContent {
                includeGroupAndSubgroups("androidx")
                includeGroupAndSubgroups("com.android")
                includeGroupAndSubgroups("com.google")
            }
        }
    }
}
