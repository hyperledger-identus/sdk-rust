## MODIFIED Requirements

### Requirement: Emulator execution does not activate Android support

An isolated API-35 Android emulator consumer SHALL observe binding API version
`1` and the existing valid, invalid, oversized and redacted-error behavior
families. The evidence SHALL run only in the active hosted weekly/manual slow
lane or its exact local reproduction and SHALL NOT mutate a user AVD.
`SDK-LIM-002` and `SDK-LIM-003` SHALL remain effective.

The distributable package proof SHALL remain the exact API-21 ARM64 AAR and
ELF contract. Hosted runtime evidence MAY use a separately built, explicitly
test-only x86_64 AAR from the same Rust source, Cargo lock, UniFFI generator,
Kotlin/consumer source and exact JNA dependency when the hosted ARM64 runner
cannot provide nested virtualization. The test-only ABI SHALL NOT be added to
the distributable ARM64 AAR, publication metadata, or support matrix.

The hosted runtime SHALL use the API-35 default AOSP x86_64 image on a Linux
x86_64 runner with KVM hardware acceleration. Missing KVM, image, boot, install
or behavior capability SHALL fail closed and retain bounded diagnostics. The
test SHALL NOT depend on Google APIs or Play Store.

#### Scenario: one ephemeral emulator passes

- **WHEN** the generated test-only local AAR executes on the selected emulator
- **THEN** the receipt SHALL name its test-only role, SDK/image/API/ABI inputs
  while making no claim that the distributable ARM64 ELF executed, and no
  physical-device, other-ABI, publication, compatibility-matrix or support
  claim

#### Scenario: distributable and runtime ABIs remain separate

- **WHEN** the hosted slow lane constructs both evidence roles
- **THEN** the distributable SDK AAR contains only `arm64-v8a`, the runtime AAR
  contains only test-owned `x86_64`, and neither artifact is combined or
  published

#### Scenario: hosted acceleration is unavailable

- **WHEN** the Linux runtime job cannot use KVM or the emulator exits or times
  out before boot
- **THEN** the job fails and retains the emulator log and partial receipt for
  diagnosis rather than skipping or silently using software emulation

#### Scenario: Google services are not a runtime dependency

- **WHEN** the native DID smoke test provisions its API-35 x86_64 AVD
- **THEN** it selects the default AOSP image and makes no Google API, Play Store
  or additional license dependency part of the proof
