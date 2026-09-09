# Constraint and limitation impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/228
Constraint blockers: none

## Existing entries affected

- `SDK-SEC-001` continues to prohibit authored unsafe Rust; static packaging
  does not create a first-party exception.
- `SDK-SEC-002` continues to prohibit raw secret material across FFI; the
  package contains public identifier behavior only.
- `SDK-SEC-003` retains the 2,048-byte DID and 4,096-byte DID URL bounds.
- `SDK-COMPAT-002`, `SDK-COMPAT-004` and `SDK-COMPAT-005` retain Rust 1.98.1.
- `SDK-ARCH-001` keeps generic crates free of Apple, UniFFI and product policy.
- `SDK-LIM-002` remains effective. A local unsigned package and one Simulator
  receipt are not a supported or distributed mobile SDK.
- `SDK-LIM-003` remains effective for physical-device and complete platform
  portability evidence.

## Introduced or changed constraints

The leaf binding crate may emit a static library in addition to its current
artifacts. Device and Simulator variants must remain distinct and must be built
from the same source, Cargo lock, Rust 1.98.1 and API version. The package must
use the exact generated Swift/header contract and the reviewed fail-closed
static-library module-map normalization, set an explicit minimum iOS version,
expose no absolute build path, and contain no generated cache or build directory
in Git. No unchecked or unsafe Swift compiler flag is permitted.

The default development shell and Linux fast line remain unchanged. Apple
construction and execution run in the dedicated binding environment and the
weekly/manual slow macOS workflow. The package is local evidence only and is
not signed, uploaded or referenced by a remote SwiftPM checksum.

## Introduced or changed limitations

- Only arm64 iOS and arm64 iOS Simulator variants are constructed. Intel
  Simulator, Catalyst and other Apple platforms are not claimed.
- One available Simulator runtime is execution evidence; physical devices,
  installation, signing and older OS runtimes are unverified.
- The package contains one Rust static library. Composition with another Rust
  static XCFramework is unverified and may produce duplicate runtime symbols.
- Xcode and Apple SDK inputs are locally/hosted observed rather than fetched by
  Nix; exact versions are recorded in receipts.
- No public Swift source/API stability, package URL, checksum or semantic
  version is activated.

## Consumer and product impact

Existing Rust and host Swift/Kotlin consumers are unchanged. Future Apple
integrators gain a reproducible reference shape but no supported download.
Oxid, Midnight, midnight-identity, NeoPRISM, Lace and Apollo remain unchanged.

## Activation and rollback

After merge, `develop` contains Apple package evidence and a slow-lane gate;
`SDK-LIM-002` remains the effective authority. Activation requires a separate
material change with distribution, physical-device, signing, compatibility and
Android evidence. Rollback is deletion of the static artifact mode, generator
entry point, fixture and gate.

## Evidence

Evidence includes exact tool versions, dependency/license non-drift, double
build/package comparison, architecture and minimum-platform inspection,
public-symbol and absolute-path checks, SwiftPM build, iOS Simulator runtime,
full factory/Cargo/Nix gates and distinct architecture/security review.
