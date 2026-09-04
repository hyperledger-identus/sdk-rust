# Credential status delta specification

## ADDED Requirements

### Requirement: Status method and purpose identifiers are open and bounded

The SDK SHALL provide distinct owned `CredentialStatusMethod` and
`CredentialStatusPurpose` values. Each SHALL accept non-empty UTF-8 of at most
256 bytes with no surrounding whitespace or control character, validate before
one successful allocation, preserve accepted spelling exactly, and avoid a
closed registry of methods or purposes.

#### Scenario: unrelated status profiles share the same identifiers

- **WHEN** adapters construct W3C `BitstringStatusListEntry`, W3C `revocation`
  and `suspension`, and Midnight same-contract or external non-membership names
- **THEN** each SHALL round-trip through the matching open identifier without a
  chain, format, protocol, or global-registry dependency

#### Scenario: unsafe identifier input fails closed

- **WHEN** a method or purpose is empty, oversized, padded with whitespace, or
  contains a control character
- **THEN** construction SHALL return a static typed error without retaining or
  rendering the rejected text

### Requirement: Status opaque values are bounded and role-specific

The SDK SHALL provide distinct reference, handle, revision, and observed-value
types, each accepting either exact validated text or exact non-empty bytes.
Reference values SHALL accept at most 2,048 bytes, handles at most 1,024 bytes,
and revisions and observed values at most 256 bytes. Text SHALL have no
surrounding whitespace or control character. Accepted data SHALL be exposed
only through explicit borrowed text/byte accessors.

#### Scenario: W3C and Midnight values preserve their source representation

- **WHEN** a W3C status-list URL/index and a Midnight binary registry
  reference/commitment/revision/state are constructed
- **THEN** each SHALL preserve exact text or bytes without coercion or a common
  wire encoding

#### Scenario: empty and oversized opaque values fail safely

- **WHEN** any opaque role receives empty, invalid text, or limit-plus-one data
- **THEN** it SHALL return the role's stable error without retaining or
  displaying caller data

### Requirement: Complete status bindings support bounded multiplicity

A `CredentialStatusBinding` SHALL contain exactly one method, purpose,
reference, and handle. `CredentialStatusBindings` SHALL contain 1–16 complete,
exactly unique bindings. Absence SHALL be represented by no collection rather
than a `None` binding variant. Construction SHALL check bounds before an
allocation-free pairwise duplicate scan and retain the caller-owned vector.

#### Scenario: one credential carries revocation and suspension entries

- **WHEN** two W3C-shaped bindings have distinct purposes and handles
- **THEN** both SHALL coexist and remain attributable in one collection

#### Scenario: incomplete, empty, oversized, and duplicate states are excluded

- **WHEN** callers attempt an absent field, an empty/oversized collection, or
  an exact duplicate binding
- **THEN** the public construction surface SHALL make incompleteness
  unrepresentable or return a stable typed error

### Requirement: Freshness and query requirements remain policy-neutral

`CredentialStatusFreshness` SHALL contain optional minimum opaque revision and
optional maximum `DurationMillis` age, with at least one criterion.
`CredentialStatusRequirements` SHALL contain optional method and purpose
allow-lists of 1–16 unique values plus optional freshness. `None` SHALL mean no
generic restriction; an empty present allow-list SHALL be rejected. A
`CredentialStatusQuery` SHALL own one complete binding and one requirements
value without choosing a clock, comparing revisions, retrieving status, or
deciding credential usability.

#### Scenario: two freshness strategies fit one query contract

- **WHEN** a Midnight adapter requests a minimum opaque ledger revision and a
  W3C adapter requests evidence no older than a duration
- **THEN** both SHALL construct complete queries without importing a ledger,
  clock, registry, network client, or method-specific comparator

#### Scenario: ambiguous requirement shapes fail at construction

- **WHEN** freshness has no criterion or a present allow-list is empty,
  oversized, or duplicated
- **THEN** construction SHALL reject the shape instead of assigning sentinel
  semantics

### Requirement: Status evidence is attributable and time ordered

`CredentialStatusEvidence` SHALL own the complete binding it describes, one
open observed value, optional opaque revision, and optional
`UnixTimestampMillis` observation and expiry times. When both timestamps exist,
observation SHALL be no later than expiry. Construction SHALL not imply
cryptographic proof, freshness, trust, or usability.

#### Scenario: status facts preserve method-specific meaning

- **WHEN** an adapter reports W3C revocation value `0` or a Midnight binary
  state at a revision
- **THEN** evidence SHALL preserve its binding, value, revision, and available
  timestamps without converting it to a closed lifecycle enum

#### Scenario: reversed evidence time fails closed

- **WHEN** observation is later than expiry
- **THEN** construction SHALL return a static time-range error

### Requirement: Status diagnostics and errors protect correlating values

Debug for opaque values and every aggregate SHALL NOT render reference, handle,
revision, or observed-value contents. Aggregate Debug MAY expose method,
purpose, variant, lengths, counts, and optional-field presence. Every status
construction error SHALL map to `IdentusError` with capability `credential`,
kind `InvalidInput`, a stable static `credential.*` code, and static text.

#### Scenario: correlating status data does not enter diagnostics

- **WHEN** known reference, handle, revision, and observed-value canaries are
  formatted directly, inside bindings/queries/evidence, or through every new
  error bridge
- **THEN** no canary SHALL appear

### Requirement: Status construction remains allocation-conscious

Text parsing SHALL validate borrowed input before one successful allocation.
Byte constructors SHALL retain transferred vectors. Collection constructors
SHALL check bounds before bounded pairwise comparisons and SHALL allocate no
temporary registry or set. A manual ignored release diagnostic SHALL report
representative construction throughput without a machine-dependent pass
threshold.

#### Scenario: performance path is measurable without flaky correctness

- **WHEN** the ignored release diagnostic constructs representative bindings,
  requirements, queries, and evidence through the public API
- **THEN** it SHALL report elapsed time and throughput while correctness tests
  remain independent of host timing

### Requirement: Status core remains wire and infrastructure free

The status module SHALL add no serde/wire model, arbitrary JSON payload,
registry/network/ledger dependency, clock, reader/writer/verifier port,
cryptographic proof validation, trust decision, persistence, consumer import,
or downstream modification.

#### Scenario: adapters retain their proper ownership

- **WHEN** a Midnight or W3C adapter needs wire decoding, registry lookup,
  revision comparison, proof verification, or a usability decision
- **THEN** it SHALL layer that behavior outside this structural core
