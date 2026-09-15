# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/276
Constraint blockers: none

## Existing entries affected

- `SDK-ARCH-001`: Android runtime mechanics remain outside generic crates.
- `SDK-SUPPLY-001`: exact external NDK, AOSP, JNA, Gradle and Rust inputs remain
  explicit and fail closed.
- `SDK-LIM-002`: the UniFFI facade remains experimental and unpublished.
- `SDK-LIM-003`: Android remains not runtime-certified; two narrow receipts do
  not activate target-wide support.
- `SDK-REPO-001`: integration remains through protected `develop`.

## Introduced or changed constraints

The distributable SDK evidence SHALL continue to build and inspect exactly one
ARM64 ABI. A separate test-only x86_64 library/AAR MAY execute the same source,
generated Kotlin API, exact JNA dependency, and behavior cases only inside the
weekly/manual harness. The test artifact SHALL be labeled as test-only and
SHALL NOT enter the ARM64 package or any publication path.

Hosted runtime execution SHALL require Linux x86_64, usable KVM, the exact
API-35 default AOSP x86_64 image, bounded timeout, isolated state, and durable
failure diagnostics. Missing acceleration or image capability SHALL fail, not
skip or silently downgrade the proof.

## Introduced or changed limitations

The hosted runtime receipt executes a test-only x86_64 artifact and therefore
does not prove the ARM64 ELF executes on a device. ARM64 behavior retains the
existing 2026-09-09 local emulator receipt until a protected ARM64 runtime can
replace it. Neither ABI is supported, published, signed, device-tested, or
certified. Hosted SDK/image/KVM availability remains an external slow-lane
input.

## Consumer and product impact

No consumer receives a new ABI, artifact, dependency, API, support tier, wire
behavior, or migration. Oxid, Midnight, midnight-identity, NeoPRISM, Lace and
Apollo remain unchanged.

## Activation and rollback

Activation requires a protected merge and a successful full slow canary at the
exact merged `develop` SHA. Rollback restores the known-unrunnable hosted ARM64
AVD coupling without changing consumer artifacts. Reunifying package/runtime
proof requires a later protected ARM64 runner/device decision.

## Evidence

The failed canary, official runner/Android documentation, catalog hash, exact
package/ABI receipts, policy mutations, factory/OpenSpec validation, local Nix,
protected CI, hosted macOS package proof, hosted Linux KVM/runtime proof, and a
distinct architecture/security review form the evidence.
