# ADR 0172: version DIDComm engine and application protocols separately

- **Status:** Accepted as program architecture
- **Date:** 2026-09-30
- **Decision authority:** project-sponsor review recorded in issue
  [#497](https://github.com/hyperledger-identus/sdk-rust/issues/497)
- **Related:** ADR 0061, ADR 0164, ADR 0169, ADR 0171; issues #419, #420,
  #491, #500, and #501
- **Constraint impact:** material; selects DIDComm Messaging v2.1 as the core
  target while leaving engine and application dependency choices open

## Context

SDK-TS packages message packing, routing, secret resolution, mediation,
pickup, issue-credential, presentation, out-of-band, basic message, and agent
tasks together. Those responsibilities do not share one version or maturity
level. DIF lists DIDComm Messaging v2.1 as ratified, while application
protocols have independent PIURIs, revisions, publishers, roles, and status.
For example, Present Proof 3.0 is currently draft and Coordinate Mediation 3.0
has its own state and transport requirements.

Treating “DIDComm support” as one feature would make conformance and compatibility
claims meaningless and would couple every protocol release to the engine.

## Decision

DIDComm Messaging v2.1 pack, unpack, routing, attachment, `from_prior`, DID
resolution, secret-resolution, resource, and error behavior is one core Rust
capability. Its engine may be built or provided by a qualified maintained Rust
crate behind an Identus-owned facade under issue #491.

Application protocols are independently versioned Rust modules above the core.
A machine-readable catalog records, for every supported protocol:

- exact specification revision, status, publisher, and PIURI family;
- supported roles, message types, attachments/formats, and problem reports;
- state-schema version, deterministic transitions, and typed effects;
- security, privacy, resource, storage, timer, and replay constraints;
- official, Identus, differential, and service-interoperability fixtures;
- target and binding evidence, known consumers, compatibility/deprecation, and
  owning implementation issue.

The initial catalog assesses basic message/problem report, discover features,
out-of-band, issue credential, present proof, revocation notification,
coordinate mediation/routing, and message pickup. Catalog inclusion is not an
implementation or support claim. Each protocol is delivered by a bounded
issue and may support only named roles or profiles initially.

Application protocols emit typed effects to ADR 0171's runtime. They do not
perform ambient I/O, choose user consent, hold raw keys, or own deployment
policy. Existing Cloud Agent and Mediator versions are phase-one black-box E2E
oracles, not architecture dependencies.

## Consequences

- Engine security review and application workflow evolution are decoupled.
- Support claims can name exact protocol versions and roles.
- SDK-TS and existing services remain valuable compatibility evidence without
  defining the Rust module layout.
- More catalog and migration metadata is required, but it prevents accidental
  draft or cross-version parity claims.

## Alternatives rejected

- One `identus-messaging` feature flag for all behavior: hides incompatible
  versions and creates a high-coupling release unit.
- Copy the SDK-TS plugin/module boundary: imports donor tasks and framework
  abstractions.
- Implement application protocols before selecting the engine boundary: makes
  message/attachment/error assumptions implicit.
- Wait for every protocol to stabilize: blocks useful independently versioned
  capabilities and interoperability work.

## Verification and rollback

Issue #500 owns the catalog and offline validator. Issue #491 owns the core
engine boundary. Issue #419 supplies Mediator evidence and issue #420 governs
fixture authority. Protocol support is reported only when its own catalog row,
implementation, required quality evidence, target proof, migration, and E2E
gate are complete.

Because no engine or protocol is activated here, rollback is documentary. A
later implemented protocol can be disabled or versioned independently without
removing the DIDComm v2.1 core.
