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

sourceSets {
    main {
        kotlin.srcDir("../target/uniffi-generated-a")
    }
}

application {
    mainClass.set("org.hyperledger.identus.spike.SmokeKt")
    applicationDefaultJvmArgs = listOf(
        "-Djna.library.path=${file("../target/release").absolutePath}",
    )
}
