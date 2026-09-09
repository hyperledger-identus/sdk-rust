# browser-did-bindings Specification

## Purpose

Define the isolated, bounded and reviewable browser WASM contract for generic
DID and DID URL value parsing without activating browser support or publication.

## Requirements

### Requirement: Browser DID bindings isolate the domain model

The SDK SHALL provide bounded DID and DID URL parsing through an isolated
browser WASM crate over `identus-did`. The crate SHALL expose only SDK-owned
JavaScript values and errors, and SHALL keep wasm-bindgen annotations, types and
dependencies out of generic domain crates.

#### Scenario: browser parses public identifier values

- **WHEN** initialized browser JavaScript parses a valid DID or DID URL
- **THEN** the returned owned class SHALL preserve the exact value and all
  applicable components without exposing a Rust/domain/dependency type

### Requirement: Browser failures remain bounded and redacted

The facade SHALL preserve the domain parser's 2,048-byte DID and 4,096-byte DID
URL ceilings. Invalid and oversized input SHALL throw a closed SDK-owned error
with a stable constant code and SHALL NOT expose caller text, parser details or
implementation diagnostics.

#### Scenario: invalid browser input is rejected

- **WHEN** a caller supplies invalid or oversized input containing a canary
- **THEN** Chromium and Firefox SHALL observe the matching constant error code
  and every exposed error property SHALL omit the canary

### Requirement: Browser package generation is exact and reviewable

The runtime and official test harness SHALL use exact aligned wasm-bindgen
versions, and wasm-pack SHALL be pinned by the repository Nix lock. Two release
`web` builds SHALL produce byte-identical complete output trees. A committed
TypeScript declaration snapshot SHALL cover the versioned functions, values,
getters, errors and lifecycle methods.

#### Scenario: browser package does not drift

- **WHEN** the package evidence script builds the crate twice
- **THEN** complete-tree and declaration comparisons SHALL pass before hashes,
  uncompressed byte measurements or browser tests are accepted

### Requirement: Two browser engines execute API version one

The package SHALL expose binding API version `1`. Headless Chromium and Firefox
SHALL execute matching valid, invalid, oversized, redaction, component and
version tests in the weekly/manual slow Ubuntu line. Ordinary Linux pull
requests SHALL retain the existing fast factory/build/lint/test line.

#### Scenario: slow browser evidence runs

- **WHEN** the scheduled or manually dispatched browser job runs
- **THEN** both engines SHALL pass the same behavior families or the job SHALL
  fail without converting one engine's evidence into the other's

### Requirement: Browser integration obligations are explicit

The generated package SHALL be browser-native ESM with an awaited initializer.
It SHALL require no Node, framework, direct DOM/browser API, worker, thread or
async callback. Documentation SHALL assign WASM asset delivery, CSP/CORS,
paired-resource caching and explicit freeing of retained WASM-owned values to
the consuming application.

#### Scenario: experimental adapter lands

- **WHEN** the adapter and two-engine evidence merge to `develop`
- **THEN** FFI/browser support SHALL remain `not-supported`, artifact size SHALL
  remain measurement-only and no publication, React Native, bundler-version or
  certification promise SHALL be introduced
