# Separate peer and PRISM DID method ownership

## Why

A2 currently groups peer DID and portable PRISM semantics without deciding
profiles, crate boundaries, or how NeoPRISM can become smaller. SDK-TS supplies
current consumer behavior but is not normative. NeoPRISM supplies mature Rust
PRISM behavior but mixes portable method/state logic with Cardano, persistence,
and node effects. Implementing either donor shape directly would violate the
accepted platform-core and reusable-module rules.

## What changes

- Keep `identus-did` method-neutral.
- Specify `identus-did-peer` with explicit current numalgo-4 and opt-in
  numalgo-2 compatibility profiles.
- Use available Rust peer DID crates as differential oracles rather than
  importing their DID/document/secret ecosystems.
- Supersede ADR 0110's initial PRISM disposition: SDK-Rust owns portable PRISM
  CRUD/state, VDR semantics, and deterministic indexing in separate crates.
- Keep concrete Cardano clients, SQL, HTTP, CLI, deployment, and node
  orchestration in NeoPRISM initially.
- Create ordered child issues #521, #522, and #523, record the discovered
  generic long-form resource prerequisite in #524, and update A2/IDR-040.

## Capabilities

### Added capabilities

- `peer-did-method`: bounded versioned peer DID creation and resolution.
- `prism-did-method`: portable PRISM method CRUD and state reduction.
- `prism-vdr-indexing`: portable PRISM VDR rules and deterministic indexing
  behind effect ports.

### Modified capabilities

- `identus-platform-core-migration`: A2 owns the reusable PRISM foundation and
  a measured NeoPRISM reduction path, not only a portable codec decision.
- `ssi-upstream-program`: IDR-040 is specified by #493 and its children.

## Non-goals

No production Rust, dependency, target support, crate publication, SDK-TS or
NeoPRISM mutation, Cardano client, database, service, or deprecation lands in
this planning change.

## Delivery

Issue #493 owns this planning packet and ADR 0174. #524 must settle the bounded
generic DID/PRISM long-form policy before #521. #523 can implement
independently after #5/A1 evidence. #522 additionally depends on #521. SDK-TS
and NeoPRISM adoption use later immutable pins and cannot authorize SDK API
design or premature deletion.
