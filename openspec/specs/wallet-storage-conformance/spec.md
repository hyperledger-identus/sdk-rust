# wallet-storage-conformance Specification

## Purpose
TBD - created by archiving change add-wallet-storage-conformance. Update Purpose after archive.
## Requirements
### Requirement: Behavioral conformance is reusable and runtime-neutral

The SDK SHALL provide a verification-layer `identus-wallet-conformance` crate
whose public async checks invoke the five `identus-wallet` storage traits
directly. The crate SHALL select no executor and SHALL have `identus-wallet` as
its only runtime dependency. No production SDK crate SHALL depend on it.

#### Scenario: Consumer uses its native test runtime

- **WHEN** a downstream adapter awaits the matching conformance entry point
- **THEN** the actual production port is exercised without importing an SDK
  runtime, database, codec, encryption or product type

### Requirement: Exact-record suites prove conditional lifecycle semantics

Each store suite SHALL cover missing load, insert-only success, read-after-write,
duplicate insert conflict, unconditional replacement, stale-revision write and
delete conflict, current-revision deletion and unconditional deletion of a
missing record. It SHALL verify isolated scopes and SHALL verify that failed
preconditions preserve the current record.

Fixtures SHALL use consumer-owned scope, key and value types. Test-only bounds
MAY require values to implement `Clone + PartialEq`; no consumer type SHALL be
required to implement Debug, Display, serialization, hashing or ordering.

#### Scenario: Adapter loses compare-and-swap safety

- **WHEN** an adapter accepts a stale revision, reuses an invalidated revision,
  mutates after a failed precondition or leaks a record across scopes
- **THEN** the matching exact-record suite fails at a static named step

### Requirement: List suites prove bounded terminating recovery traversal

Credential, DID and protocol-state entry points SHALL accept a preseeded
consumer fixture with exact expected index entries. They SHALL request pages
smaller than the expected set and verify per-request size, cursor progress,
bounded termination, absence of duplicate observations and exact expected
multiset membership. Secret and status-cache entry points SHALL require and
exercise no list authority.

#### Scenario: Pagination stalls or exceeds authority

- **WHEN** a list-capable adapter returns too many entries, repeats a cursor,
  fails to terminate, duplicates an entry or omits/adds an expected entry
- **THEN** conformance fails without formatting the entry or cursor value

### Requirement: Receipts and failures are redaction-safe

Successful checks SHALL return only aggregate operation and observed-entry
counts. Failures SHALL expose only a static suite step and closed failure kind.
Debug and Display SHALL never contain fixture scopes, keys, values, index
entries, revisions or cursors and SHALL not require those types to implement a
formatting trait.

#### Scenario: Secret-bearing fixture fails

- **WHEN** a non-Debug canary fixture triggers each public failure class
- **THEN** the crate compiles and rendered diagnostics expose only the static
  step and failure class

### Requirement: Local proof does not substitute for downstream adoption

The crate SHALL prove every public entry point with a test-only memory
implementation and SHALL include an ignored release performance diagnostic
without a machine-dependent threshold. `IDR-010` SHALL remain short of
delivered until two independent downstream implementations, including an
encrypted adapter, publish immutable conformance receipts.

#### Scenario: SDK-local test kit passes

- **WHEN** all local memory tests and diagnostics succeed but downstream
  receipts do not exist
- **THEN** the backlog remains `specified` and makes no storage, encryption or
  custody implementation claim

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

### Requirement: List traversal separates adapter calls from bounded evidence

The SDK MUST keep asynchronous list-port orchestration separate from the
deterministic bounded evidence that validates returned pages, entries,
continuation progress, termination, and final membership. Both responsibilities
MUST remain private behind the existing list-capable checker functions.

#### Scenario: Adapter returns a valid paginated index

- **WHEN** every page respects the requested bound, entries are unique, cursors
  progress, traversal terminates, and the observed set equals the fixture
- **THEN** the scenario preserves the exact request sequence and operation count
- **AND** returns the same aggregate report as before decomposition.

#### Scenario: Adapter violates a pagination invariant

- **WHEN** the adapter fails, exceeds the page bound, duplicates an entry,
  returns wrong membership, repeats a cursor, or keeps returning continued
  pages beyond the expected membership
- **THEN** the scenario fails at the existing static list step with the existing
  closed failure kind and priority reachable through validated page values
- **AND** no scope, entry, cursor, or adapter error enters diagnostics.

#### Scenario: Evidence remains resource bounded

- **WHEN** the scenario accepts pages and continuation cursors
- **THEN** observed entries and cursor history remain bounded by the validated
  fixture cardinality and fixture-derived request limit
- **AND** no callback table, dynamic dispatch, executor, or new dependency is
  introduced.

#### Scenario: Touched function signal is removed semantically

- **WHEN** code-health evidence is refreshed from the protected implementation
- **THEN** no list-scenario function exceeds a configured function threshold
- **AND** the improvement comes from one cohesive evidence owner rather than
  forwarding helpers, generated code, moved tests, a waiver, or weaker policy.

