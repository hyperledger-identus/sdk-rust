# cross-language-compatibility-foundation Specification

## Purpose
TBD - created by archiving change specify-a1-compatibility-foundation. Update Purpose after archive.
## Requirements
### Requirement: Compatibility evidence is split into cohesive versioned records

The SDK SHALL maintain independently versioned machine records for vector
provenance, canonical Rust-to-language DTO/error mappings, consumer-visible
changes, and risk-routed quality evidence. Each record SHALL have a stable ID
and SHALL reference other records by ID rather than duplicate their payload.

#### Scenario: two records use the same fixture

- **WHEN** an adapter mapping and quality declaration rely on one vector
- **THEN** both reference the vector's stable ID and immutable payload hash
  instead of embedding separate copies

### Requirement: Vector authority and redistribution are explicit

Every vector SHALL record its authority class, exact source revision or
normative version, source path/section, authorship and redistribution decision,
immutable inputs, expected output or error, profile scope, owning capability,
and exact test selectors. Repetition across donor SDKs SHALL NOT promote a
consumer regression to normative authority.

#### Scenario: copied language-SDK tests agree

- **WHEN** SDK-TS, SDK-Swift, and SDK-KMP contain the same DID example
- **THEN** the catalog records the relevant pinned sources as consumer evidence
  and selects authority from the standard or an accepted Identus profile

### Requirement: Rust contracts remain canonical across language adapters

Every adapter mapping SHALL identify the canonical Rust DTO or error code and
the versioned language-specific shape, conversion direction, compatibility
class, vector evidence, migration window, deprecation state, and rollback.
Language DTOs and errors SHALL NOT constrain the canonical Rust contract.

#### Scenario: a language SDK exposes a legacy error

- **WHEN** migration requires retaining that error shape
- **THEN** a tested outer mapping may preserve it for a bounded version window
  without adding the legacy shape to the Rust core

### Requirement: Consumer-visible changes are recorded before merge

The SDK SHALL record every additive, fixed, behavioral, deprecated, breaking,
or security-relevant consumer change with affected capabilities, packages,
mappings, old
and new behavior, migration action and window, release-note class, rollback,
and exact vector and quality evidence. Ledger validation SHALL reject dangling
references.

#### Scenario: a change is described as internal

- **WHEN** its public, wire, error, persistence, ABI, target, or runtime effect
  differs for a supported consumer
- **THEN** it is ledgered with its actual consumer-visible classification

### Requirement: Quality obligations are risk-routed and exact

Each delivered capability SHALL declare property, fuzz, benchmark, and
differential obligations. Every class SHALL either reference exact selectors,
seeds, thresholds, artifacts, and target/risk routing or include a reviewed
not-applicable rationale. A generic CI run SHALL NOT satisfy an unspecified
quality class.

#### Scenario: a quality class is not useful for a slice

- **WHEN** the responsible issue records a reviewed not-applicable rationale
- **THEN** validation may accept the declaration without inventing evidence

### Requirement: A1 precedes consumer mutation

The A1 parent SHALL close only after all four record contracts, offline
validators, mutation tests, seed DID records, and combined graph validation
are complete. The first SDK-TS DID canary SHALL remain a downstream adoption
proof blocked by the A1 completion receipt.

#### Scenario: the schemas are planned but SDK-TS is unchanged

- **WHEN** the A1 planning package merges
- **THEN** no consumer adoption or target support is claimed and implementation
  proceeds only through the four child issues
