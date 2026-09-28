# Wallet storage conformance ownership

## ADDED Requirements

### Requirement: Exact-record lifecycle phases are explicit and transcript-bound

The SDK MUST implement the exact-record conformance scenario as named private
lifecycle phases behind the unchanged public checker functions. Scenario
orchestration, phase assertions, and aggregate evidence counting MUST remain
separate reviewable responsibilities while preserving the exact successful
operation transcript and static failure contract.

#### Scenario: Exact adapter satisfies the complete lifecycle

- **WHEN** an adapter correctly implements missing reads, insert-only writes,
  scoped reads, replacement, revision conflicts, and conditional deletion
- **THEN** the suite performs the same 16 port calls in the same order
- **AND** returns 16 completed operations and zero observed list entries.

#### Scenario: Exact adapter violates one lifecycle invariant

- **WHEN** a load, write, conflict, revision, scope, value, or delete result
  violates its phase contract
- **THEN** the suite fails at the existing static step with the existing closed
  failure kind
- **AND** no fixture value, revision, or adapter error enters diagnostics.

#### Scenario: Private responsibilities are decomposed

- **WHEN** code-health evidence is refreshed after the refactor
- **THEN** no exact-scenario function exceeds the configured function attention
  thresholds
- **AND** the improvement comes from named invariant phases rather than a
  forwarding wrapper, generated code, moved tests, or a metric waiver.

### Requirement: Scenario ownership preserves the public verification boundary

Exact-record and list traversal implementations MUST remain private siblings
behind the crate-root fixture types and five public async checks. They MUST NOT
add adapter authority, runtime selection, dynamic dispatch, dependencies,
features, or supported public paths.

#### Scenario: Existing consumer rebuilds

- **WHEN** consumer test code imports fixtures, failures, reports, and checker
  functions from `identus_wallet_conformance`
- **THEN** it compiles without source changes and observes the same trait bounds,
  results, counts, failures, and diagnostics.

#### Scenario: List-capable check combines evidence

- **WHEN** credential, DID, or protocol-state conformance completes exact and
  list scenarios
- **THEN** the existing aggregate report combines their operation and observed-
  entry counts without sharing or conflating their private invariants.
