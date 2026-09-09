plugins {
    kotlin("jvm") version "2.2.20"
    application
}

repositories {
    mavenCentral()
}

dependencyLocking {
    lockAllConfigurations()
}

dependencies {
    implementation("net.java.dev.jna:jna:5.18.1")
}

kotlin {
    jvmToolchain(17)
}

val generatedDir = providers.environmentVariable("IDENTUS_UNIFFI_GENERATED_DIR")
val libraryDir = providers.environmentVariable("IDENTUS_UNIFFI_LIBRARY_DIR")

sourceSets {
    main {
        kotlin.srcDir(generatedDir)
    }
}

application {
    mainClass.set("org.hyperledger.identus.did.SmokeKt")
    applicationDefaultJvmArgs = listOf("-Djna.library.path=${libraryDir.get()}")
}
