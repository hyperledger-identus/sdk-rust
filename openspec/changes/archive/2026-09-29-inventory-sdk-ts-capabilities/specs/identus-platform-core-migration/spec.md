# identus-platform-core-migration Specification

## ADDED Requirements

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
SHALL result in `defer` and a bounded child issue.

#### Scenario: a feature exists but its contract is ambiguous

- **WHEN** source code or a unit test demonstrates behavior without normative,
  reviewed Identus, or named consumer authority
- **THEN** the inventory classifies it as implementation evidence and does not
  port it until a child issue resolves the contract

#### Scenario: a first canary is selected

- **WHEN** the inventory confirms bounded DID and DID URL values as the first
  reversible SDK-TS Rust-backed path
- **THEN** a separate canary issue defines an opt-in facade, shared vectors,
  TypeScript DTO/errors, browser/Node/package evidence, consumer rehearsal, and
  rollback before any SDK-TS mutation
