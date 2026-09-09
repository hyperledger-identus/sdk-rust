# Apple package research

Research class: storage-ffi
Research status: ready
Decision date: 2026-09-09
Source retrieval date: 2026-09-09
Research blockers: none

## Problem and existing implementation

The current implementation is the accepted host foundation in
`crates/uniffi-did`, its separately locked bindgen tool and host validation
script. The concrete consumer call shape is unchanged: Swift calls
`bindingApiVersion()`, `parseDid(_:)` and `parseDidUrl(_:)` and receives owned
records or closed `DidBindingError` cases. This change packages that already
accepted ABI; it does not add functions, handles, secrets or mutable state.

The package shape is:

1. one `IdentusDidFFI` local binary target backed by a static-library
   XCFramework containing distinct iOS and iOS Simulator variants;
2. one `IdentusDid` Swift source target containing the generated UniFFI source
   and depending on `IdentusDidFFI`;
3. one test target importing only the public `IdentusDid` product.

The host foundation proves generated Swift behavior against a macOS dynamic
library, but does not prove iOS target compilation, static linking,
device/Simulator slice selection, SwiftPM binary-target import, or execution in
an iOS runtime. `SDK-LIM-002` therefore remains effective before and after this
change.

## Normative sources

- Apple, "Creating a multi-platform binary framework bundle":
  https://developer.apple.com/documentation/xcode/creating-a-multi-platform-binary-framework-bundle
- Apple PackageDescription `binaryTarget(name:path:)`:
  https://developer.apple.com/documentation/packagedescription/target/binarytarget(name:path:)
- UniFFI 0.32 foreign-language bindings and library-mode generation:
  https://mozilla.github.io/uniffi-rs/latest/tutorial/foreign_language_bindings.html
- UniFFI 0.32 module guide and XCFramework module-map convention:
  https://mozilla.github.io/uniffi-rs/latest/tutorial/udl_file.html
- UniFFI 0.32.0 source, tag `v0.32.0`, commit
  `5c7b73906358e1a7acdc1bdc7bf5cd86fb27e44c`:
  https://github.com/mozilla/uniffi-rs/tree/v0.32.0
- UniFFI Xcode module-map compatibility report #2917:
  https://github.com/mozilla/uniffi-rs/issues/2917
- UniFFI multiple-static-library composition report #1710:
  https://github.com/mozilla/uniffi-rs/issues/1710

Apple and UniFFI documentation are mechanism authorities. The two upstream
issues are risk evidence, not normative support promises. The repository's
existing native-binding specification, constraints and issue #228 own product
scope.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| Rust `staticlib` plus Apple XCFramework | Rust 1.98.1 / Xcode 26.4 | `conditional-adopt` | It is the native Apple packaging shape and can prove separate device/Simulator slices without a new Rust dependency. | Reconsider if exact local linking fails, archive output is nondeterministic, or a supported distribution shape requires a different ABI. |
| Generated Swift source target over a local binary target | UniFFI 0.32.0 / `5c7b7390` | `conditional-adopt` | Keeps UniFFI's generated API while SwiftPM owns selection and linking of the XCFramework. | Reconsider if normalized static module import or runtime evidence regresses. |
| Strict static-library module-map normalizer | repository-local | `conditional-adopt` | Xcode 26.4 proves UniFFI's generated framework declaration and builtin `use` lines do not import from a static-library binary target; an exact-input adapter can fail on generator drift and emit Apple's plain module shape. | UniFFI emits an Xcode-compatible static-library module map or SwiftPM/Xcode changes the required form. |
| Free-form hand-authored C header, module map or Swift bridge | repository-local | `not-adopt` | It would create a second ABI definition and conceal generator compatibility defects. | Reconsider only if an approved versioned ABI intentionally becomes independent of UniFFI. |
| `lipo` device and Simulator archives together | Apple toolchain | `not-adopt` | Apple requires distinct platform variants in an XCFramework. | Never for device/Simulator composition. |
| Multiple leaf Rust static XCFrameworks | current upstream evidence | `defer` | Duplicate Rust runtime symbols are a known integration risk and this slice has only one library. | An aggregate native SDK design or exact multi-library composition proof is approved. |
| Remote SwiftPM binary target/publication | not applicable | `defer` | Signing, archive checksum, hosting, release ownership and compatibility are outside this evidence slice. | A named consumer and distribution/support ADR approve publication. |

