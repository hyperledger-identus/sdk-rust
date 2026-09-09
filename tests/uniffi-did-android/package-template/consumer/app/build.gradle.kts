plugins {
    id("com.android.application")
    kotlin("android")
}

val identusDidAar = providers.gradleProperty("identusDidAar")

android {
    namespace = "org.hyperledger.identus.did.consumer"
    compileSdk = 35

    defaultConfig {
        applicationId = "org.hyperledger.identus.did.consumer"
        minSdk = 21
        targetSdk = 35
        versionCode = 1
        versionName = "0.0.0-local"
    }
}

dependencyLocking {
    lockAllConfigurations()
}

dependencies {
    implementation(files(identusDidAar))
    implementation("net.java.dev.jna:jna:5.18.1@aar")
}

kotlin {
    jvmToolchain(17)
}
