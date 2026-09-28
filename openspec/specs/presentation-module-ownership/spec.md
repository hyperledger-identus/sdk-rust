# presentation-module-ownership Specification

## Purpose
TBD - created by archiving change decompose-presentation-model. Update Purpose after archive.
## Requirements
### Requirement: Presentation responsibilities have cohesive private owners

The SDK MUST keep bounded values, semantic requests, candidate/disclosure
validation, and generated artifact/receipt behavior in cohesive private owners
behind the unchanged crate-root presentation vocabulary.

#### Scenario: A semantic request is constructed

- **WHEN** callers construct scalar values, claim requests, filters, queries,
  or a presentation request
- **THEN** existing syntax, size, count, uniqueness, and value-free invariants
  apply in the same order with the same errors.

#### Scenario: Candidates and disclosures are validated

- **WHEN** candidates or selected disclosures are bound to a request
- **THEN** exact query, format, credential handle, claim path, intent,
  required-claim, coverage, and multiplicity invariants remain enforced
- **AND** DCQL, Midnight, and unrelated formats remain orthogonal.

#### Scenario: Generated artifacts cover a disclosure plan

- **WHEN** format adapters return opaque artifacts
- **THEN** existing binding uniqueness, format, selection coverage, per-artifact,
  artifact-count, and aggregate-byte invariants remain enforced
- **AND** receipt projection remains value-free and excludes the challenge.

### Requirement: Decomposition is compatibility preserving

The SDK MUST preserve existing public paths, signatures, visibility, derives,
constness, limits, errors, validation ordering, retained values, debug
redaction, dependencies, features, and target support.

#### Scenario: Consumers rebuild after the decomposition

- **WHEN** existing code imports presentation vocabulary from
  `identus_presentations`
- **THEN** it compiles without source changes
- **AND** no private child module becomes a supported public path.

#### Scenario: Code-health evidence is refreshed

- **WHEN** the ownership move is complete
- **THEN** the presentation hotspot is removed from the governed baseline
- **AND** no new over-threshold descendant or unrelated waiver is introduced.

