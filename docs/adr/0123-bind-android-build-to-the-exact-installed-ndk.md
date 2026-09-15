# ADR 0123: bind the Android build to the exact installed NDK

- **Status:** Accepted by project-sponsor direction
- **Date:** 2026-09-15
- **Issue:** [#276](https://github.com/hyperledger-identus/sdk-rust/issues/276)
- **Builds on:** ADR 0100, ADR 0120 and ADR 0122
- **Review no later than:** 2026-12-08 and before any Android support claim

## Context

Exact-merged-head slow run `34983050826` passed candidate, coverage,
performance, browser DID, both full Nix matrices and AOSP package installation.
The native Android verifier then rejected the packaged ELF because its NDK
build identity was not 12077973.

The workflow installs exact package `ndk;27.0.12077973`, and ADR 0100 requires
that version. The verifier nevertheless gave inherited `ANDROID_NDK_ROOT`
precedence over `$ANDROID_SDK_ROOT/ndk/27.0.12077973`. GitHub runner-image
revision `f95c0c791f690fa64eaf9788bea06643a4176db5` documents the current hosted
default as 27.3.13750724. The exact package was installed but bypassed.

Android's side-by-side model places a requested version under
`<android-sdk>/ndk/<version>`. The selected compiler and installed package can
therefore share the already validated SDK root and exact version without a
host-specific path.

## Decision

1. Derive the compiler root unconditionally as
   `$android_sdk/ndk/27.0.12077973`; do not use an ambient NDK root fallback.
2. Require that directory and its `source.properties` to exist, and require
   `Pkg.Revision` to identify exactly 27.0.12077973.
3. Bind `ANDROID_NDK`, `ANDROID_NDK_HOME`, and `ANDROID_NDK_ROOT` to the exact
   path for every child process. Remove `ANDROID_NDK_LATEST_HOME` from the
   verifier environment so later native build helpers cannot silently drift.
4. Keep the direct API-21 AArch64 linker path and existing ELF note assertions
   for NDK r27/build 12077973. Metadata validates selected tool provenance; the
   ELF independently validates the packaged artifact.
5. Record the selected `source.properties` SHA-256 in the evidence receipt.
   Do not record or depend on the hosted runner's absolute path.
6. Keep NDK version, AOSP image, API/ABI, Rust/Cargo, AAR/JNA inputs, behavior
   cases and Android/FFI support limitations unchanged.

## Consequences

The workflow's exact NDK install becomes an effective input rather than a
bypassable declaration. A runner default can change without silently changing
the SDK artifact, and future native build helpers inherit the reviewed path.

The NDK remains external `sdkmanager` test tooling. Exact package identity and
metadata are checked, but the remote archive is not vendored or checksum-pinned
by this repository. The hosted macOS canary remains authoritative because the
development host has no Android SDK.

No production Rust source, dependency, public API, ABI, wire format, feature,
MSRV, target tier, AAR shape or consumer migration changes.

## Alternatives considered

- Adopt ambient NDK 27.3.13750724: rejected as an unreviewed toolchain upgrade
  coupled to mutable runner state.
- Keep ambient fallback and weaken the expected build: rejected because the
  exact workflow install would remain misleading.
- Validate metadata only: rejected because it would not prove which linker
  produced the packaged library.
- Set aliases globally in the workflow: rejected because the leaf verifier can
  own, validate and constrain the native tool boundary more cohesively.

## Verification and rollback

Offline policy checks and mutations enforce SDK-relative selection, exact
metadata, child aliases, latest-alias removal, checksum receipt and existing
ELF identity. Protected PR CI validates the exact head. A complete post-merge
slow canary must pass exact installation, compilation, package inspection, AVD
boot and behavior. The first natural scheduled run remains issue #276's final
cadence acceptance.

Rollback reverts this ADR and verifier/policy changes, restoring the known-red
NDK mismatch without changing consumer state or released artifacts.
