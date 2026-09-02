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
