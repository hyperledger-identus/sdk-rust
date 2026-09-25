# ssi-upstream-program Specification

## Purpose

Define the repository-owned, executable SSI upstream dependency program that
orders generic component delivery from Apollo, NeoPRISM, midnight-identity,
Lace ID Portal and Oxid evidence without coupling the SDK to those repositories.
## Requirements
### Requirement: The SDK owns a complete canonical backlog

The repository SHALL contain exactly one canonical CSV row for every `IDR-*`
entry in the supplied portfolio backlog and SHALL exclude `MID-*` entries from
SDK ownership. Each row SHALL preserve its priority, target gate, outcome,
acceptance evidence, consumer dependency and portfolio commitment.

#### Scenario: Canonical SDK backlog is validated

- **WHEN** the backlog validation command reads the canonical CSV
- **THEN** all thirty expected `IDR-*` identifiers are present exactly once and
  no non-`IDR-*` owner row is accepted

#### Scenario: Source row is lost or duplicated

- **WHEN** an identifier is absent or appears more than once
- **THEN** validation fails and names the missing or duplicate identifier

### Requirement: Portfolio commitment is distinct from delivery status

Every row SHALL carry a source-derived `commitment` and a separately evaluated
`delivery_status`. Allowed delivery states SHALL be `delivered`,
`in_progress`, `specified`, `queued`, and `conditional`. A row SHALL NOT be
marked `delivered` merely because its commitment is `Foundation` or
`Committed`.

#### Scenario: Committed work is not implemented

- **WHEN** a source row is committed but has no immutable acceptance evidence
- **THEN** its delivery status remains `specified`, `queued`, or `in_progress`
  rather than `delivered`

### Requirement: Every row has durable issue ownership

Every row SHALL reference a repository issue. A queued or conditional row MAY
reference the program issue, but implementation SHALL NOT begin until a focused
component issue contains the slice contract. Existing component issues SHALL be
reused where their scope is still valid.

#### Scenario: Unplanned row remains in the program queue

- **WHEN** no focused component issue exists for a row
- **THEN** the row references issue #20 and remains `queued` or `conditional`

#### Scenario: Component implementation begins

- **WHEN** a row enters implementation
- **THEN** it references a focused issue with sources, scope, non-scope,
  threats, dependency cone, target matrix and objective acceptance evidence

### Requirement: Repository boundaries remain directional

The SDK SHALL own chain-neutral SSI types, ports, protocol engines,
cryptographic utilities and conformance evidence. It SHALL NOT own Midnight or
Compact runtime behavior, PRISM/Cardano ledger integration, wallet/product
policy, trust decisions, consent, custody, UI, deployment or concrete product
storage.

#### Scenario: Generic behavior is found in a consumer

- **WHEN** equivalent behavior is useful to at least two independent consumers
- **THEN** its source is classified for a focused SDK slice without adding a
  dependency from the SDK to the consumer

#### Scenario: Source behavior is chain- or product-specific

- **WHEN** behavior depends on Midnight, Compact, PRISM operations, Cardano,
  product policy, custody, UI or deployment
- **THEN** it is classified `remain-downstream` and excluded from the generic
  SDK dependency graph

### Requirement: Source evidence is immutable and classified

The program SHALL record an immutable repository revision and path for each
candidate source surface and classify it as `extract`, `adapt`,
`conformance-only`, or `remain-downstream`. A later code or fixture port SHALL
also record its license and transformation.

#### Scenario: Apollo behavior informs Rust compatibility

- **WHEN** Kotlin Apollo behavior or vectors are used
- **THEN** Apollo is classified as `conformance-only` and the Rust
  implementation source is independently identified

#### Scenario: Source revision advances

- **WHEN** a donor default branch changes after inspection
- **THEN** the canonical revision remains unchanged until a reviewed update
  records the new evidence

### Requirement: Backlog drift fails the repository gate

The repository SHALL provide a deterministic validation command that rejects
schema drift, invalid identifiers or enumerations, duplicate rows, missing
issue links, non-SDK ownership and unknown source aliases. The command SHALL
run in the existing structural CI path without building a donor repository.

#### Scenario: Valid canonical backlog is checked

- **WHEN** local or hosted structural checks run
- **THEN** the validator exits successfully without network or donor access

#### Scenario: Invalid field is introduced

- **WHEN** a row contains an unknown status, malformed issue or unknown source
- **THEN** the validator exits non-zero with a field-specific diagnostic

### Requirement: Downstream reduction follows compatible upstream delivery

Apollo deprecation and NeoPRISM reduction SHALL NOT be claimed by this program
contract alone. A downstream MAY remove or repoint generic implementation only
after an immutable SDK candidate provides equivalent behavior and its own
adoption issue passes compatibility evidence.

#### Scenario: SDK component is only specified

- **WHEN** the equivalent SDK component has not produced an immutable candidate
- **THEN** Apollo and NeoPRISM remain unchanged

#### Scenario: Compatible SDK candidate exists

- **WHEN** the component passes its conformance and consumer-shaped adapter
  evidence
- **THEN** a separate downstream issue may plan dependency repointing and code
  removal without changing the SDK component issue

