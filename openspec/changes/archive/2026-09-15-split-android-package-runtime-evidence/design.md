## Context

Run `34992633358` proved that the exact ARM64 library and AAR are valid through
compilation, deterministic comparison, ELF inspection, generated Kotlin,
Gradle library assembly, and consumer APK assembly. The AVD failed only after
those checks. Hosted arm64 macOS cannot supply the nested virtualization the
ARM64 guest needs, so retrying the same topology cannot establish a reliable
weekly gate.

The SDK must preserve two distinct truths: the intended package contains only
the reviewed ARM64 ABI; shared binding behavior executes on Android. CI
infrastructure must not force a second distributable ABI or pretend x86_64
execution is ARM64 execution.

## Decisions

### Parameterize one verifier around explicit evidence roles

Keep one Android verifier with two explicit modes. `arm64-package` runs on
macOS, builds twice for `aarch64-linux-android`, generates and normalizes the
ARM64 AAR, inspects the exact API/NDK/ELF/symbol/path contract, assembles the
consumer, writes a package receipt, and exits before AVD creation.

`x86_64-runtime` runs on Linux x86_64, builds a separate test-only
`x86_64-linux-android` AAR from the same crate/lock/template, assembles the same
consumer source, verifies the test ABI, boots the exact API-35 default AOSP
x86_64 image with KVM, and executes the unchanged success/failure markers. Its
artifact root and receipt label make the non-distributable role explicit.

### Keep the release boundary singular

The ARM64 AAR remains exactly one `jni/arm64-v8a` SDK library. The x86_64 AAR
exists only under an ignored runtime-evidence tree in its job. No combined AAR,
workspace target tier, publication metadata, or support claim is introduced.

### Split hosted authority by capability

The macOS full matrix retains host and Apple behavior plus ARM64 Android
package evidence. A dedicated Ubuntu x86_64 job owns Android runtime execution
because GitHub documents Android hardware acceleration there. It checks
`/dev/kvm` before work and never falls back to software emulation. The final
immutable receipt includes the new job result.

### Preserve diagnostics on every exit

The verifier writes role/version/package receipts incrementally and prints the
bounded tail of `emulator.log` if startup exits or times out. The workflow
uploads the Android evidence tree with `if: always()` and seven-day retention,
so failed canaries retain the process cause without exposing user data.

## Risks and mitigations

- Runtime ABI differs from the SDK ABI: label it test-only everywhere; retain
  independent ARM64 ELF/package proof and do not combine artifacts.
- Source/API drift between roles: both modes use one script, crate, lock,
  generator config, package templates, JNA hash, and behavior cases.
- Linux acceleration unavailable: check KVM explicitly and fail with retained
  diagnostics; never silently skip or use an unbounded software emulator.
- Job cost/disk growth: dedicate one weekly/manual job with a bounded timeout,
  exact packages, isolated output, and seven-day retention; fast CI is unchanged.
- Failure logs disclose environment paths: receipts avoid absolute paths and
  diagnostics contain only synthetic build/emulator state.

## Alternatives

Keeping ARM64 runtime on hosted macOS contradicts the documented runner
capability. Removing runtime proof weakens the contract. Publishing x86_64
expands the product surface. A third-party action adds an avoidable dependency.
A self-hosted ARM64 device/runner is deferred until ownership and security are
explicitly provisioned.

## Rollback

Revert ADR 0124, verifier modes, x86_64 bindings target, dedicated job, policy
mutations and spec deltas. This restores the prior ARM64-only artifact plus
known-red hosted emulator topology without touching consumers or releases.
