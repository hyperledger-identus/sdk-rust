## Context

The accepted host foundation already owns the Swift ABI and generator lock. The
Apple package must compose those artifacts without making Xcode or SwiftPM part
of a generic Rust crate and without committing generated binaries.

## Goals

- Prove exact arm64 device/Simulator packaging and SwiftPM consumption.
- Preserve API version, bounds, redaction and domain isolation.
- Keep build products ephemeral, deterministic and slow-lane only.
- Make unsupported surfaces and the multi-static-library risk explicit.

## Decisions

### Static library XCFramework

Use Rust `staticlib` for `aarch64-apple-ios` and
`aarch64-apple-ios-sim`. Pass the two archives independently to
`xcodebuild -create-xcframework -library ... -headers ...`; never combine
platforms with `lipo`.

### Generated SwiftPM wrapper

Use UniFFI's Swift-specific library-mode generator to produce Swift source, the
C header and an XCFramework-compatible `module.modulemap`. A Swift source target
depends on the local binary target. Only the package template and behavioral
test source are tracked; generated sources and binaries live under `target/`.

### Reproducibility contract

The script builds and assembles two independent package trees. It compares
their complete normalized contents and checks architecture, platform variant,
minimum deployment target, required exported symbols and absence of worktree
paths. Size is measured and reported, not gated by an invented budget.

### Execution contract

Use `xcodebuild test` against one deterministic available arm64 iOS Simulator
destination. Tests repeat the host contract for version, valid DID/DID URL,
invalid input, oversized input and redacted errors. Failure to find a compatible
runtime is an external evidence blocker and must not be converted into a pass.

### Distribution posture

Do not sign, zip, upload or create a remote SwiftPM target. One local package is
not a release artifact. Future distribution should prefer one aggregate Rust
static library rather than multiple leaf Rust XCFrameworks until composition is
proven.

## Risks and mitigations

- Xcode 26 module import drift: execute the exact local binary-target package;
  do not rely on host `swiftc` evidence or unchecked flags.
- Stale generated API: reuse normalized snapshot checks and generate from the
  same library metadata/version.
- Platform confusion: inspect XCFramework `Info.plist` and archive load
  commands, and keep device/Simulator archives separate.
- Accidental artifact commit: emit under `target/` and assert repository status
  remains free of generated package files.
- Runtime symbol collisions: record single-binary scope and defer aggregate SDK
  packaging design.

## Rollback

Remove the staticlib crate type, Swift generator binary, Apple fixture/script,
weekly step and ADR. The original host foundation remains functional.
