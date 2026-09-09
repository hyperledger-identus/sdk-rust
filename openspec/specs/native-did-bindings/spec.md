# native-did-bindings Specification

## Purpose
TBD - created by archiving change add-uniffi-did-host-bindings. Update Purpose after archive.
## Requirements
### Requirement: Native DID bindings isolate the domain model from UniFFI

The SDK SHALL provide DID and DID URL parsing through an isolated native
binding crate. The crate SHALL depend on `identus-did`, SHALL expose only
SDK-owned FFI values, and SHALL keep UniFFI annotations, types and dependencies
out of generic domain crates.

#### Scenario: DID value crosses the ABI

- **WHEN** a caller parses a valid DID through the native facade
- **THEN** the returned owned record SHALL preserve its exact value, method and
  method-specific identifier without exposing a Rust/domain/dependency type

#### Scenario: DID URL value crosses the ABI

- **WHEN** a caller parses a valid DID URL through the native facade
- **THEN** the returned owned record SHALL preserve its DID, path, query,
  fragment and exact round-trip representation

### Requirement: Native DID failures are bounded and redacted

The facade SHALL preserve the domain parser's 2,048-byte DID and 4,096-byte DID
URL bounds. Invalid, oversized and unexpected-panic failures SHALL return closed
error cases with stable SDK-owned codes and SHALL NOT return caller text, domain
details, panic payloads or implementation diagnostics.

#### Scenario: invalid input is rejected

- **WHEN** a Swift or Kotlin caller supplies an invalid or oversized DID or DID
  URL containing a canary value
- **THEN** the facade SHALL return the matching closed case and constant code,
  and every returned error representation SHALL omit the canary

#### Scenario: authored wrapper logic unexpectedly unwinds

- **WHEN** an internal wrapper operation panics before producing a value
- **THEN** the authored boundary SHALL catch the unwind and return only the
  constant internal error case/code without exposing the panic payload

### Requirement: Native binding generation is exact and reproducible

The runtime and generator SHALL use exact matching UniFFI versions. Generator
CLI dependencies SHALL remain outside the runtime workspace dependency cone.
Two generations from the same release library and lock SHALL produce identical
complete trees, and normalized public API snapshots SHALL cover every exported
function, record property, error case/code and ABI-version value.

#### Scenario: reviewed API does not drift

- **WHEN** the host verification script generates Swift and Kotlin bindings
  twice
- **THEN** complete-tree and normalized API comparisons SHALL pass or fail the
  gate before host consumer execution

### Requirement: Host consumers execute the versioned value slice

The facade SHALL expose binding API version `1`. Swift and Kotlin/JVM host tests
SHALL compile, link and execute valid, invalid, oversized, redacted-error and
version checks against the built dynamic library. The host execution and
separately locked generator supply-chain checks SHALL run in the existing
weekly slow macOS lane, not the Linux-only fast pull-request lane.

#### Scenario: host language consumes API version one

- **WHEN** the Swift and Kotlin/JVM smoke programs load the generated binding
- **THEN** each SHALL observe API version `1` and pass the same success and
  failure behavior families

#### Scenario: fast and slow CI remain deliberately separated

- **WHEN** ordinary pull-request CI runs during the active-development phase
- **THEN** the Linux fast line SHALL retain factory, build, lint and test gates,
  while macOS host execution and generator deny/audit evidence run weekly or by
  manual dispatch in the slow workflow

### Requirement: Host proof does not activate mobile or public FFI support

The component SHALL remain experimental and unpublished. Until #222 supplies
mobile package and runtime evidence, the SDK support policy SHALL keep
`SDK-LIM-002` effective and SHALL NOT claim iOS, Android, React Native, browser,
Node, store, release or certification support from host execution.

#### Scenario: experimental foundation is merged

- **WHEN** the crate and host evidence land on `develop`
- **THEN** the inventory SHALL identify the implemented experimental component
  while the machine-readable FFI status remains `not-supported`

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

### Requirement: Android AAR carries one explicit SDK ABI

The experimental Android package SHALL build `identus-uniffi-did` from the
same source, Cargo lock and Rust 1.98.1 toolchain for
`aarch64-linux-android`, using exact NDK 27.0.12077973 and minimum Android API
21. The SDK AAR SHALL contain the generated Kotlin API and exactly one SDK
native library at `jni/arm64-v8a/libidentus_uniffi_did.so`; it SHALL NOT
contain another SDK ABI or a copied/shadowed JNA implementation.

#### Scenario: packaged ABI is inspected

- **WHEN** the package gate opens the AAR
- **THEN** its native payload SHALL be an API-21 AArch64 ELF with the required
  ABI-version, checksum, DID and DID URL symbols and reviewed runtime needs

### Requirement: JNA remains an explicit exact dependency

Android consumers SHALL resolve exact `net.java.dev.jna:jna:5.18.1@aar`
separately from the local SDK AAR. The gate SHALL lock and inventory that
dependency and SHALL execute a consumer using only the SDK AAR plus its
declared exact runtime dependency.

#### Scenario: local consumer assembles

- **WHEN** the consumer application depends on the local SDK AAR
- **THEN** adding the exact JNA AAR dependency SHALL package both native
  libraries without requiring caller-owned JNI code or copied resources

### Requirement: Android construction is deterministic and ephemeral

Two independent builds SHALL produce byte-identical Rust libraries, generated
Kotlin and normalized complete AAR trees. The gate SHALL reject unexpected
archive paths, absolute checkout paths, ABI/API/ELF drift and missing symbols;
it SHALL record tool versions, dependency closure and artifact sizes. Generated
outputs, Gradle state and AVD state SHALL remain under ignored `target/` paths.

#### Scenario: package generation is repeated

- **WHEN** two independent package trees are constructed from the same locks
- **THEN** normalized content comparison SHALL pass or identify the exact
  differing path before runtime execution

### Requirement: Emulator execution does not activate Android support

An isolated arm64 API-35 emulator consumer SHALL observe binding API version
`1` and the existing valid, invalid, oversized and redacted-error behavior
families. The evidence SHALL run only in the weekly/manual slow macOS lane and
SHALL NOT mutate a user AVD. `SDK-LIM-002` and `SDK-LIM-003` SHALL remain
effective.

#### Scenario: one ephemeral emulator passes

- **WHEN** the generated local AAR executes on the selected emulator
- **THEN** the receipt SHALL name its SDK/image/API/ABI inputs while making no
  physical-device, other-ABI, publication, compatibility-matrix or support
  claim
