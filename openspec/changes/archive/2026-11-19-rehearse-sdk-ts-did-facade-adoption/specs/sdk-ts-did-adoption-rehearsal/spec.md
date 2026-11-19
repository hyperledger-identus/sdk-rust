# sdk-ts-did-adoption-rehearsal Specification

## ADDED Requirements

### Requirement: The adoption journey is immutable and consumer-shaped

The rehearsal SHALL bind the exact SDK-Rust and SDK-TS revisions and SHALL
compose DID and DID URL values, DID Document projection, injected resolution
and dereferencing, selected peer DID creation/local resolution, and frozen
pairwise `from`/`to` values in one journey. It SHALL NOT reproduce DIDComm
packing, transport, custody, storage, mediation, or agent lifecycle.

#### Scenario: a pairwise relationship crosses the facade

- **WHEN** the fixture creates and resolves the selected peer DID and carries
  the resulting values through the request-shaped boundary
- **THEN** exact identifiers, document identity, relationship references,
  service projection, and injected resolver correlation remain stable

### Requirement: One assertion body exercises both provider routes

Every applicable assertion SHALL execute unchanged against the pinned legacy
compatibility provider and Rust-backed provider. A Rust-owned addition or
intentional deviation SHALL be named explicitly and SHALL NOT be presented as
legacy parity.

#### Scenario: the selected peer profiles differ

- **WHEN** the shared journey reaches numalgo 4
- **THEN** the Rust route proves the addition while the legacy route records the
  immutable not-applicable disposition without skipping a shared assertion

### Requirement: Provider selection and rollback are terminal

Browser and Node.js SHALL select either an injected compatibility provider or
the Rust-backed provider before generated initialization. A selected provider
failure SHALL be terminal and SHALL NOT invoke, initialize, or otherwise fall
through to the alternate provider.

#### Scenario: the Rust provider rejects a DID

- **WHEN** the selected Rust route returns a stable rejection
- **THEN** the journey retains that rejection, records no legacy invocation,
  and exercises rollback only through a new explicit pre-initialization choice

### Requirement: Applicable web targets retain exact receipts

Node ESM/CommonJS, real browser, React SSR, and hydrated React client SHALL run
the same journey at one exact SDK-Rust head. The receipt SHALL bind the donor
revision, source digests, assertion counts, routes, and target results and SHALL
fail on missing, duplicate, skipped, focused, or quarantined cases. React
Native SHALL remain structurally not applicable.

#### Scenario: a target omits one provider route

- **WHEN** an applicable target receipt lacks either required route or a shared
  assertion result
- **THEN** the rehearsal fails and consumer-proof cannot be recorded

### Requirement: Evidence does not activate the external product route

The rehearsal SHALL remain private, unpublished, opt-in, and non-default. It
SHALL NOT claim external SDK-TS mutation, support, accepted budgets,
publication, stable API, default activation, DIDComm runtime, credential
workflow, React Native, or mdoc delivery.

#### Scenario: every local target passes

- **WHEN** all repository-local receipts are complete and valid
- **THEN** local consumer-proof and rollback evidence may close while the
  capability remains partial until separate product and support gates pass
