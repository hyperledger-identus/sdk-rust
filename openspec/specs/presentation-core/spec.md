# presentation-core Specification

## Purpose

Define bounded, format-neutral presentation request and credential-candidate
semantics that protocol and product adapters can share without importing wire,
proof, consent, trust, storage, or chain behavior.
## Requirements
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

### Requirement: Selected claims preserve request intent without values

The SDK SHALL provide a `PresentationSelectedClaim` containing one
`CredentialClaimPath` and one `PresentationClaimIntent`. It SHALL contain no
claim value, opening, predicate parameter, label, display data, proof input or
format-specific path syntax. A credential selection SHALL contain zero to 64
selected claims with unique complete paths.

#### Scenario: reveal and predicate selections share one value-free shape

- **WHEN** an OID4VP-shaped reveal claim and a Midnight-shaped predicate claim
  are selected from their corresponding requests
- **THEN** both SHALL preserve the requested path and intent without importing
  protocol, product, circuit or claim-value data

#### Scenario: ambiguous selected claims fail closed

- **WHEN** one credential selection contains more than 64 selected claims or
  repeats a complete claim path
- **THEN** construction SHALL return a static typed error before the invalid
  collection escapes

### Requirement: Credential selections identify one available candidate

A `PresentationCredentialSelection` SHALL contain one presentation query ID,
one opaque credential handle and zero to 64 selected claims. It SHALL retain
the transferred selected-claim vector. The type SHALL make no claim that a
credential exists, remains available, is trusted, is valid, was selected by a
particular user interface or can produce a proof.

#### Scenario: a selected credential remains an opaque local reference

- **WHEN** a product has already authorized an available credential and its
  requested claims
- **THEN** the selection SHALL preserve the query correlation, opaque local
  handle and value-free claim intents without storage or consent semantics

### Requirement: Disclosure plans validate request and candidate consistency

A `PresentationDisclosurePlan` SHALL contain 1–64 credential selections and
SHALL validate them against one `PresentationRequest` and one
`PresentationCandidateSet`. A candidate set SHALL retain a private exact
request snapshot, and plan construction SHALL reject a supplied request that
differs in any field before revalidating the candidate set. Construction SHALL
reject duplicate query-ID/handle pairs, require
every selection to reference an available candidate, and require every
selected path and intent to match a claim requested by that query and a path
reported satisfiable by that candidate. Every required query claim SHALL be
selected. Every request query SHALL have at least one credential selection,
and a query whose `multiple` flag is false SHALL have exactly one. A query with
no explicit claim requests SHALL have no explicit selected claims. The same
credential handle MAY satisfy distinct query IDs.

#### Scenario: one plan carries OID4VP, Midnight and future-format choices

- **WHEN** authorized selections cover each query using available candidates,
  requested claim intents and permitted multiplicity
- **THEN** the plan SHALL retain selection order and exact opaque handles as a
  format-neutral proof-generation input

#### Scenario: cross-object mismatch and over-disclosure fail closed

- **WHEN** a plan is empty or oversized, omits a query, selects multiple
  credentials for a single-valued query, repeats a pair or path, references an
  unavailable candidate, selects an unrequested path or intent, selects a path
  unavailable from its candidate, omits a required claim, or pairs candidates
  with a different request
- **THEN** construction SHALL return the corresponding static error and SHALL
  retain no diagnostic copy of caller data

### Requirement: Disclosure plans remain private and allocation-conscious

Debug for selected claims, credential selections and disclosure plans SHALL
NOT render query IDs, credential handles or claim-path contents. It MAY expose
safe intent values, lengths and counts. Every new failure SHALL map to an
`IdentusError` with capability `presentation`, kind `InvalidInput`, a stable
static `presentation.*` code and static public text. Collection bounds SHALL be
checked before bounded pairwise membership, uniqueness and coverage scans.
Constructors SHALL add no temporary map or set and SHALL add no dependency. A
manual ignored release diagnostic SHALL report representative disclosure-plan
validation throughput without a machine-dependent threshold.

#### Scenario: selected canaries never enter diagnostics

- **WHEN** query, handle and path canaries are formatted through every new
  public value and every new error is bridged
- **THEN** no canary SHALL appear in Debug, Display, error code or public text

#### Scenario: selection policy and protocol behavior stay downstream

- **WHEN** a consumer needs ranking, consent, DCQL alternative sets, proof
  execution, a presentation artifact, receipt, lifecycle state or persistence
- **THEN** it SHALL layer that behavior outside the disclosure-plan contract
