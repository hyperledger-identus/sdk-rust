# identus-platform-core-migration Specification

## Purpose
TBD - created by archiving change establish-identus-platform-core-program. Update Purpose after archive.
## Requirements
### Requirement: Reusable Identus semantics have one authoritative core

The program SHALL designate `sdk-rust` as the authoritative owner of reusable
Identus domain and protocol semantics. TypeScript, Swift, Kotlin, and React
Native surfaces SHALL compose that core through isolated adapters and retain
only reviewed language ergonomics, packaging, platform integration, product
policy, and time-bounded compatibility behavior.

#### Scenario: behavior is reusable across platforms

- **WHEN** a capability has equivalent domain or protocol semantics in more
  than one Identus SDK
- **THEN** its target owner is an appropriate generic `identus-*` Rust crate
  unless recorded evidence selects an upstream dependency or a justified
  platform-specific owner

#### Scenario: repository deletion is not assumed

- **WHEN** a language SDK still owns supported packaging or platform behavior
- **THEN** it remains an idiomatic facade or platform shell even after its
  duplicate reusable implementation is removed

### Requirement: Every capability has an explicit disposition and evidence

Before a capability migration begins, the registry SHALL record its stable
identifier, legacy owner and revision, target owner, disposition, specification
basis, public/wire/persistence surface, test authority, binding target,
compatibility class, deprecation phase, legacy-bug policy, dependencies,
security notes, issue, and evidence links. Allowed dispositions SHALL include
`move-to-rust`, `retain-platform`, `replace-upstream`, `deprecate`, `drop`, and
`defer`.

#### Scenario: unsupported or duplicate behavior is found

- **WHEN** inventory finds a feature with no continuing product, standard, or
  consumer justification
- **THEN** the program records `deprecate` or `drop`, a replacement if one
  exists, and phased exit criteria instead of porting it automatically

#### Scenario: evidence is incomplete

- **WHEN** current consumers, normative behavior, or target support cannot be
  established
- **THEN** the capability remains `defer` and cannot enter implementation

### Requirement: Test evidence has declared authority

The program SHALL classify relevant tests as `normative`, `identus-contract`,
`consumer-regression`, `implementation-regression`, or `exploratory`. Only the
first three SHALL constrain cross-SDK compatibility without an explicit
promotion decision. Shared vectors SHALL include provenance, immutable inputs,
expected outputs or errors, version/profile scope, and owning capability.

#### Scenario: legacy tests disagree

- **WHEN** two SDK tests encode conflicting behavior
- **THEN** the program resolves the conflict using normative sources, reviewed
  Identus profile decisions, and consumer evidence rather than majority vote

#### Scenario: implementation accident has useful coverage

- **WHEN** a local regression test has no public or normative basis
- **THEN** it may guide implementation quality but does not force compatibility
  until explicitly promoted

### Requirement: Consumer-visible changes drive migration and release evidence

The migration program SHALL record every consumer-visible additive, fixed,
behavioral, deprecated, breaking, or security change in a machine-readable
ledger before merge. The record SHALL identify affected capabilities/packages,
old and new behavior, replacement, compatibility window, migration action,
first/last version, release-note class, and evidence. Migration guides and
release notes SHALL be generated or reviewed from this ledger.

#### Scenario: a change is believed to be non-breaking

- **WHEN** a capability adds behavior or fixes an implementation defect without
  changing a supported contract
- **THEN** it is still recorded with evidence so downstream release notes and
  later compatibility analysis do not depend on memory

#### Scenario: a public contract changes

- **WHEN** names, behavior, errors, wire values, persistence, ABI, or runtime
  requirements change incompatibly
- **THEN** a migration action, compatibility window, and rollback are required
  before merge

### Requirement: Deprecation and legacy-bug compatibility are phased

