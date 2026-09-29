# Generated presentation validation ownership

## ADDED Requirements

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

