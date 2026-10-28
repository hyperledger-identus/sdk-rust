# sd-jwt-engine-selection Specification

## Purpose
TBD - created by archiving change qualify-sd-jwt-engine. Update Purpose after archive.
## Requirements
### Requirement: Stable base and evolving profile are independently pinned

The SDK SHALL treat RFC 9901 base SD-JWT and SD-JWT VC as separate capabilities.
Each SHALL record an immutable normative version, content hash, supported
serialization, exact media/protected type, migration rule, and vector set.
A profile revision SHALL NOT silently change base behavior or an existing
profile identifier.

#### Scenario: SD-JWT VC advances while RFC 9901 remains stable

- **WHEN** a newer SD-JWT VC draft or RFC is assessed
- **THEN** the profile receives an explicit versioned migration and differential evidence while the RFC 9901 base contract remains unchanged unless its own errata requires a reviewed change

### Requirement: Dependency adoption is private and all-gates

A candidate engine SHALL be adopted only when one immutable release passes the
SDK's normative parity, valid and invalid behavior, Rust 1.89.0 MSRV, Rust
1.98.1 primary, license/provenance, advisory, direct/resolved cone,
unsafe/native, host/portable-target, resource-bound, diagnostic-redaction,
secret-handling, maintenance, and concrete-consumer-payoff gates. Candidate
types and policies SHALL remain private behind Identus-owned staged states.

#### Scenario: Candidate implements RFC 9901 but exceeds the release MSRV

- **WHEN** a current candidate requires Rust newer than 1.89.0 for the 0.1.x line
- **THEN** it remains an oracle or deferred candidate and no production dependency or public candidate type is added

### Requirement: Local implementation remains focused and replaceable

When no candidate passes every adoption gate, the SDK SHALL implement only the
bounded deterministic RFC 9901 mechanics needed by named issuance and
presentation consumers. The focused implementation SHALL live in the
independently selectable `identus-sd-jwt` crate and expose only SDK-owned
staged values. Signing, verification, entropy, time, key resolution, trust,
custody, replay, transport, persistence, and consent SHALL remain behind
existing focused Identus capabilities or caller-owned policy. Candidate types
and production dependencies SHALL remain outside the public and resolved
production graph.

#### Scenario: External frameworks fail cohesion or bounds

- **WHEN** no released engine clears the complete adoption matrix
- **THEN** the implementation issue builds an Identus-owned focused engine and retains external implementations only as independent differential oracles

#### Scenario: private mechanics are later replaced

- **WHEN** a future immutable engine passes every adoption and equivalence gate
- **THEN** it may replace internal mechanics without changing the staged Identus public contract or silently changing RFC 9901 behavior

### Requirement: Canonical and legacy SD-JWT VC identifiers do not mix

Canonical SD-JWT VC issuance and verification SHALL use `dc+sd-jwt` under the
pinned profile. `vc+sd-jwt` SHALL NOT be canonical output and MAY be accepted
only by a separately versioned, opt-in compatibility adapter with a named
consumer, bounded lifetime, provenance, unambiguous result, and removal trigger.

#### Scenario: Legacy SDK-TS credential is presented

- **WHEN** a named migration adapter receives an otherwise valid legacy `vc+sd-jwt` credential
- **THEN** the adapter reports legacy provenance explicitly and the canonical engine neither emits the legacy identifier nor treats both identifiers as interchangeable policy

### Requirement: Evidence covers hostile behavior and a complete offline journey

Implementation SHALL use versioned provenance-bound RFC 9901 and SD-JWT VC
vectors plus clean-room negative cases. The RFC 9901 base quality declaration
SHALL cover property tests, compact/disclosure/claim-graph/key-binding fuzzing,
bounded worst-case benchmarks, at least two differential oracles,
cross-language migration fixtures, and an offline
issue-store-select-present-key-bind-verify journey through public SDK APIs.
Official examples SHALL be distinguished from SDK-authored and donor evidence,
and the evolving SD-JWT VC profile SHALL remain a separate declaration.

#### Scenario: Valid round trip passes but rejection behavior differs

- **WHEN** a candidate or local engine accepts duplicate digests, unused disclosures, reserved claims, malformed encodings, mismatched key binding, or over-limit work that the pinned contract rejects
- **THEN** conformance fails even if valid issuance and presentation round trips succeed

#### Scenario: the base offline journey passes without a VC profile

- **WHEN** RFC 9901 issuance, envelope storage, claim selection, key binding, and verification complete through public SDK seams
- **THEN** the base quality declaration may become passing without claiming SD-JWT VC, OID4VC, HTTP, trust, publication, or downstream adoption completion

### Requirement: Compile evidence remains narrower than runtime support

The assessment and implementation SHALL record host, WASM/browser, iOS, and
Android evidence exactly. Compile-only success SHALL NOT be represented as
runtime, packaging, FFI, hardware-backed-key, certification, or device support.

#### Scenario: Portable target compiles without a runtime harness

- **WHEN** the crate compiles for a portable target but no browser or device journey executes
- **THEN** the target is reported as compile-only and the existing runtime-support limitation remains effective