Deprecation SHALL progress through explicit evidence-gated phases and SHALL NOT
remove behavior solely by date. A legacy bug SHALL default to
`fix-and-document`; it MAY be simulated only for a proven released-consumer
dependency through a bounded, versioned, testable, observable, owned, and
time-limited outer compatibility mode. Security or privacy defects SHALL fail
closed and SHALL NOT be simulated.

#### Scenario: replacement is not ready

- **WHEN** a deprecated capability lacks a supported replacement and consumer
  migration evidence
- **THEN** it cannot advance to `default-off` or `removed`

#### Scenario: a consumer requires historical buggy behavior

- **WHEN** an atomic migration is impractical and the behavior is safe to
  reproduce
- **THEN** a compatibility facade may simulate it with an explicit opt-in or
  version scope, owner, warning/detection path, tests, and removal release

#### Scenario: historical behavior weakens security or privacy

- **WHEN** reproducing a bug would expose secrets, accept invalid trust, weaken
  cryptography, bypass bounds, or corrupt persisted state
- **THEN** the migration fails closed and documents an intentional break

### Requirement: Language-SDK discovery uses an ordered evidence baseline

SDK-TS SHALL establish the initial normalized language-SDK capability catalog.
SDK-Swift SHALL follow as the near-current parity and deviation review, and
SDK-KMP SHALL follow both as the older Android/JVM compatibility review. Donor
recency SHALL NOT override accepted standards, current profiles, official
vectors, accepted Identus decisions, or stronger existing SDK-Rust behavior.

#### Scenario: an older SDK exposes a unique behavior

- **WHEN** SDK-KMP contains behavior absent from SDK-TS and SDK-Swift
- **THEN** the behavior remains discovery evidence until standards, current
  product use, consumers, and maintained implementation options justify a
  normalized capability and disposition

#### Scenario: SDK-TS is behind SDK-Rust

- **WHEN** SDK-Rust already implements a stronger current bounded contract
  than SDK-TS
- **THEN** the inventory records SDK-TS compatibility risk without regressing
  the Rust contract to donor behavior

### Requirement: SDK-Rust target names express responsibilities

SDK-Rust SHALL use responsibility-based crate, module, capability, and public
type names. Apollo, Castor, Pollux, Mercury, Pluto, and EdgeAgent SHALL NOT be
introduced as SDK-Rust target ownership boundaries. Those names MAY appear in
immutable provenance, historical reports, explicit legacy-import behavior, and
temporary language-SDK compatibility facades with an exit plan.

#### Scenario: a donor source path uses a historical module name

- **WHEN** inventory links source or tests below a historical module directory
- **THEN** the target capability uses a responsibility-based identifier and
  records the historical name only as a source alias

#### Scenario: a language SDK needs a transition alias

- **WHEN** a released consumer still imports a historical language-SDK symbol
- **THEN** the language facade may retain a deprecated compatibility alias,
  while SDK-Rust remains free of that target name and the change ledger records
  its migration and removal gates

### Requirement: Divergent format implementations normalize to current profiles

The program SHALL normalize divergent credential and protocol implementations
against current evidence. The target SHALL be selected from the exact current standard/profile, official
vectors, accepted Identus compatibility requirements, and maintained Rust
library evidence. Literal dependency or draft parity with a donor SHALL NOT be
an implementation goal.

#### Scenario: SD-JWT implementations differ

- **WHEN** SDK-TS, SDK-Swift, or SDK-KMP use different SD-JWT packages or draft
  behavior
- **THEN** the inventory separates RFC 9901 mechanics from the selected
  SD-JWT VC profile and requires a current Rust-library assessment before
  implementation

#### Scenario: AnonCreds support differs

- **WHEN** a language SDK lacks AnonCreds or embeds an older fork or wrapper
- **THEN** the target evaluates the current AnonCreds 1.0 specification and
  maintained Rust reference implementation, while VDR, secret, revocation,
  native, mobile, and WASM responsibilities remain explicit

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
