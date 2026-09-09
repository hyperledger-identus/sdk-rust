# Constraint and limitation impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/230
Constraint blockers: none

## Existing entries affected

- `SDK-SEC-001` continues to prohibit authored unsafe Rust.
- `SDK-SEC-002` continues to prohibit raw secret material across FFI.
- `SDK-SEC-003` retains the 2,048-byte DID and 4,096-byte DID URL bounds.
- `SDK-COMPAT-002`, `SDK-COMPAT-004` and `SDK-COMPAT-005` retain Rust 1.98.1.
- `SDK-ARCH-001` keeps Android/UniFFI/build mechanics outside generic crates.
- `SDK-LIM-002` and `SDK-LIM-003` remain effective; a local AAR and one
  emulator are not a supported or portable mobile SDK.

## Introduced or changed constraints

The SDK AAR must contain only one `arm64-v8a` SDK native library. Exact JNA
5.18.1 is an explicit consumer dependency and must not be merged or shadowed
into the SDK AAR. Builds use exact Rust, NDK, Gradle, AGP, Kotlin, JDK and API
inputs, compare two normalized complete package trees, and reject unexpected
architecture, API, ELF, symbol or path evidence.

Generated code, binaries, emulator state and Android SDK downloads remain
under ignored paths. Existing user AVDs are not read or changed. The default
shell and fast Linux line remain unchanged; the gate is weekly/manual macOS.

## Introduced or changed limitations

- Only arm64-v8a and one API-35 arm64 emulator are proven.
- Physical devices, older/newer runtime matrices, other ABIs, Maven metadata,
  signing, publication, Android Keystore and application lifecycle behavior
  remain unverified.
- Hosted Android SDK/system-image and Apple virtualization availability are
  observed slow-lane inputs, not release guarantees.
- JNA remains dependency-owned native code behind generated types.

## Consumer and product impact

Existing Rust, host and Apple consumers are unchanged. An Android experiment
gains a reproducible local AAR and explicit JNA integration shape, but Oxid,
Midnight, midnight-identity, NeoPRISM, Lace and Apollo remain unchanged and no
supported download is created.

## Activation and rollback

Remove the Android target from the bindings toolchain, Android package/test
template, gate invocation, ADR and support evidence. ABI version 1, host and
Apple proofs, and all generic crates remain unchanged.

Activation as a supported SDK requires a separate directed change covering
publication, metadata, signing, physical devices, ABI/runtime matrices,
consumer ownership and compatibility policy.

## Evidence

Evidence includes exact tool and artifact versions, double-build/package
comparison, Gradle dependency locking, AAR and ELF inventory, API/ABI/symbol
inspection, path hygiene, emulator behavior, full repository gates and a
distinct exact-diff architecture/security review.
