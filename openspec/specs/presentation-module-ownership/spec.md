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

### Requirement: Disclosure construction preserves phased validation authority

The SDK MUST keep public disclosure-plan construction separate from one private
deterministic owner that validates request, candidate, selection, selected-
claim, query-coverage, and multiplicity invariants.

#### Scenario: A valid disclosure plan is constructed

- **WHEN** ordered selections exactly reference compatible candidates and
  satisfy every request query and required claim
- **THEN** the plan retains the same cloned request and caller-ordered selection
  values as before decomposition
- **AND** introduces no extra public type or consumer bound.

#### Scenario: Multiple disclosure invariants are invalid

- **WHEN** an input violates more than one collection, candidate, duplicate,
  claim, coverage, or multiplicity invariant
- **THEN** validation returns the same existing error selected by the current
  phase and caller/request iteration order
- **AND** no request, query, credential, path, or claim value enters diagnostics.

#### Scenario: Validation remains bounded and cohesive

- **WHEN** the private owner traverses selections, selected claims, candidates,
  and request queries
- **THEN** existing constructor-enforced resource limits remain authoritative
- **AND** no callback table, dynamic dispatch, synchronization, dependency,
  generic framework, metric waiver, or forwarding-only helper chain is added.

### Requirement: Generated presentation construction preserves phased validation authority

The SDK MUST keep public generated-presentation construction separate from one
private deterministic owner that validates aggregate payload, artifact binding,
format, duplicate, and plan-coverage invariants.

#### Scenario: A valid generated presentation is constructed

- **WHEN** ordered artifacts exactly cover a request-bound disclosure plan with
  compatible formats and a bounded aggregate payload
- **THEN** the generated presentation retains the same owned plan and caller-
  ordered artifact values as before decomposition
- **AND** introduces no extra public type or consumer bound.

#### Scenario: Multiple generated-artifact invariants are invalid

- **WHEN** input violates more than one collection, request, payload-budget,
  selection, format, duplicate, or coverage invariant
- **THEN** validation returns the same existing error selected by the current
  phase and artifact/binding/plan iteration order
- **AND** no query, credential, format, payload, or claim value enters
  diagnostics.

#### Scenario: Validation remains bounded and cohesive

- **WHEN** the private owner traverses artifacts, bindings, plan selections,
  and request queries
- **THEN** existing constructor-enforced resource limits remain authoritative
- **AND** no index allocation, callback table, dynamic dispatch,
  synchronization, dependency, generic framework, metric waiver, or forwarding-
  only helper chain is added.

