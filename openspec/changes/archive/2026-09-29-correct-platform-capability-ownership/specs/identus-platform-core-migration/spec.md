# identus-platform-core-migration Delta Specification

## MODIFIED Requirements

### Requirement: SDK-TS inventory is implementation-ready evidence

The SDK-TS inventory SHALL pin the release and revision and record every
capability's source paths, package/public/wire/persistence/operational surface,
profile and dependencies, test authority, consumers, compatibility risk,
target and platform owners, disposition, security and resource concerns,
migration phase, rollback, and evidence issue. Missing implementation evidence
SHALL result in `defer` and a bounded child issue. SDK-Rust SHALL define the
canonical reusable DTO and error contracts; language facades MAY preserve
idiomatic or legacy shapes through an explicit, versioned, tested, and
deprecatable adapter that does not constrain the Rust core.

#### Scenario: a feature exists but its contract is ambiguous

- **WHEN** source code or a unit test demonstrates behavior without normative,
  reviewed Identus, or named consumer authority
- **THEN** the inventory classifies it as implementation evidence and does not
  port it until a child issue resolves the contract

#### Scenario: a first canary is selected

- **WHEN** the inventory confirms bounded DID and DID URL values as the first
  reversible SDK-TS Rust-backed path
- **THEN** a separate canary issue defines an opt-in facade, shared vectors,
  explicit legacy-to-Rust DTO/error translation, browser/Node/package evidence,
  consumer rehearsal, and rollback before any SDK-TS mutation

## ADDED Requirements

### Requirement: Portable private-workspace behavior converges in Rust

Portable domain, protocol, format, and cryptographic behavior SHALL move from
a language SDK's private workspace to SDK-Rust or a qualified Rust
dependency behind an Identus-owned facade. Build configuration, package
loading, and host integration MAY remain language-specific. Protobuf used by a
DID method SHALL be treated as codec evidence, not as authority for unrelated
package ownership.

#### Scenario: a private WASM engine exists

- **WHEN** SDK-TS bundles AnonCreds, DIDComm, JWE, or another portable engine
  through a private workspace package
- **THEN** the inventory assigns the portable capability to Rust and requires
  exact engine/dependency qualification rather than retaining duplicate
  TypeScript ownership

### Requirement: DID methods and services compose the Rust DID core

SDK-Rust SHALL own generic DID values, documents, verification relationships,
services, resolution, dereferencing, registration, and method dispatch.
Portable method codecs and deterministic state, including supported
peer DID numalgos, SHALL compose those contracts. Concrete ledger, network,
custody, persistence, and product policy SHALL remain injected or downstream.

#### Scenario: a DID method needs chain infrastructure

- **WHEN** Prism or another DID method requires ledger observation or
  transaction submission
- **THEN** its portable codec and method semantics may live in a focused Rust
  module while the concrete chain adapter remains outside the generic core

### Requirement: DIDComm engine and application protocols are separately versioned

DIDComm Messaging v2.1 pack/unpack/routing behavior SHALL be a core Rust
capability. Every application protocol SHALL have a separate capability record
pinning its specification revision, status, PIURI, roles, message types, state
schema, effects, fixtures, consumers, migration, and deprecation.

#### Scenario: an issuance workflow is added

- **WHEN** SDK-Rust adds issue-credential behavior
- **THEN** the issue-credential protocol version and state machine are reviewed
  independently from the DIDComm engine and from present-proof, mediation,
  pickup, out-of-band, revocation notification, or basic-message support

### Requirement: The portable agent runtime is Rust-owned and effect-driven

SDK-Rust SHALL target a small executor-neutral runtime that deterministically
dispatches bounded versioned events through protocol state machines and emits
typed effects. Scheduling implementation, network/TLS, storage engines,
custody, user consent, telemetry sinks, and deployment lifecycle SHALL enter
through explicit host adapters.

#### Scenario: the same protocol runs in browser and native hosts

- **WHEN** a protocol transition requires resolution, transport, persistence,
  a timer, or user input
- **THEN** it emits a typed effect with correlation, cancellation, resource,
  and checkpoint semantics that either host can execute without reimplementing
  the protocol state machine

### Requirement: Networking separates protocol semantics from host execution

Rust protocol crates SHALL own bounded request/response values, validation,
correlation, timeout/cancellation intent, and transport ports. Browser, Node,
native, mobile, and server adapters SHALL own host I/O policy. An adapter MAY
be implemented in Rust/WASM or as thin language code, but SHALL NOT duplicate
protocol validation, secret handling, or state transitions.

#### Scenario: a browser uses fetch

- **WHEN** a browser adapter executes a protocol request through `fetch`
- **THEN** TypeScript or Rust/WASM performs only the declared host effect and
  returns a bounded response to the same canonical Rust protocol contract

### Requirement: Service replacement begins with immutable interoperability evidence

Cloud Agent and Mediator behavior SHALL first be exercised as immutable
black-box E2E interoperability evidence. A later Rust reference service SHALL
be composed from public SDK primitives and explicit adapters. Deprecation SHALL
require protocol, persistence/migration, tenancy, operations, performance,
consumer, and rollback parity and a separate decision.

#### Scenario: existing service behavior is needed for a Rust protocol

- **WHEN** an SDK-Rust DIDComm or credential protocol reaches E2E validation
- **THEN** tests pin the existing service version and expected wire/error
  behavior before a replacement service is treated as compatible

### Requirement: Risk-appropriate generative and differential quality evidence is mandatory

Every capability SHALL identify whether property, fuzz, benchmark, and
differential evidence is required and SHALL justify any not-applicable class.
Required evidence MAY run in focused, slow, or release lanes according to cost,
but its exact-head result or recorded debt SHALL remain visible. These methods
are engineering evidence and SHALL NOT become normative authority by
themselves.

#### Scenario: a protocol parser and state transition are implemented

- **WHEN** untrusted wire data drives a versioned transition
- **THEN** bounded examples are supplemented by applicable property invariants,
  fuzz targets, performance/resource baselines, and differential or official
  vector comparisons before production promotion
