# ADR 0124: separate Android package and runtime evidence

- **Status:** Accepted under standing agent authority
- **Date:** 2026-09-16
- **Issue:** [#276](https://github.com/hyperledger-identus/sdk-rust/issues/276)
- **Builds on:** ADR 0100, ADR 0120, ADR 0122 and ADR 0123
- **Review no later than:** 2026-12-08 and before any Android support claim

## Context

Exact-merged-head slow run `34992633358` passed deterministic ARM64 library and
AAR construction, exact NDK metadata and ELF identity, Kotlin generation,
library assembly and consumer APK assembly. The API-35 ARM64 AVD then exited
before boot on hosted `macos-latest`.

GitHub documents that nested virtualization is unavailable on its arm64 macOS
runners. Android documents that accelerated VM execution requires matching
host/guest architecture and a host hypervisor. Requiring an ARM64 guest inside
that hosted VM therefore couples SDK evidence to an unsupported topology.
GitHub separately documents Android hardware acceleration on hosted Linux.

Artifact correctness and runtime behavior are distinct claims. The intended
SDK AAR must remain ARM64-only, while the same Rust/Kotlin behavior can be
exercised through a non-distributable test ABI on supported CI infrastructure.

## Decision

1. Keep exact ARM64 package evidence on hosted macOS: two deterministic
   `aarch64-linux-android` builds, one-ABI AAR, NDK/API/ELF/symbol/hardening/path
   inspection, generated Kotlin and consumer assembly.
2. Do not start an Android VM on hosted arm64 macOS.
3. Add a dedicated hosted Linux x86_64 weekly/manual job. It requires usable
   KVM and the exact API-35 default AOSP x86_64 image, builds a separate
   `x86_64-linux-android` AAR from the same crate/lock/generator/templates, and
   executes the unchanged synthetic behavior contract.
4. Label the runtime library, AAR, directories and receipt `test-only`. Never
   combine it with the ARM64 AAR, publication metadata or target support list.
5. Keep exact Rust 1.98.1, NDK 27.0.12077973, API 21, JNA 5.18.1, Gradle/AGP/
   Kotlin/JDK inputs and default AOSP/no-blanket-license policy.
6. Upload role-specific receipts and available diagnostics on both success and
   failure with exact SHA/run-attempt identity and seven-day retention. A
   missing KVM/image, emulator exit/timeout, or behavior failure remains red.
7. Keep `SDK-LIM-002` and `SDK-LIM-003` effective. x86_64 execution does not
   prove ARM64 device execution or activate any Android/FFI support claim.

## Consequences

The weekly lane can answer both evidence questions using hosted capabilities
that actually exist. The ARM64 consumer artifact does not expand, and the
test-only ABI cannot be mistaken for a release input. Failure diagnosis becomes
durable instead of disappearing with the runner workspace.

The hosted runtime receipt no longer proves that the ARM64 ELF itself executes.
The prior local ARM64 emulator receipt remains historical evidence, not active
hosted certification. A protected self-hosted ARM64 runner or device lab is the
preferred future replacement when ownership, credentials, and availability are
explicitly governed.

One additional weekly Linux job and Rust standard-library target increase slow
cost. The PR fast lane, public crates, Cargo dependency cone, API, ABI, wire
behavior, supported targets and consumers remain unchanged.

## Alternatives considered

- Retry ARM64 on hosted arm64 macOS: rejected because the runner explicitly
  lacks nested virtualization.
- Remove runtime execution: rejected because it weakens binding evidence.
- Add x86_64 to the SDK AAR: rejected because CI must not create a new consumer
  ABI or support promise.
- Use a third-party emulator action: rejected because direct exact commands are
  sufficient and avoid another action dependency.
- Provision ARM64 hardware now: deferred because it requires protected
  infrastructure, credentials, ownership and availability policy.

## Verification and rollback

Offline policy mutations enforce role/runner/ABI/package separation, KVM
admission, exact inputs, durable artifacts, and final receipt binding. Local
compatible Nix and protected PR CI validate the exact head. A post-merge full
canary must pass both hosted roles before the first natural schedule can close
issue #276.

Rollback reverts this ADR, x86_64 bindings target, verifier modes, dedicated
job, artifacts, policy checks and spec deltas. It restores the known-red hosted
ARM64-emulator topology without changing a consumer or released artifact.