## Compatibility and dependency evidence

- SDK Rust: 1.98.1; installed targets include `aarch64-apple-ios` and
  `aarch64-apple-ios-sim`.
- UniFFI: exact 0.32.0 from `mozilla/uniffi-rs` commit
  `5c7b73906358e1a7acdc1bdc7bf5cd86fb27e44c`, MPL-2.0. The existing separately
  locked tool already contains the required generator dependency cone.
- Xcode: 26.4 build 17E192; Apple Clang 21.0.0; Apple Swift 6.3.
- Local SDKs: iPhoneOS 26.4 and iPhoneSimulator 26.4. Available runtimes include
  iOS 17.5 and 26.4 on arm64 Apple Silicon.
- Swift tools version: 6.0, with package platform floor `.iOS(.v15)` and Rust
  build environment `IPHONEOS_DEPLOYMENT_TARGET=15.0`.

MSRV remains the repository's evidence-driven Rust 1.85 floor; primary and
packaging evidence use the pinned Rust 1.98.1 etalon. The exact version and
features of the existing UniFFI dependency remain 0.32.0 with the already
isolated generator feature set; no new feature is activated in the runtime
workspace.

These local Xcode/SDK versions are evidence inputs, not repository-pinned
downloads. Rust and UniFFI remain pinned by the repository. The slow workflow
records hosted versions and fails on incompatible drift.

The direct and resolved dependency cone is unchanged. The leaf crate's public
Rust API and UniFFI ABI version remain unchanged. Public and wire compatibility
remain unchanged because this is packaging of ABI version 1, not an interface
extension. Adding
`staticlib` changes artifact output only. The root runtime graph and separately
locked generator graph add no package, feature, license or native dependency.
No external type crosses the SDK facade boundary. Android, WASM, desktop and
downstream repositories are unchanged.

License and provenance remain Apache-2.0 for SDK-authored files and MPL-2.0 for
the exact pinned UniFFI source. Supply-chain evidence remains the two existing
lockfiles, repository-pinned Rust/Nix inputs, cargo-deny/audit gates and absence
of a dependency diff. Protocol/draft currency is not applicable to the Apple
packaging mechanism; the DID value behavior remains governed by final W3C DID
Core 1.0 through the already accepted host specification.

## Security, privacy and maintenance evidence

The reachable interface remains bounded public DID/DID URL text. It contains no
secret, key, signing, storage, network, callback, async or object-handle
surface. Existing redaction and input bounds remain enforced in Rust and are
re-executed from Swift. No first-party unsafe Rust is introduced; dependency-
owned FFI/native behavior remains quarantined behind the leaf crate.

The package gate rejects absolute checkout paths, compares complete generated
trees, inspects public symbols and records artifact size. Generated sources and
binaries live under ignored `target/`; only templates and verification logic
are maintained. Xcode/SDK drift is an explicit slow-lane compatibility signal,
not a reason to weaken the fast Linux line.

### Primary-source findings

Apple's XCFramework documentation requires alternate build systems to create a
static library per platform/destination, keep iOS and Simulator binaries
separate, and pass each library plus headers to
`xcodebuild -create-xcframework`. It explicitly warns against combining device
and Simulator slices with `lipo`. The implementation follows that shape.

Apple PackageDescription exposes a local `binaryTarget(name:path:)` only on
Apple platforms. A source target is therefore required to carry UniFFI's
generated Swift API above the C/static binary target. No remote URL or archive
checksum becomes a contract in this slice.

UniFFI 0.32 provides `uniffi_bindgen_swift()` and documents library-mode
generation with `--xcframework`, `--modulemap` and
`--modulemap-filename module.modulemap`. The module guide requires the
XCFramework module map to use that conventional filename. A dedicated binary in
the already isolated bindgen tool is preferable to hand-editing generated
module maps.

### Compatibility risks and decisions

### Xcode 26 module-map report

`mozilla/uniffi-rs#2917` reports that the generated `use "Darwin"`,
`use "_Builtin_stdbool"` and `use "_Builtin_stdint"` declarations can make a
SwiftPM binary target fail under Xcode 26/27. The issue is open and its only
maintainer response reports successful Xcode 26.5 use, so it is evidence of a
configuration-sensitive risk rather than proof that 0.32 is unusable.

