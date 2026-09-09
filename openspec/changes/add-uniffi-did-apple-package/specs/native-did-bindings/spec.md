## ADDED Requirements

### Requirement: Apple package keeps device and Simulator variants distinct

The experimental Apple package SHALL build `identus-uniffi-did` as a static
library from the same source, lock and Rust 1.98.1 toolchain for arm64 iOS and
arm64 iOS Simulator. It SHALL pass the variants separately, with the same
generated public header, to `xcodebuild -create-xcframework` and SHALL NOT merge
device and Simulator objects with `lipo`.

#### Scenario: XCFramework variants are inspected

- **WHEN** the package gate constructs the XCFramework
- **THEN** its manifest and binaries SHALL identify one arm64 iOS variant and
  one arm64 iOS Simulator variant with the declared minimum iOS version

### Requirement: SwiftPM composes generated Swift over one binary target

The local Swift package SHALL expose generated `IdentusDid` Swift source in a
source target that depends on one `IdentusDidFFI` local binary target. The
binary target SHALL contain an XCFramework-compatible `module.modulemap`, the
generated C header and one Rust static library per declared variant. Generation
SHALL use the exact separately locked UniFFI 0.32.0 tool. A static-library
module-map adapter MAY remove the generated `framework` qualifier and explicit
Darwin/builtin `use` declarations only after matching the complete expected
generator output; unexpected generator drift SHALL fail closed. The generated
C header and Swift source SHALL remain unmodified, and unchecked or unsafe
Swift compiler flags SHALL NOT be used.

#### Scenario: package imports the public module

- **WHEN** Xcode builds a test target that depends only on the `IdentusDid`
  product
- **THEN** Swift SHALL import the generated API and link the matching Simulator
  binary without caller-owned header, module or linker configuration

### Requirement: Apple package construction is deterministic and ephemeral

Two builds from the same source and locks SHALL produce byte-identical static
archives, generated sources/headers/module maps and normalized XCFramework and
Swift-package trees. The gate SHALL inspect required public symbols, reject
absolute worktree paths, record artifact sizes without inventing a threshold,
and keep generated package contents under ignored `target/` paths.

#### Scenario: package generation is repeated

- **WHEN** the Apple gate constructs two independent output trees
- **THEN** complete normalized content comparison SHALL pass before consumer
  compilation or identify the exact differing path

### Requirement: Simulator execution does not activate Apple support

An Xcode-driven iOS Simulator test SHALL observe binding API version `1` and
the existing valid, invalid, oversized and redacted-error behavior families.
This evidence SHALL run in the weekly/manual slow macOS lane. `SDK-LIM-002`
SHALL remain effective because the package is unsigned, unpublished and not
executed on a physical device, and because multi-Rust-static-library composition
is unproven.

#### Scenario: local package passes on one Simulator runtime

- **WHEN** the generated local Swift package test passes on the selected arm64
  iOS Simulator
- **THEN** the receipt SHALL name Xcode, SDK, runtime and architecture while
  making no physical-device, distribution, older-runtime or public support claim
