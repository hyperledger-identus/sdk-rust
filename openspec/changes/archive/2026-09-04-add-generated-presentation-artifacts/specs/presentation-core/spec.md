# Generated presentation artifact delta specification

## ADDED Requirements

### Requirement: Presentation artifacts are opaque, bounded, and selection-bound

The SDK SHALL provide a `PresentationArtifactBinding` containing one
presentation query ID and one opaque credential handle. A
`PresentationArtifact` SHALL contain one `CredentialFormat`, 1–64 unique
bindings, and non-empty opaque bytes of at most 4 MiB. Construction SHALL check
bounds and binding uniqueness before retaining transferred vectors. An
artifact MAY bind multiple disclosure selections only when their request
queries use the artifact's format.

#### Scenario: one artifact represents one or several selections

- **WHEN** an OID4VP adapter produces one presentation per selected credential
  and a VCDM- or Midnight-shaped adapter aggregates several same-format
  selections
- **THEN** both SHALL use the same opaque artifact API without importing a
  codec, wire container, proof type or chain dependency

#### Scenario: malformed artifact input fails closed

- **WHEN** bindings are empty, oversized or duplicated, or artifact bytes are
  empty or exceed 4 MiB
- **THEN** construction SHALL return the corresponding static typed error
  before invalid state escapes

### Requirement: Generated presentations exactly cover their disclosure plan

A `PresentationDisclosurePlan` SHALL privately retain the exact
`PresentationRequest` used during construction. A `GeneratedPresentation`
SHALL contain 1–64 artifacts totaling at most 16 MiB and validate them against
one supplied request and disclosure plan. The supplied request SHALL equal the
plan's private request snapshot. Every artifact binding SHALL identify a
selection in the plan, SHALL use that selection query's format, and SHALL
appear in exactly one artifact. Every plan selection SHALL be represented
exactly once across the artifact collection. Construction SHALL retain caller
artifact order and transferred payload vectors.

#### Scenario: OID4VP and Midnight results preserve adapter output

- **WHEN** format adapters return bounded artifacts whose bindings exactly
  cover a validated plan
- **THEN** the generated presentation SHALL preserve artifact order, formats,
  bindings and exact bytes without claiming proof validity or delivery

#### Scenario: cross-object and resource mismatches fail closed

- **WHEN** a plan belongs to another request, artifacts are empty or
  oversized, total bytes overflow or exceed 16 MiB, a binding is unknown or
  repeated across artifacts, a selected credential is omitted, or an artifact
  format differs from a bound query
- **THEN** construction SHALL return a stable static error without retaining a
  diagnostic copy of caller data

### Requirement: Receipt inputs summarize generated disclosures without values

The SDK SHALL derive `PresentationReceiptInput` only from a validated
`GeneratedPresentation`. It SHALL contain the request verifier, optional
purpose and one ordered `PresentationReceiptEntry` per disclosure-plan
selection. Each entry SHALL contain query ID, opaque credential handle,
credential format and value-free selected claim path/intent descriptors.
Entries SHALL follow disclosure-plan order regardless of artifact order. The
receipt input SHALL exclude request challenge, artifact/proof bytes, claim
values, openings, predicate parameters, transport state, timestamp, outcome,
verification result and trust decision.

#### Scenario: a product receives the minimum owner-private disclosure summary

- **WHEN** artifacts were constructed for a validated DCQL-, Midnight- or
  unrelated-format plan
- **THEN** receipt input SHALL retain who was addressed, optional purpose, the
  selected local credentials, formats and value-free disclosure descriptors
  without retaining challenge or artifact contents

#### Scenario: receipt input does not assert external success

- **WHEN** a consumer derives a receipt input before applying its own
  transport completion and persistence policy
- **THEN** the type SHALL make no claim of consent, sending, receipt by a
  verifier, verifier acceptance, proof validity or credential trust

### Requirement: Artifact and receipt diagnostics protect presentation contents

Debug SHALL NOT render verifier, purpose, challenge, query ID, credential
handle, claim path or artifact bytes for artifact bindings, artifacts,
generated presentations, receipt entries, receipt inputs or disclosure plans.
It MAY expose safe format/intent values, lengths, counts and optional-field
presence. Every new failure SHALL map to an `IdentusError` with capability
`presentation`, kind `InvalidInput`, a stable static `presentation.*` code and
static public text. Collection and byte bounds SHALL be checked before bounded
pairwise correlation scans, total length SHALL use checked arithmetic, and
constructors SHALL add no temporary map or set and no dependency. A manual
ignored release diagnostic SHALL report representative generated-presentation
and receipt-input throughput without a machine-dependent threshold.

#### Scenario: artifact and receipt canaries do not enter diagnostics

- **WHEN** verifier, purpose, challenge, query, handle, path and payload
  canaries are formatted through every new public value and error bridge
- **THEN** no canary SHALL appear in Debug, Display, error code or public text

#### Scenario: codecs, transport and receipt policy stay downstream

- **WHEN** a consumer needs proof execution, VP Token encoding, HTTP or DC API
  transport, delivery acknowledgement, timestamping, persistence, retention or
  audit policy
- **THEN** it SHALL layer that behavior outside the generated-presentation and
  receipt-input contracts
