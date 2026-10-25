# consumer-change-ledger Specification

## ADDED Requirements

### Requirement: The consumer change ledger is closed, versioned, and honest

The SDK SHALL maintain a closed, independently versioned change ledger whose
records have stable IDs and additive lifecycle links. Governance or tooling
work with no consumer-visible effect SHALL leave the canonical ledger empty and
the rendered evidence SHALL state that no consumer-visible change is recorded.

#### Scenario: ledger governance is implemented

- **WHEN** schema, validation, and rendering are added without changing a
  consumer-visible contract
- **THEN** the canonical ledger contains no synthetic migration record and its
  deterministic views explicitly report the empty state

#### Scenario: a historical record is replaced

- **WHEN** a record needs correction or supersession after merge
- **THEN** a versioned replacement links both directions and the historical
  record remains available

### Requirement: Every consumer-visible change has a complete classification

Every record SHALL identify its capability, change and release-note classes,
visibility dimensions, affected packages and consumers, old and new behavior,
source and target versions, compatibility window, migration action,
deprecation and legacy-bug treatment, observability, fallback, rollback,
removal gate, issue, pull request, and exact evidence. Validation SHALL enforce
class-specific combinations rather than infer compatibility from paths,
language shapes, or SemVer alone.

#### Scenario: a supposedly internal change affects a supported error

- **WHEN** the stable error code or redaction contract changes for a consumer
- **THEN** the record includes the `error` visibility dimension and the actual
  compatibility class, migration, release note, and rollback evidence

#### Scenario: a breaking change is proposed

- **WHEN** a public, wire, error, persistence, ABI, target, runtime, behavioral,
  or security contract becomes incompatible
- **THEN** validation rejects the record unless its migration, bounded window,
  compatible versions, observability, fallback, and safe rollback are complete

### Requirement: Ledger evidence resolves across the A1 registries

Ledger records SHALL reference stable vector and quality-declaration IDs from
their canonical local registries. A language-adapter migration SHALL reference
stable adapter-mapping IDs; a Rust-only record SHALL carry an explicit reviewed
not-applicable mapping disposition. Validation SHALL reject duplicate,
inactive, dangling, capability-mismatched, or decoratively unrelated
references and SHALL NOT duplicate referenced payloads or treat a historical
quality receipt as current promotion evidence.

#### Scenario: a DID migration cites A1 evidence

- **WHEN** the synthetic DID fixture references #420 vectors, #505 mappings,
  and `did.syntax.quality.v1`
- **THEN** validation proves that every ID is active, capability-coherent, and
  connected through shared vectors without adding the fixture to the
  canonical ledger

#### Scenario: a quality receipt has aged

- **WHEN** the referenced quality declaration exists but its time-bounded
  receipt is not current for a release candidate
- **THEN** ledger validation resolves the declaration ID but makes no freshness
  or promotion claim

### Requirement: Deprecation, removal, and unsafe legacy behavior fail closed

Deprecation SHALL advance only through the evidence-gated phases defined by
ADR 0164, and removed records SHALL remain historical. Legacy-bug simulation
SHALL be bounded to safe outer compatibility behavior. A security or privacy
weakening SHALL select `reject-as-unsafe`; rollback SHALL NOT restore it.

#### Scenario: removal lacks consumer exit evidence

- **WHEN** a record proposes `default-off` or `removed` without a supported
  replacement, migration evidence, compatibility window, and removal gate
- **THEN** validation fails regardless of a calendar date

#### Scenario: a legacy behavior exposes secrets

- **WHEN** compatibility would reproduce secret leakage or weakened trust
- **THEN** temporary simulation is rejected and the record documents an
  intentional fail-closed security break

### Requirement: Qualifying changes declare compatibility impact explicitly

Every qualifying new OpenSpec change SHALL provide a closed machine-readable
compatibility-impact declaration. `consumer-visible` work SHALL cite one or
more canonical ledger IDs. `behavior-neutral` work SHALL cite no ledger ID and
SHALL provide a substantive rationale within a closed governance,
documentation, test-only, internal-refactor, or tooling scope. Omission and
contradiction SHALL fail readiness.

#### Scenario: an internal refactor preserves contracts

- **WHEN** a qualifying change declares behavior-neutral internal refactoring
  with zero ledger IDs and evidence that public, wire, error, persistence, ABI,
  target, runtime, and security behavior are unchanged
- **THEN** the factory accepts the disposition without inventing a migration

#### Scenario: a migration omits its ledger record

- **WHEN** a qualifying change declares or produces consumer-visible behavior
  but references no canonical ledger ID
- **THEN** the factory rejects the candidate before merge readiness

### Requirement: Migration and release evidence is deterministic and inert

The ledger SHALL render deterministic release-note, migration, compatibility-
window, rollback/removal, and limitation views. Normal validation SHALL fail on
render drift. Rendering and validation SHALL NOT execute stored commands,
fetch remote sources, publish notes, activate deprecation, select versions, or
authorize a release.

#### Scenario: records are reordered without semantic change

- **WHEN** equivalent validated input is presented in a different table order
- **THEN** the renderer produces the same stable sorted evidence

#### Scenario: the ledger is empty

- **WHEN** no canonical change record exists
- **THEN** every derived view states the empty condition without claiming a
  migration, release note, compatibility window, or rollback event
