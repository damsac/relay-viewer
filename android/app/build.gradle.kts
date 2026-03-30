plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.compose.compiler)
}

android {
    // NOTE: Java/Kotlin package names cannot contain hyphens.
    // If relay-viewer has hyphens, use relay_viewer here instead.
    namespace = "dev.damsac.relay_viewer"
    compileSdk = 35

    defaultConfig {
        applicationId = "dev.damsac.relay_viewer"
        minSdk = 26
        targetSdk = 35
        versionCode = 1
        versionName = "0.1.0"
        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }

    buildFeatures {
        compose = true
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }

    // Ensure UniFFI bindings exist before building
    tasks.register("ensureUniffiGenerated") {
        doLast {
            val bindingsDir = file("src/main/java/dev/damsac/relay_viewer/rust")
            if (!bindingsDir.exists() || bindingsDir.listFiles()?.isEmpty() != false) {
                throw GradleException(
                    "UniFFI Kotlin bindings not found at ${bindingsDir.path}. " +
                    "Run: just gen-kotlin"
                )
            }
        }
    }
    tasks.named("preBuild") { dependsOn("ensureUniffiGenerated") }
}

dependencies {
    implementation(platform(libs.compose.bom))
    implementation(libs.compose.ui)
    implementation(libs.compose.material3)
    implementation(libs.compose.activity)
    implementation(libs.jna) { artifact { type = "aar" } }
    androidTestImplementation(platform(libs.compose.bom))
    androidTestImplementation(libs.compose.ui.test.junit4)
    debugImplementation(libs.compose.ui.test.manifest)
}
