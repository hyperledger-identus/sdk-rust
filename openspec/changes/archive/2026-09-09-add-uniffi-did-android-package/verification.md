# Verification receipt

Verification status: passed locally
Verification date: 2026-09-09
Base: develop@f157820aa312e56aa558361f8332609933c6b8e2
Evidence head: e00a221826d0d96c22ff2d3cc7199e2a1c281a25

## Android package and runtime receipt

- Rust: `rustc 1.98.1 (48a229cea 2026-09-01)` from the dedicated Nix bindings
  shell with the Android target and LLVM tools.
- Android inputs: NDK 27.0.12077973, minimum API 21, compile/target API 35,
  AGP 8.13.2, Gradle 8.14.4, Kotlin 2.2.20 and Nix JDK 17.0.19.
- Runtime image:
  `system-images;android-35;google_apis_playstore;arm64-v8a`; observed ABI
  `arm64-v8a` and API 35 in an isolated AVD.
- JNA: exact 5.18.1 Android AAR, SHA-256
  `7f053e3ec99e14dd71259c82c1c8a02738d64a13c31226b2acc170f3060951e0`.
- SDK ELF: 409,304 bytes; normalized SDK AAR: 274,774 bytes; consumer APK:
  1,527,525 bytes. Sizes are observations, not gates.
- Two independent builds produced byte-identical Rust libraries, generated
  Kotlin, dependency locks and normalized complete AAR trees. ABI/API/NDK,
  symbols, runtime needs, hardening, path hygiene and the single-native-payload
  contract passed inspection.
- The consumer loaded the local SDK AAR plus separate JNA and emitted the fixed
  success marker after all version, valid, invalid, oversized and redaction
  checks passed.

## Host and Apple regression evidence

- `check-uniffi-did-host.sh`: six Rust tests, strict Clippy, deterministic
  Swift/Kotlin generation, macOS Swift execution and Kotlin/JVM/JNA execution
  passed.
- `check-uniffi-did-apple.sh`: deterministic device/Simulator XCFramework,
  SwiftPM compilation and iPhone 15 Pro/iOS 17.5 arm64 Simulator behavior
  passed with the expanded bindings shell.

## Factory, dependency and exact-diff evidence

- Focused `identus-uniffi-did` test (6/6), strict Clippy and isolated bindgen
  Cargo check passed.
- `scripts/factory check`, research readiness, material constraint readiness,
  strict OpenSpec validation and diff whitespace checks passed.
- Root and bindgen Cargo locks and generic `crates/core`, `crates/did`,
  `crates/crypto` and `crates/jose` are unchanged from the recorded base.
- The three pre-finalization commits have verified GPG signatures and DCO
  trailers. The distinct exact-diff architecture/security review has no open
  finding.

## Full reproducible matrix

`nix flake check --print-build-logs` passed all 30 compatible aarch64-darwin
checks. This included Rust 1.98.1 native, all-feature, minimal-feature,
KMP-compatible, WASM, iOS and Android builds; strict Clippy; rustdoc;
formatting; factory/policy; dependency/license/advisory gates; and release
Nextest profiles. The workspace suite ran 673 tests: 673 passed and 22
configured diagnostics were skipped. KMP-compatible crypto ran 131/131 tests.

Nix reported x86_64-linux locally incompatible; hosted Linux fast CI is the
independent authority. Existing nonfatal offline yanked-index and macOS Nix
fixup-scanner diagnostics did not fail a derivation.

## Exclusions

Hosted CI remains mandatory before merge. Publication, signing, physical
devices, wider ABI/runtime coverage, Keystore integration, lifecycle behavior
and supported Android distribution remain excluded.
