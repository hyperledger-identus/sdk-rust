## ADDED Requirements

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
version checks against the built dynamic library.

#### Scenario: host language consumes API version one

- **WHEN** the Swift and Kotlin/JVM smoke programs load the generated binding
- **THEN** each SHALL observe API version `1` and pass the same success and
  failure behavior families

### Requirement: Host proof does not activate mobile or public FFI support

The component SHALL remain experimental and unpublished. Until #222 supplies
mobile package and runtime evidence, the SDK support policy SHALL keep
`SDK-LIM-002` effective and SHALL NOT claim iOS, Android, React Native, browser,
Node, store, release or certification support from host execution.

#### Scenario: experimental foundation is merged

- **WHEN** the crate and host evidence land on `develop`
- **THEN** the inventory SHALL identify the implemented experimental component
  while the machine-readable FFI status remains `not-supported`
