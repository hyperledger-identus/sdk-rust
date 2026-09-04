# Presentation core delta specification

## ADDED Requirements

### Requirement: Presentation scalar roles are distinct, bounded, and private

The SDK SHALL provide distinct owned `PresentationQueryId`,
`PresentationPurpose`, `PresentationChallenge`, and
`PresentationCredentialHandle` values. Query IDs SHALL accept 1–128 ASCII
bytes, require an alphanumeric first byte, and allow only alphanumeric bytes
plus `.`, `_`, `-`, and `:` thereafter. Purpose SHALL accept exact non-empty
UTF-8 of at most 2,048 bytes with no surrounding whitespace or control
character. Challenge and handle SHALL preserve exact non-empty text or bytes of
at most 1,024 bytes; text SHALL have no surrounding whitespace or control
character. Validation SHALL precede owned-string allocation and transferred
byte vectors SHALL be retained.

#### Scenario: unrelated adapters preserve exact scalar values

- **WHEN** DCQL-, Midnight-, Oxid-, and dummy-format-shaped query IDs,
  purposes, challenges, and local handles satisfy their generic bounds
- **THEN** each value SHALL round-trip exactly through its role-specific
  accessor without protocol, chain, storage, or wire interpretation

#### Scenario: unsafe scalar input fails closed

- **WHEN** a scalar is empty, oversized, padded, control-containing, or a query
  ID violates its ASCII token grammar
- **THEN** construction SHALL return a static typed error without retaining or
  rendering caller input

### Requirement: Claim requests carry bounded generic disclosure intent

The SDK SHALL define stable round-tripping claim intents `reveal` and
`predicate`. A `PresentationClaimRequest` SHALL contain one
`CredentialClaimPath`, one intent, and one required flag, but no claim value,
predicate parameter, label, display content, or format-specific path syntax.

#### Scenario: clear and predicate requests share one claim shape

- **WHEN** an OpenID-shaped reveal request and a Midnight-shaped predicate
  request use credential claim paths
- **THEN** both SHALL retain path, intent, and required state without importing
  DCQL values, circuit arguments, threshold types, or product labels

#### Scenario: unknown claim intent is rejected

- **WHEN** an intent spelling is not `reveal` or `predicate`
- **THEN** parsing SHALL fail without allocation or fallback semantics

### Requirement: Credential queries are format-aware and bounded

A `PresentationCredentialQuery` SHALL contain one query ID, one
`CredentialFormat`, `multiple` and `requires_holder_binding` flags, optional
issuer/type/schema allow-lists, and zero to 64 claim requests. Each present
allow-list SHALL contain 1–16 unique credential descriptor values; absence
SHALL mean no generic restriction and an empty present list SHALL be rejected.
Complete claim paths SHALL be unique within a query. Construction SHALL not
match credentials, apply issuer trust, interpret schemas, or parse
format-specific metadata.

#### Scenario: DCQL, Midnight, and unrelated formats share one query

- **WHEN** adapters request an SD-JWT VC, a Midnight committed credential, and
  a dummy format with different issuer/type/schema filters and claim intents
- **THEN** all SHALL use the same query API while preserving their open
  credential format and accepted descriptors

#### Scenario: ambiguous query collections are rejected

- **WHEN** a present filter is empty, oversized, or duplicated, claim requests
  exceed 64, or a complete claim path repeats
- **THEN** query construction SHALL fail before the ambiguous state escapes

### Requirement: Presentation requests contain unique bounded queries

A `PresentationRequest` SHALL contain one verifier `CredentialEntityId`,
optional purpose, optional challenge, and 1–16 credential queries with unique
query IDs. Construction SHALL preserve query order and transferred vectors and
SHALL not add transport, response mode/URI, client metadata, origin, consent,
trust, protocol state, or authorization behavior.

#### Scenario: one verifier requests multiple credential formats

- **WHEN** a verifier requests one required SD-JWT VC query and one Midnight
  query through the same request
- **THEN** both ordered queries SHALL remain independently addressable without
  a protocol or product dependency

#### Scenario: empty, oversized, or duplicate query sets fail

- **WHEN** a request contains zero or more than 16 queries or repeats a query
  ID
- **THEN** construction SHALL reject it rather than invent order, overwrite,
  or sentinel semantics

### Requirement: Candidate sets are validated against their request

A `PresentationCredentialCandidate` SHALL contain one query ID, one local
credential handle, one credential format, and zero to 64 unique satisfiable
claim paths. A `PresentationCandidateSet` SHALL contain zero to 64 candidates
and validate each against one `PresentationRequest`: the query SHALL exist, the
format SHALL match, every candidate path SHALL be requested by that query,
every required query path SHALL be covered, and no query-ID/handle pair SHALL
repeat. An empty set SHALL represent a valid no-match result.

#### Scenario: complete candidates cover required claims

- **WHEN** candidates for SD-JWT VC and Midnight queries use matching formats,
  only requested paths, and all required paths
- **THEN** the set SHALL retain their order and exact handles without claiming
  credential validity, availability, disclosure ability, trust, or consent

#### Scenario: cross-query and over-disclosure mistakes fail

- **WHEN** a candidate references an unknown query, uses another format,
  repeats a pair/path, includes an unrequested path, omits a required path, or
  the set exceeds 64 candidates
- **THEN** construction SHALL return the corresponding static error

### Requirement: Presentation diagnostics and errors protect correlating data

Debug for scalar and aggregate presentation values SHALL NOT render verifier,
purpose, challenge, query ID, local handle, issuer filter, schema filter, or
claim-path contents. Aggregate Debug MAY expose safe format/intent values,
lengths, counts, booleans, and optional-field presence. Every presentation
construction error SHALL map to an `IdentusError` with capability
`presentation`, kind `InvalidInput`, a stable static `presentation.*` code, and
static public text.

#### Scenario: presentation canaries do not enter diagnostics

- **WHEN** known verifier, purpose, challenge, query, handle, filter, and path
  canaries are formatted directly or in aggregates and every error is bridged
- **THEN** no sensitive or caller-rejected canary SHALL appear

### Requirement: Presentation validation stays allocation-conscious and narrow

All collection bounds SHALL be checked before duplicate, membership, or subset
scans. Constructors SHALL retain caller-owned vectors and use bounded pairwise
comparisons without temporary maps or sets. A manual ignored release diagnostic
SHALL report representative request/candidate validation throughput without a
machine-dependent correctness threshold. The crate SHALL add no serde/wire,
network, runtime, storage, crypto, chain, product, or external dependency.

#### Scenario: the hot construction path is measurable and portable

- **WHEN** the release diagnostic constructs and validates representative
  requests and candidate sets through the public API
- **THEN** it SHALL report elapsed time and throughput while normal correctness
  tests remain independent of host timing

#### Scenario: adapters retain protocol and product ownership

- **WHEN** an adapter needs DCQL encoding, candidate lookup/ranking, consent,
  proof execution, lifecycle state, persistence, or a presentation artifact
- **THEN** it SHALL layer that behavior outside this request/query/candidate
  core
