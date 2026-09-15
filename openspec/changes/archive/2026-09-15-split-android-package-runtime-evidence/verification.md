# Verification receipt

- **Verification status:** passed locally
- **Verification date:** 2026-09-16
- **Planning head:** `99e5af77d00ed2408a9d0b3ed6c588e7da1b968e`
- **Preflight head:** `26aaaefd8cc6b71254e966078737197c7555fa32`
- **Implementation head:** `59dd76abeed6a3ab72d8626e25c322a6065ea0c2`
- **Base:** `develop@f2221d444e85e4b5e4bffdc0c0fd2d7e43ec83da`

## Passed locally

- `python3 scripts/tests/support-policy.py`: all 209 mutation tests passed.
- `python3 scripts/check-support-policy.py .`: support contract passed.
- `bash -n scripts/check-uniffi-did-android.sh`: passed.
- Pinned `shellcheck scripts/check-uniffi-did-android.sh`: passed.
- Pinned `actionlint .github/workflows/nix-checks.yml`: passed.
- `./scripts/factory check --change split-android-package-runtime-evidence`:
  all factory contracts and 71 OpenSpec items passed.
- `nix flake check --fallback`: all compatible `aarch64-darwin` checks passed,
  including Rust 1.98.1 build/test/Clippy/docs, feature and MSRV evidence,
  Android/iOS/WASM cross-builds, dependency policy and text/TOML/Nix linting.
- The bindings shell built the `x86_64-linux-android` Rust standard library and
  resolved its target library directory from the pinned 1.98.1 toolchain.
- `git diff --check`: passed.
- All implementation commits have valid local GPG signatures and DCO trailers.

## Hosted evidence boundary

This ARM64 macOS host has no Android SDK or KVM, so it cannot execute either
new CI role end to end. The protected PR must prove the exact-head fast gate.
After merge, a full manually dispatched slow run on the exact merged SHA is the
authority for deterministic ARM64 package construction on hosted macOS and the
test-only x86_64 behavior receipt on hosted Linux with KVM.

Issue #276 remains open through the first healthy natural Monday schedule. The
post-merge canary does not promote Android or FFI support and does not prove
that the ARM64 library executes on an ARM64 device.

## Deliberate exclusions

No release AAR gains x86_64, no package is published or signed, and no physical
device, ARM64 runtime, wider API/ABI matrix, Android Keystore, application
lifecycle, performance or certification claim is added. Hosted emulator,
platform-tools and command-line-tool revisions remain runner-provided; only the
NDK, platform, system-image package identity, Rust, JDK, Gradle, AGP, Kotlin and
JNA inputs named by the existing contract are fixed.
