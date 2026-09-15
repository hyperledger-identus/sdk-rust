## Why

Exact-merged-head slow run `34992633358` proved the deterministic ARM64 AAR,
exact NDK 27.0.12077973 compiler/ELF identity, consumer APK assembly, and every
other slow job. The final API-35 ARM64 AVD then exited before boot on hosted
`macos-latest`.

GitHub documents that hosted arm64 macOS runners do not support nested
virtualization. The current contract nevertheless requires an ARM64 Android
VM inside that runner, so a valid SDK artifact cannot make the weekly lane
green. The verifier also redirects its emulator log into ephemeral job storage
without printing or uploading it when startup fails.

## What changes

- Accept ADR 0124 separating the distributable ARM64 package proof from the
  behavior-equivalent test-only Android runtime proof.
- Keep the exact ARM64 AAR, API-21 ELF, NDK, determinism, dependency, symbol,
  hardening, and path-hygiene checks on hosted macOS; do not run an Android VM
  there.
- Run the same generated Kotlin API and Rust source behavior through an
  explicitly test-only `x86_64` AAR on an API-35 default AOSP emulator in a
  dedicated hardware-accelerated hosted Linux job.
- Persist package/runtime receipts and emulator diagnostics as exact-run
  artifacts, including failure paths.
- Extend the offline policy and mutation suite so release and test-only ABIs,
  runner roles, exact packages, KVM admission, and evidence retention cannot
  silently collapse into one support claim.

## Capabilities

### Modified capabilities

- `native-did-bindings`: retains one ARM64 SDK artifact while moving runtime
  behavior to a separate test-only x86_64 artifact.
- `weekly-slow-evidence`: adds a bounded Linux Android runtime job and durable
  diagnostics while keeping the macOS Apple/ARM64 package gate.

## Non-goals

- No second ABI in the distributable SDK AAR, Android support activation,
  Maven publication, physical-device proof, API expansion, NDK upgrade, Google
  APIs/Play image, consumer change, or production Rust behavior change.
- No claim that x86_64 is a supported SDK target; it exists only inside the
  ephemeral behavior harness.

## Delivery

Issue #276 owns this zero-stack-depth repair. After protected merge, a complete
manual slow canary must pass at the exact merged `develop` SHA. The issue stays
open for the first successful natural Monday schedule receipt.
