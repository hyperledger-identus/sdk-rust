# ADR 0100: prove a local UniFFI DID Android package

- **Status:** Accepted
- **Date:** 2026-09-09
- **Decision authority:** issues #163, #222 and #230 under standing SDK mandate
- **Builds on:** ADR 0098 and ADR 0099
- **Related work:** issue #230

## Context

The host foundation proves generated Kotlin on a macOS JVM, and the Apple
proof packages the same ABI for iOS. Neither proves Android NDK linking, AAR
composition, Android JNA loading or execution in an Android runtime.

UniFFI 0.32's Kotlin binding uses JNA. JNA 5.18.1 provides a dedicated Android
AAR containing dispatch libraries for several ABIs. Copying those libraries
into an arm64-only SDK AAR would hide a runtime dependency, complicate license
and resource provenance, and blur the SDK's claimed ABI surface.

## Decision

1. Build the existing ABI-versioned `identus-uniffi-did` cdylib for arm64
   Android using Rust 1.98.1, NDK 27.0.12077973 and minimum API 21.
2. Use exact UniFFI 0.32.0 to generate Kotlin, then package that source and
   exactly one SDK shared library as a local arm64-v8a AAR with exact AGP
   8.13.2, Gradle 8.14.4, Kotlin 2.2.20 and JDK 17.
3. Do not merge, shade or copy JNA into the SDK AAR. The ephemeral consumer
   declares exact `net.java.dev.jna:jna:5.18.1@aar`; eventual Maven metadata
   must represent the equivalent transitive dependency.
4. Build/package independently twice and compare libraries, generated source
   and normalized complete AAR trees. Inspect architecture, Android API/NDK
   note, hardening, needed libraries, exported symbols, paths and sizes.
5. Execute API version, valid identifier, invalid identifier, oversized input
   and redacted-error behavior in a fresh arm64 API-35 AVD whose state is
   isolated below ignored `target/`.
6. Keep all Android proof work in the dedicated bindings shell and
   weekly/manual slow macOS lane. The fast Linux line is unchanged.

## Consequences

The repository gains reproducible evidence that its narrow DID ABI can be
loaded from an Android AAR and executed on one emulator. It does not gain a
published or supported Android SDK.

The Android Gradle/JNA cone remains an outer build-and-consumer concern and
does not enter generic Rust crates or public Rust types. JNA remains
dependency-owned native code behind redacted SDK behavior. No new secret,
signing, storage, network, callback, async or object-handle surface is added.

Only arm64-v8a on one API-35 emulator is proven. Other ABIs, physical devices,
runtime matrices, Keystore integration, Maven publication and application
lifecycle behavior remain deferred. `SDK-LIM-002` and `SDK-LIM-003` remain
effective.

## Compatibility and rollback

The Rust API and cross-language ABI stay at version 1. Rollback removes the
dedicated Android toolchain target, fixture, gate, workflow invocation and this
evidence. Host/Apple bindings and every generic crate remain unchanged.
