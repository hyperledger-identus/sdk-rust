# Presentation disclosure validation ownership

## ADDED Requirements

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
