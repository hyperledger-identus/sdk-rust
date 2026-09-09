## Context

The accepted leaf binding crate and generator lock already own the Kotlin ABI.
Android packaging must consume those inputs without adding Android concerns to
generic Rust crates or committing generated artifacts.

## Decisions

### One narrow SDK AAR

Build the existing cdylib for `aarch64-linux-android` with the NDK API-21
clang linker. Generate Kotlin from that exact library and compile a library
module whose AAR contains the generated classes and
`jni/arm64-v8a/libidentus_uniffi_did.so`.

### Explicit JNA consumer dependency

Do not fatten or shadow the SDK AAR with JNA. A separate consumer application
depends on the local SDK AAR and exact JNA 5.18.1 `@aar`. This proves the local
contract while leaving future Maven publication to express the same dependency
in metadata.

### Determinism and inspection

Build two independent Rust/generated/Gradle trees. Normalize only archive
metadata, then compare every path and byte. Inspect ELF architecture, Android
API/NDK note, hardening flags, needed libraries, public symbols and absolute
paths. Record sizes without inventing a threshold.

### Ephemeral runtime proof

Create an isolated AVD under `target/` from an exact arm64 API-35 image, boot it
headlessly, install a minimal consumer, and invoke the generated public API.
The app emits a fixed success/failure marker after checking version, valid,
invalid, oversized and redacted behavior. The gate treats missing emulator
capability as failure, not success, and never mutates the user's named AVDs.

## Risks and mitigations

- Dependency/resource collisions: JNA remains a separate exact AAR.
- Stale bindings: generate twice from the built library and compare output.
- ABI/platform confusion: inspect the actual packaged ELF and AAR paths.
- Host contamination: isolate Gradle homes, Android AVD state and build output
  beneath `target/`.
- Support inflation: retain all mobile limitations and publish nothing.

## Rollback

Delete the isolated toolchain target, fixture, script, slow-lane call, ADR and
evidence. The accepted host/Apple foundation continues unchanged.