### Requirement: IDR-010 storage ports have focused ownership

The canonical backlog SHALL keep `IDR-010` at `specified` under focused issue
#91 after the port contract from #89 gains reusable behavioral conformance.
It SHALL remain short of delivered until independent in-memory and encrypted
consumer adapters publish immutable conformance receipts. Concrete product
storage SHALL remain downstream.

#### Scenario: Reusable conformance enters implementation

- **WHEN** issue #91 adds the wallet storage test-support crate
- **THEN** the IDR-010 row references #91 with status `specified` and preserves
  its existing priority, gate, predecessors, outcome and acceptance evidence

#### Scenario: SDK-local harness passes without consumer receipts

- **WHEN** all generic checks and SDK-local test doubles pass but fewer than two
  independent immutable consumer receipts exist
- **THEN** IDR-010 remains `specified` rather than `delivered`

### Requirement: IDR-023 begins with a focused Credential Offer transport slice

The canonical backlog SHALL keep `IDR-023` at `in_progress` and reference open
focused child #350 after #348 delivers bounded unencrypted Deferred Credential
payload-error handling. Issue #250 delivered bounded request construction,
issue #345 delivered bounded issued/pending success responses with exact
pending transaction correlation, and issue #348 composes the existing
Credential Error parser with deferred-specific classification and lifecycle
guidance. Issue #350 owns the next authorization-code server-binding seam.

The active slice SHALL NOT claim the full OID4VCI engine, HTTP execution, RFC
6750 challenge parsing, token ownership/validation, TLS, interval scheduling,
retry/invalidation effects, request/response encryption, credential
verification/storage, proof-count correlation, format/chain extensions,
consumer adoption, publication or release.

#### Scenario: Payload-error child hands off to authorization-code binding

- **WHEN** issue #348 is delivered and focused successor #350 is open
- **THEN** `IDR-023` references #350 with `delivery_status=in_progress`
- **AND** #7 and #20 remain open component and program parents

#### Scenario: payload-error validation is not engine completion

- **WHEN** a Deferred Credential Request validates and classifies bounded
  payload errors without transport, authorization, timing or lifecycle effects
- **THEN** the backlog keeps `IDR-023` in progress rather than delivered

### Requirement: Active backlog ownership is live before autonomous selection

The repository SHALL provide a network-explicit, read-only audit that validates
every canonical backlog issue reference against the configured GitHub
repository before a supervisor selects an `in_progress` row for autonomous
implementation. Every referenced issue SHALL exist and be visible, and every
`in_progress` row SHALL reference an open durable component or focused issue.

The required offline backlog checker SHALL remain network-free. A `specified`
or `delivered` row MAY retain a closed delivery issue as durable evidence; it
MUST receive a new focused open issue and become `in_progress` before further
implementation begins.

#### Scenario: Active owner is open

- **WHEN** the live audit resolves every referenced issue and each
  `in_progress` owner is open
- **THEN** it reports the ledger fresh without claiming that any component is
  complete, conformant or correctly prioritized

#### Scenario: Active owner is closed

- **WHEN** an `in_progress` row references a closed issue
- **THEN** the live audit fails with the row and issue identifiers before a
  supervisor delegates implementation

#### Scenario: Coordination state is unavailable

- **WHEN** GitHub, authentication or issue visibility cannot resolve any
  referenced issue
- **THEN** the live audit fails closed while the deterministic offline checker
  and required fast CI remain independently usable

#### Scenario: Specified evidence is closed

- **WHEN** a `specified` row references the closed issue that established its
  current reusable contract
- **THEN** the live audit accepts the evidence reference but does not treat the
  row as selectable active implementation

### Requirement: IDR-004 delivery is functional and evidence-bounded

The canonical backlog SHALL mark IDR-004 `delivered` when every named
chain-neutral crypto and JOSE surface has immutable implementation evidence,
the executable Apollo comparison contains no open capability gap, standards
and compatibility vectors plus fuzz/coverage evidence pass, portable target
evidence is stated at its honest tier, and remaining limitations have separate
owners.

IDR-004 delivery SHALL NOT imply registry publication, SemVer or support
activation, production consumer adoption, language-binding support, mobile or
browser runtime proof, certification, Apollo deprecation, or NeoPRISM code
removal. Those outcomes SHALL remain under IDR-011, IDR-044, consumer adoption,
and downstream lifecycle authority.

#### Scenario: Functional evidence is complete

- **WHEN** every IDR-004 outcome is implemented or explicitly accepted with
  immutable evidence and the Apollo manifest reports zero gaps
- **THEN** IDR-004 may retain closed issue #286 as its delivered evidence owner

#### Scenario: Release or adoption is incomplete

- **WHEN** the candidate remains unpublished or a downstream adoption PR is
  open
- **THEN** IDR-004 may remain functionally delivered while release and
  lifecycle claims remain false under their separate owners

#### Scenario: Hardening remains independently owned

- **WHEN** a delivered surface has a bounded non-functional hardening issue
- **THEN** the report names that issue and limitation without representing the
  underlying functional capability as absent or the hardening as complete
