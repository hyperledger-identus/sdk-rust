plugins {
    id("com.android.application") version "8.2.2"
    id("org.jetbrains.kotlin.android") version "1.9.23"
}

android {
    namespace = "io.identus.example"
    compileSdk = 34

    defaultConfig {
        applicationId = "io.identus.example"
        minSdk = 24
        targetSdk = 34
        versionCode = 1
        versionName = "1.0"
    }

    buildTypes {
        release {
            isMinifyEnabled = false
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }

    // Point jniLibs to the directory where `cargo ndk` places .so files.
    // Each ABI directory under jniLibs/ is populated by `just build-android-example`.
    sourceSets {
        getByName("main") {
            jniLibs.srcDirs("src/main/jniLibs")
        }
    }
}

dependencies {
    // JNA AAR — required by UniFFI-generated Kotlin bindings for native library loading.
    // The @aar classifier provides Android-specific native stubs.
    implementation("net.java.dev.jna:jna:5.14.0@aar")

    // AndroidX AppCompat — required by AppCompatActivity used in MainActivity.kt
    implementation("androidx.appcompat:appcompat:1.6.1")
}
