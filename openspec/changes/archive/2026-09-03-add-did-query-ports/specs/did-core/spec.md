## ADDED Requirements

### Requirement: Bounded extensible DID resolution options

The DID Core capability SHALL provide immutable `ResolutionOptions` containing
optional `accept`, `expandRelativeUrls`, `versionId` and `versionTime` values
plus method/extension members. Common members SHALL reuse their existing typed
validators. Absence and explicit false SHALL remain distinct, extensions SHALL
NOT shadow common keys, and native construction and serde SHALL enforce the
same bounded metadata policy.

#### Scenario: common and method options remain portable

- **WHEN** a PRISM- or Midnight-shaped resolver receives common options and a
  method extension
- **THEN** it SHALL observe the exact typed values and preserved extension JSON
  without importing chain, method, transport or runtime types

#### Scenario: malformed resolution options fail closed

- **WHEN** an option map contains a malformed media type, datetime or version,
  a reserved extension collision, excessive input or invalid open JSON
- **THEN** every public construction path SHALL reject it with the existing
  redaction-safe DID resolution error boundary

### Requirement: Bounded extensible DID URL dereferencing options

The capability SHALL provide immutable `DereferencingOptions` containing
optional `accept` and `verificationRelationship` values plus extension members.
Verification relationship names SHALL be non-empty bounded printable ASCII and
remain open to extension vocabularies. The option type SHALL be independent of
resolution options because W3C DID URL dereferencing is at risk.

#### Scenario: verification relationship remains open and safe

- **WHEN** a core or extension verification relationship is supplied
- **THEN** its exact valid ASCII spelling SHALL reach the dereferencer while
  empty, control-bearing, padded or oversized values are rejected

#### Scenario: at-risk dereferencing does not freeze the resolver

- **WHEN** the dereferencing contract evolves or an implementation supports
  resolution only
- **THEN** `ResolutionOptions` and `DidResolver` SHALL remain independently
  usable without implementing or depending on dereferencing

### Requirement: Object-safe runtime-neutral DID query ports

The capability SHALL define `DidResolver` and `DidUrlDereferencer` as
`Send + Sync`, object-safe `#[identus::port]` traits. Resolution SHALL accept a
borrowed validated `Did` and `ResolutionOptions` and asynchronously return a
`DidResolutionResult`. Dereferencing SHALL accept a borrowed validated `DidUrl`
and `DereferencingOptions` and asynchronously return a
`DidUrlDereferencingResult`. Named boxed `Send` future aliases SHALL make the
allocation and lifetime contract explicit without selecting an async runtime.

#### Scenario: independent method implementations use one injection seam

- **WHEN** PRISM- and Midnight-shaped mock implementations are stored and
  invoked through trait objects
- **THEN** both SHALL receive exact SDK inputs and return valid SDK result
  envelopes without method, chain, HTTP, executor or product dependencies

#### Scenario: standards failures use one result channel

- **WHEN** a resolver encounters unsupported options/methods, not-found or an
  internal implementation failure
- **THEN** it SHALL return the corresponding W3C error result rather than a
  second generic transport/error result channel

### Requirement: Uniform bounded option validation

Raw resolution and dereferencing option JSON SHALL be limited to 64 KiB before
deserialization. Extension maps SHALL contain at most 64 members with names no
longer than 256 bytes; arbitrary strings SHALL be at most 64 KiB; open values
SHALL be at most 32 levels and 4,096 aggregate nodes. Common scalar values SHALL
retain their smaller purpose-specific limits.

#### Scenario: native construction cannot bypass option limits

- **WHEN** the same excessive option member or extension tree is supplied to a
  builder/constructor or JSON entry point
- **THEN** both paths SHALL reject it under the same resource policy

#### Scenario: option and dynamic dispatch cost remains observable

- **WHEN** representative options are parsed and a trait-object resolver is
  invoked repeatedly in a release diagnostic
- **THEN** throughput SHALL be recorded as evidence without a machine-specific
  CI pass threshold
