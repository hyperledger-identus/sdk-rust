# identus-platform-core-migration Specification

## Purpose
TBD - created by archiving change establish-identus-platform-core-program. Update Purpose after archive.
## Requirements
### Requirement: Reusable Identus semantics have one authoritative core

The program SHALL designate `sdk-rust` as the authoritative owner of reusable
Identus domain and protocol semantics. TypeScript, Swift, Kotlin, and React
Native surfaces SHALL compose that core through isolated adapters and retain
only reviewed language ergonomics, packaging, platform integration, product
policy, and time-bounded compatibility behavior.

#### Scenario: behavior is reusable across platforms

- **WHEN** a capability has equivalent domain or protocol semantics in more
  than one Identus SDK
- **THEN** its target owner is an appropriate generic `identus-*` Rust crate
  unless recorded evidence selects an upstream dependency or a justified
  platform-specific owner

#### Scenario: repository deletion is not assumed

- **WHEN** a language SDK still owns supported packaging or platform behavior
- **THEN** it remains an idiomatic facade or platform shell even after its
  duplicate reusable implementation is removed

### Requirement: Every capability has an explicit disposition and evidence

Before a capability migration begins, the registry SHALL record its stable
identifier, legacy owner and revision, target owner, disposition, specification
basis, public/wire/persistence surface, test authority, binding target,
compatibility class, deprecation phase, legacy-bug policy, dependencies,
security notes, issue, and evidence links. Allowed dispositions SHALL include
`move-to-rust`, `retain-platform`, `replace-upstream`, `deprecate`, `drop`, and
`defer`.

#### Scenario: unsupported or duplicate behavior is found

- **WHEN** inventory finds a feature with no continuing product, standard, or
  consumer justification
- **THEN** the program records `deprecate` or `drop`, a replacement if one
  exists, and phased exit criteria instead of porting it automatically

#### Scenario: evidence is incomplete

- **WHEN** current consumers, normative behavior, or target support cannot be
  established
- **THEN** the capability remains `defer` and cannot enter implementation

### Requirement: Test evidence has declared authority

The program SHALL classify relevant tests as `normative`, `identus-contract`,
`consumer-regression`, `implementation-regression`, or `exploratory`. Only the
first three SHALL constrain cross-SDK compatibility without an explicit
promotion decision. Shared vectors SHALL include provenance, immutable inputs,
expected outputs or errors, version/profile scope, and owning capability.

#### Scenario: legacy tests disagree

- **WHEN** two SDK tests encode conflicting behavior
- **THEN** the program resolves the conflict using normative sources, reviewed
  Identus profile decisions, and consumer evidence rather than majority vote

#### Scenario: implementation accident has useful coverage

- **WHEN** a local regression test has no public or normative basis
- **THEN** it may guide implementation quality but does not force compatibility
  until explicitly promoted

### Requirement: Consumer-visible changes drive migration and release evidence

The migration program SHALL record every consumer-visible additive, fixed,
behavioral, deprecated, breaking, or security change in a machine-readable
ledger before merge. The record SHALL identify affected capabilities/packages,
old and new behavior, replacement, compatibility window, migration action,
first/last version, release-note class, and evidence. Migration guides and
release notes SHALL be generated or reviewed from this ledger.

#### Scenario: a change is believed to be non-breaking

- **WHEN** a capability adds behavior or fixes an implementation defect without
  changing a supported contract
- **THEN** it is still recorded with evidence so downstream release notes and
  later compatibility analysis do not depend on memory

#### Scenario: a public contract changes

- **WHEN** names, behavior, errors, wire values, persistence, ABI, or runtime
  requirements change incompatibly
- **THEN** a migration action, compatibility window, and rollback are required
  before merge

### Requirement: Deprecation and legacy-bug compatibility are phased

Deprecation SHALL progress through explicit evidence-gated phases and SHALL NOT
remove behavior solely by date. A legacy bug SHALL default to
`fix-and-document`; it MAY be simulated only for a proven released-consumer
dependency through a bounded, versioned, testable, observable, owned, and
time-limited outer compatibility mode. Security or privacy defects SHALL fail
closed and SHALL NOT be simulated.

#### Scenario: replacement is not ready

- **WHEN** a deprecated capability lacks a supported replacement and consumer
  migration evidence
- **THEN** it cannot advance to `default-off` or `removed`

#### Scenario: a consumer requires historical buggy behavior

- **WHEN** an atomic migration is impractical and the behavior is safe to
  reproduce
- **THEN** a compatibility facade may simulate it with an explicit opt-in or
  version scope, owner, warning/detection path, tests, and removal release

#### Scenario: historical behavior weakens security or privacy

- **WHEN** reproducing a bug would expose secrets, accept invalid trust, weaken
  cryptography, bypass bounds, or corrupt persisted state
- **THEN** the migration fails closed and documents an intentional break

