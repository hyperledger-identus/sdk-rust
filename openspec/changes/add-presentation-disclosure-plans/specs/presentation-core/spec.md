# Presentation disclosure plan delta specification

## ADDED Requirements

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
`PresentationCandidateSet`. Construction SHALL revalidate the candidate set
against the supplied request, reject duplicate query-ID/handle pairs, require
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
