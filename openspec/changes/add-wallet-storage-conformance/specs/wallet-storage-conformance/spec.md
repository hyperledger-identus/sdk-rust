## ADDED Requirements

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
