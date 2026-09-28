## ADDED Requirements

### Requirement: Classifier implementation is cohesive and evidence-bound

The unpublished code-health classifier SHALL isolate protocol, cfg evaluation,
AST span projection, module reachability, and request orchestration behind
private cohesive modules. No classifier module SHALL exceed the 1,000 authored
nonblank production-line attention threshold after the change. Canonical policy
and report evidence SHALL bind the complete classifier source set and resolved
`syn`/`proc-macro2` package identities in addition to classifier name, protocol,
command, and population projection.

#### Scenario: Classifier source changes without evidence migration

- **WHEN** any private classifier source byte or bound parser package identity differs from canonical policy
- **THEN** fast validation rejects the evidence before trusting its population projection

#### Scenario: Private responsibilities are inspected

- **WHEN** the classifier module tree is measured as authored production source
- **THEN** protocol, cfg, span/projection, module graph, orchestration, and test responsibilities are discoverable and no production module exceeds the attention threshold

### Requirement: Supported resolution behavior has differential and bounded evidence

The classifier SHALL maintain deterministic fixtures for every supported module
role and production/test cfg state documented by ADR 0126. Conditional path
candidates SHALL be compared only in configurations where the edge is reachable,
and recursively applied `cfg_attr` predicates SHALL be evaluated in that active
configuration. Cargo target paths SHALL be lexically normalized and SHALL fail
closed if they escape the repository. Fast tests SHALL prove projection and
graph traversal stay within their monotonic byte/span and edge bounds. Heavier
near-limit timing evidence SHALL run only in weekly/manual validation.

#### Scenario: Disabled configuration names another path

- **WHEN** a module path applies only in a configuration where the module edge is disabled
- **THEN** that path does not create a false ambiguity or require an unreachable source

#### Scenario: Boolean literal is nested in cfg algebra

- **WHEN** `true` or `false` appears inside supported `all`, `any`, or `not` predicates
- **THEN** the classifier evaluates the literal exactly rather than silently treating the predicate as unknown

#### Scenario: Cargo target uses lexical parent components

- **WHEN** a contained manifest target path uses `.` or `..`
- **THEN** its normalized repository path matches the exact source key, while an escaping path is rejected

#### Scenario: Projection and graph approach their bounds

- **WHEN** deterministic generated inputs contain many disjoint spans or module edges
- **THEN** instrumentation proves projection advances monotonically and traversal work remains bounded by declared edges, while weekly/manual stress evidence records elapsed time without slowing the fast lane

#### Scenario: Migration prose drifts from canonical evidence

- **WHEN** a documented source, projection, or canonical report digest no longer matches policy/report data
- **THEN** the factory evidence test fails with an actionable consistency diagnostic
