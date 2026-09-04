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

### Requirement: Protocol state separates active phases and terminal outcomes

The SDK SHALL provide a `PresentationLifecyclePhase` vocabulary containing
`requested`, `awaiting_authorization`, `generating`, `ready`, `delivering` and
`cancellation_requested`, and a distinct `PresentationTerminalOutcome`
vocabulary containing `completed`, `refused`, `cancelled`, `expired` and
`failed`. A `PresentationProtocolState` SHALL contain exactly one active phase
or terminal outcome and SHALL report terminality without allocation.

`awaiting_authorization` SHALL express only an unsatisfied external
prerequisite. `completed` SHALL express only that the protocol adapter reports
terminal completion; neither value SHALL assert user consent, proof validity,
verifier acceptance, credential trust or successful persistence.

#### Scenario: unrelated adapters share truthful progress

- **WHEN** OID4VP-, Midnight- and dummy-protocol coordinators prepare,
  authorize, generate, expose and deliver presentations
- **THEN** each SHALL use the same active/terminal vocabulary while retaining
  its protocol-specific substate and evidence outside the generic core

#### Scenario: completion does not become verifier evidence

- **WHEN** an adapter reaches `completed`
- **THEN** the state SHALL make no claim about proof verification, verifier
  acknowledgement, credential trust or receipt persistence

### Requirement: Presentation lifecycle transitions are conservative and exact

The SDK SHALL accept only these directed state transitions:

- `requested` to `awaiting_authorization`, `refused`, `cancelled`, `expired`
  or `failed`;
- `awaiting_authorization` to `generating`, `refused`, `cancelled`, `expired`
  or `failed`;
- `generating` to `ready`, `cancellation_requested`, `cancelled`, `expired` or
  `failed`;
- `ready` to `delivering`, `cancelled`, `expired` or `failed`;
- `delivering` to `cancellation_requested`, `completed`, `cancelled`,
  `expired` or `failed`; and
- `cancellation_requested` to `completed`, `cancelled`, `expired` or `failed`.

Every other pair, including self-transition, backward progress, a skipped
generation/delivery boundary and any transition from a terminal outcome,
SHALL fail with one static typed error.

#### Scenario: irreversible completion wins a cancellation race

- **WHEN** cancellation is requested while delivery may already be
  irreversible and the adapter subsequently observes terminal completion
- **THEN** `cancellation_requested` SHALL be allowed to become `completed`
  rather than fabricate rollback

#### Scenario: terminal truth cannot be rewritten

- **WHEN** a caller attempts to transition a completed, refused, cancelled,
  expired or failed state
- **THEN** the guard SHALL reject the attempt without changing either value

### Requirement: Lifecycle spellings are stable, strict and dependency-free

Every phase, outcome and composed state SHALL expose its exact lowercase
snake-case spelling and SHALL round-trip through strict `FromStr` parsing.
Unknown, padded or differently cased spellings SHALL return static redacted
invalid-input errors. The spellings SHALL be an adapter mapping seam, not a
serde schema or protocol wire compatibility claim.

#### Scenario: storage and protocol adapters map explicitly

- **WHEN** an adapter maps every accepted lifecycle spelling to its own
  versioned representation
- **THEN** the generic values SHALL round-trip exactly without adding serde,
  storage or protocol dependencies to the presentation crate

#### Scenario: unrecognized state fails closed

- **WHEN** an adapter supplies an unknown, padded or differently cased phase,
  outcome or state
- **THEN** parsing SHALL reject it without retaining or rendering the input

### Requirement: Lifecycle diagnostics and execution remain narrow

Lifecycle values SHALL contain no verifier, purpose, challenge, query,
credential, claim, artifact, transport identifier, timestamp, error detail or
audit evidence. Invalid phase, outcome, state and transition failures SHALL map
to `IdentusError` with capability `presentation`, kind `InvalidInput`, a stable
static `presentation.*` code and static public text. Transition validation
SHALL allocate no memory and access no network, clock, randomness, runtime,
storage, chain or product service. An ignored release diagnostic SHALL report
complete transition-matrix throughput without a machine-dependent threshold.

#### Scenario: rejected input cannot enter diagnostics

- **WHEN** state parsing rejects a caller canary and errors are formatted or
  bridged
- **THEN** no caller input SHALL appear in Debug, Display, error code or public
  text

#### Scenario: orchestration stays downstream

- **WHEN** a consumer needs consent, authorization, compare-and-swap,
  persistence, retry, timestamps, transport acknowledgement or audit policy
- **THEN** it SHALL layer those behaviors around the allocation-free state
  contract
