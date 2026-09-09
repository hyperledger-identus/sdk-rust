# Aries Askar storage spike

## Purpose

Define the isolated evidence required before Aries Askar can be proposed as an
optional implementation of SDK-owned wallet storage ports.

## ADDED Requirements

### Requirement: research dependencies remain isolated

The exact published candidate and SQLite backend SHALL exist only in a
separately locked research fixture. No Askar, SQLite, runtime, crypto, FFI or
candidate type SHALL enter a root manifest, release artifact or public Identus
API.

#### Scenario: fixture is inspected

- **WHEN** the root and research dependency graphs are compared
- **THEN** the root graph is unchanged and all candidate packages are confined
  to the explicit fixture lock

### Requirement: exact-record semantics use SDK authority

The spike SHALL adapt one `SecretStore` exact-record surface and run the shared
conformance suite. Scope/key encoding SHALL be collision-free. Revisions SHALL
be SDK-owned, non-empty, bounded and invalidated after replacement. Conditional
replacement/deletion SHALL validate the expected revision atomically inside a
candidate transaction.

#### Scenario: shared conformance executes

- **WHEN** insert, load, replace, delete, conflict and isolation behaviors run
  against a fresh encrypted in-memory SQLite store
- **THEN** the adapter satisfies the exact-store report without exporting any
  candidate model or diagnostic

#### Scenario: stale revision is presented

- **WHEN** a caller attempts replacement or deletion with an invalidated
  revision
- **THEN** the adapter returns the closed SDK conflict class and preserves the
  current record

### Requirement: security and target evidence is honest

The spike SHALL inventory pass-key ownership, transaction cancellation,
candidate diagnostics, unsafe/native reach, build scripts, native links,
licenses, advisories, MSRV and target compile/link results. It SHALL distinguish
ephemeral behavior from file persistence, crash recovery, mobile runtime,
custody, support and certification.

#### Scenario: final disposition is recorded

- **WHEN** the executable and supply-chain evidence is complete
- **THEN** an ADR selects `conditional-adopt`, `oracle/reference` or
  `not-adopt`, lists exact limitations and reconsideration triggers, and sends
  any production adapter to a separate bounded issue
