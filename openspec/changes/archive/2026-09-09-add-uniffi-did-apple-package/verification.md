# Verification receipt

Verification status: passed locally
Verification date: 2026-09-09
Base: develop@f941c73f1e9f038f6795a14828b341138ee40430
Evidence head: 0555a2827b8a66e07a76b6a5ecbb7aa8ccf24d05

## Apple construction and runtime receipt

- Rust: `rustc 1.98.1 (48a229cea 2026-09-01)` from the dedicated Nix bindings
  shell with `aarch64-apple-ios`, `aarch64-apple-ios-sim` and LLVM tools.
- Xcode: 26.4 build 17E192; Apple Swift 6.3; iPhoneOS and iPhoneSimulator SDK
  26.4; explicit minimum target iOS 15.0.
- Variants: arm64 iOS device and arm64 iOS Simulator.
- Runtime: iPhone 15 Pro, iOS 17.5
  (`com.apple.CoreSimulator.SimRuntime.iOS-17-5`), arm64; initial state
  `Shutdown`.
- Device archive: 20,046,024 bytes; Simulator archive: 20,038,248 bytes;
  complete local package: 40,194,048 bytes. Sizes are observations, not gates.
- Two independent builds produced byte-identical archives, generated files and
  normalized complete package trees. Architecture, platform/minimum version,
  plist variants, headers, module maps, required symbols and absolute-path
  absence passed inspection.
- `xcodebuild test` compiled, linked and executed API version, valid DID/DID
  URL, invalid input, oversized input and redacted-error behavior through the
  public Swift product. Result: passed.

## Focused and host evidence

- `cargo test --locked -p identus-uniffi-did --all-targets`: 6/6 passed.
- Focused strict Clippy for `identus-uniffi-did`: passed.
- `cargo check --locked --manifest-path tools/uniffi-bindgen/Cargo.toml
  --all-targets`: passed.
- `nix develop .#bindings --command ./scripts/check-uniffi-did-host.sh`:
  deterministic Swift/Kotlin generation and macOS Swift plus Kotlin/JVM host
  execution passed.
- `nix develop .#bindings --command ./scripts/check-uniffi-did-apple.sh`:
  deterministic Apple construction and Simulator execution passed.
- ShellCheck for both binding scripts: passed.

## Factory, dependency and exact-diff evidence

- `scripts/factory check`: passed before the evidence-only finalization.
- Research readiness, material-constraint readiness, strict OpenSpec validation,
  formatting and diff whitespace checks passed.
- `Cargo.lock` and `tools/uniffi-bindgen/Cargo.lock` are unchanged from the
  recorded develop base; no dependency, feature or license drift exists.
- The complete diff changes no file under `crates/core`, `crates/did`,
  `crates/crypto` or `crates/jose`. Changed first-party Rust contains no
  `unsafe` token.
- All three pre-finalization commits have verified GPG signatures and
  `Signed-off-by` DCO trailers.
- The distinct exact-diff architecture/security review is recorded in
  `review.md` with no unresolved finding.

## Full reproducible matrix

`nix flake check --print-build-logs` passed all 34 compatible aarch64-darwin
checks. This included Rust 1.98.1 native, all-feature, minimal-feature,
KMP-compatible, WASM, iOS and Android builds; strict Clippy; rustdoc;
formatting; factory and policy checks; dependency, license and advisory gates;
and release Nextest profiles. The principal workspace suite ran 673 tests:
673 passed and 22 configured diagnostics were skipped. The KMP-compatible
crypto profile ran 131/131 tests; all entropy feature profiles passed.

Nix reported x86_64-linux as locally incompatible; hosted Ubuntu fast CI is
the independent Linux authority. Existing nonfatal offline yanked-index and
macOS fixup scanner diagnostics did not fail a derivation.

## Exclusions

Hosted CI remains mandatory before merge. Publication, signing, physical-device
execution, older-runtime coverage, Android packaging/runtime, multi-static-
library composition and downstream support remain explicitly excluded.