The pre-code experiment built the exact local binary-target package under Xcode
26.4. `canImport(IdentusDidFFI)` evaluated false with UniFFI's generated
`framework module` map, leaving every generated FFI type unresolved. Removing
only the three `use` lines was insufficient. Changing that declaration to a
plain `module` and removing the three lines made the same package compile, link
and pass on the iOS 17.5 arm64 Simulator without compiler flags.

Decision: adopt a narrow fail-closed normalizer after UniFFI generation. It
accepts only the exact expected 0.32.0 module-map lines and emits a plain module
containing the generated header plus `export *`; any upstream drift stops the
gate. The header and Swift source stay unmodified and generated. Do not use
unchecked or unsafe Swift compiler flags.

### Multiple Rust static libraries

`mozilla/uniffi-rs#1710` documents duplicate Rust runtime symbols when an app
links multiple separately packaged Rust static libraries. This proof contains
one Rust static library, so it cannot establish safe composition of multiple
leaf XCFrameworks.

Decision: the evidence package is local and singular. A future public native
SDK should aggregate binding modules into one Rust static-library package or
prove another composition model before support activation. This limitation is
recorded in the ADR and support evidence.

### Determinism

XCFramework content comparison ignores filesystem timestamps by comparing file
paths and bytes. The build runs twice from the same source and lock, normalizes
the XML property list before hashing, compares every generated source/header,
and compares the static archives byte-for-byte. Absolute build paths must not
appear in package contents.

### Simulator scope

An Xcode-driven test on one available arm64 Simulator proves selection, import,
link, load and behavior for that environment. It does not prove physical
devices, older OS execution, release signing, installation or downstream app
integration.

### Dependency, license and unsafe impact

Adding `staticlib` changes artifact output only. The root runtime graph and the
separately locked generator graph add no packages. UniFFI remains the only new
FFI framework and its exact MPL-2.0 exceptions remain package-scoped. The SDK
continues to forbid authored unsafe Rust. Generated C/Swift and dependency-owned
native/unsafe reach are inspected by the package gate and exact-diff review.

## Rejected or deferred candidates

Hand-written bridge artifacts and `lipo` composition are rejected because they
violate the single generated contract or Apple's platform model. Multiple Rust
static XCFramework composition and remote package publication are deferred with
explicit triggers in the candidate table. Intel Simulator, Catalyst, macOS,
watchOS, tvOS, visionOS, physical devices, signing, distribution, Android and
React Native are outside this bounded proof and remain separate decisions.

## Open questions and blockers

No research blocker remains. Exact Xcode 26.4 module-map behavior has been
reproduced and the bounded static-module adapter has passed the same package
test without compiler flags. A future public native SDK still needs
an aggregate-library/composition decision, supported platform matrix,
physical-device evidence, signing and release ownership.

## Evidence commands

- `rustc --version`, `rustup target list --installed`, `xcodebuild -version`,
  `swift --version`, `clang --version` and `xcodebuild -showsdks` recorded the
  exact local tool and target inputs.
- `xcrun simctl list devices available` inventoried compatible arm64 Simulator
  runtimes without changing them.
- Repository inspection verified the existing leaf crate types, exact UniFFI
  lock, bindgen entry point, host script, generated header/module-map shape and
  weekly macOS workflow.
- Apple and UniFFI primary documentation plus upstream issues #2917 and #1710
  were retrieved on 2026-09-09.
- Exact `xcodebuild test` experiments proved the generated map fails, removing
  only builtin `use` lines still fails, and a plain static-library module with
  those lines removed compiles, links and executes on iOS Simulator 17.5.
- `scripts/factory research-ready`, constraint readiness, full factory
  validation, focused Apple construction/execution, Cargo/Nix checks and exact-
  diff review remain mandatory. Implementation-dependent commands are not
  claimed at this pre-code checkpoint.

Exact commands already run are the version, target, SDK, Simulator and
repository-inspection commands listed above. Unrun checks are XCFramework double
construction, archive/manifest/symbol/path inspection, SwiftPM build and iOS
Simulator execution, full Cargo/Nix gates, exact-diff review and hosted CI;
they depend on implementation and remain mandatory.

### Rollback and reconsideration

Rollback removes `staticlib`, the Swift generator entry point, package fixture,
Apple script and slow-lane step. No published artifact, consumer lock, persisted
data or migration exists. Reconsider support only after physical-device,
distribution-signing, public artifact reproducibility and downstream adoption
evidence exists together with the Android side required by #222.
