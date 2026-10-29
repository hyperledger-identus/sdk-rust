## ADDED Requirements

### Requirement: Staged lock refreshes are closed and reviewable

The SDK SHALL refresh a candidate staged lock only through an explicit issue,
review-only resolver output, complete added/removed package report, exact lock
digest update, and rerun of deterministic archive, closure, API, supply-chain,
compiler, and target evidence. Direct dependency requirements, candidate
identity, and publication state SHALL remain unchanged unless separately
specified.

#### Scenario: a frozen lock becomes stale

- **WHEN** clean `--locked` candidate preparation rejects the reviewed lock
- **THEN** the SDK generates a proposal outside the repository and reviews the
  complete dependency drift before changing tracked lock bytes

#### Scenario: undeclared drift appears

- **WHEN** the adopted lock differs from the closed proposal or contains an
  undeclared package, version, source, or checksum change
- **THEN** qualification fails before the lock can become release evidence
