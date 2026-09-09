plugins {
    id("com.android.library")
    kotlin("android")
}

android {
    namespace = "org.hyperledger.identus.did"
    compileSdk = 35

    defaultConfig {
        minSdk = 21
    }

    buildTypes {
        release {
            isMinifyEnabled = false
        }
    }
}

dependencyLocking {
    lockAllConfigurations()
}

dependencies {
    compileOnly("net.java.dev.jna:jna:5.18.1@aar")
}

kotlin {
    jvmToolchain(17)
}
