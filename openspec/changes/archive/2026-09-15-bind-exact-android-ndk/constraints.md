# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/276
Constraint blockers: none

## Existing entries affected

- `SDK-SUPPLY-001`: the exact external NDK package becomes the exact compiler
  source rather than an installed but bypassable input.
- `SDK-LIM-002` and `SDK-LIM-003`: Android/FFI support remains experimental.
- `SDK-LIM-004`: runtime proof remains weekly/manual slow evidence.
- `SDK-REPO-001`: delivery remains through protected `develop`.

## Introduced or changed constraints

Android native compilation SHALL use
`$android_sdk/ndk/27.0.12077973` selected from the same SDK root used by the
workflow installer. Ambient hosted-runner NDK aliases SHALL NOT choose the
compiler. Source metadata and artifact ELF identity SHALL independently match
the accepted revision/build, or verification SHALL fail closed.

## Introduced or changed limitations

NDK 27.0.12077973 remains externally downloaded test tooling rather than a
vendored archive. The workflow pins the SDK package identity but not its remote
archive checksum. Only the existing arm64 API-21 build and API-35 AOSP runtime
are proven. This host cannot execute the native gate.

## Consumer and product impact

No public API, ABI, AAR shape, wire behavior, Rust dependency, feature, MSRV or
support tier changes. The repair removes an ambient build input only.

## Activation and rollback

Activation requires protected merge and a successful complete slow canary at
the exact merged `develop` SHA. Rollback restores a known-red provenance
mismatch without consumer migration.

## Evidence

Official runner image documentation, official Android side-by-side NDK
guidance, canary logs, mutation tests, factory/OpenSpec validation, local Nix,
protected CI and the hosted macOS canary form the evidence.
