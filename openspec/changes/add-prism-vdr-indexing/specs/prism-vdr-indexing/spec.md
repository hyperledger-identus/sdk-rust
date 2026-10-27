# prism-vdr-indexing Delta Specification

## MODIFIED Requirements

### Requirement: PRISM VDR and indexing cores are effect-isolated

The SDK SHALL place PRISM storage-entry/state rules and bounded root traversal
in `identus-prism-vdr`, which depends on `identus-prism-did`, and SHALL place
deterministic ordered ingestion, batching, replay, and cursor semantics in
`identus-prism-indexer`, which depends on the VDR crate. These crates MAY
define bounded source, sink, store, transaction-batch, and repository ports
but SHALL NOT implement Cardano clients, SQL, HTTP, CLI, deployment, or node
orchestration.

#### Scenario: an ordered batch is indexed

- **WHEN** a bounded source supplies a complete ordered batch
- **THEN** the indexer validates and commits deterministic state atomically
  through caller-provided ports with stable replay and idempotency behavior

#### Scenario: a concrete effect is required

- **WHEN** indexing requires Blockfrost, db-sync, Oura, a wallet, SQL, or a
  service runtime
- **THEN** that implementation remains downstream and is not added to the SDK
  dependency cone

#### Scenario: crate dependency direction is validated

- **WHEN** the method, VDR, and indexer crates enter the workspace
- **THEN** the existing family deny is replaced atomically by a machine guard
  that permits only indexer-to-VDR, VDR-to-method, and method-to-generic inward
  edges, rejects every generic-to-method edge, and rejects prohibited effects
  throughout each transitive dependency cone

#### Scenario: method packages are planned but absent

- **WHEN** the method, VDR, and indexer crates have not entered the workspace
- **THEN** the dependency guard rejects the complete `identus-prism-*` family

### Requirement: ancestry and replay work are bounded

Root traversal SHALL apply finite limits to ancestry depth, lookup work,
records and signed bytes per complete transaction, and retained batch-local
state. The SDK SHALL own no channel or retry loop. Cycles, missing parents,
rollback, depth exhaustion, duplicate replay and cursor conflicts SHALL produce
deterministic redacted outcomes.

#### Scenario: ancestry is cyclic or too deep

- **WHEN** VDR-root traversal encounters a cycle or exceeds its configured
  depth
- **THEN** processing terminates without partial state commitment and returns
  the corresponding stable bounded-work outcome

## ADDED Requirements

### Requirement: complete batches plan before one atomic commit

The indexer SHALL validate and plan every record in one complete bounded batch
before invoking a caller-provided atomic commit exactly once. The commit SHALL
compare the expected cursor and make the complete ordered mutation set plus the
next cursor visible atomically. Batch capacity SHALL be the SDK backpressure
boundary; the SDK SHALL NOT own an internal channel.

A batch SHALL represent one complete transaction and carry a bounded opaque
transaction identity, a declared operation count and records with the same
identity and contiguous zero-based indices. The source adapter SHALL own
authoritative boundary extraction. The SDK SHALL reject inconsistent boundaries
and any complete transaction above the profile rather than split it.

#### Scenario: batch validation fails late

- **WHEN** any record is malformed, out of order, orphaned, cyclic, too deep or
  outside a finite limit
- **THEN** no commit call occurs and no partial state becomes visible

#### Scenario: a transaction is split or incomplete

- **WHEN** identity, declared count or contiguous indices do not describe one
  complete transaction
- **THEN** construction fails before parsing or committing any operation

#### Scenario: the same committed batch is replayed

- **WHEN** expected cursor, next cursor and deterministic batch identity match
  an already committed batch
- **THEN** the commit reports exact replay without duplicating mutations

#### Scenario: a conflicting batch is replayed

- **WHEN** the expected cursor is stale or the batch identity differs from the
  already committed transition
- **THEN** the commit reports a cursor conflict and the SDK does not retry it

### Requirement: cancellation and scheduling remain caller-owned

The SDK SHALL finish each core operation within its declared finite work bound
and SHALL NOT spawn tasks, sleep, poll a source or install a runtime. Callers
MAY compose the synchronous core with an async source or store and SHALL own
cancellation and retry policy between bounded SDK calls.

#### Scenario: a caller cancels ingestion

- **WHEN** cancellation is requested before a new batch or during adapter I/O
- **THEN** the caller stops or cancels that work without an SDK background task
  retaining a channel, cursor or partial mutation plan

#### Scenario: commit acknowledgement is ambiguous

- **WHEN** an adapter cannot determine whether its atomic commit completed
- **THEN** the caller may replay the same bounded transition and use exact
  replay semantics to distinguish applied state from a conflict

### Requirement: storage mutations are authorized at their ledger position

Before planning a storage mutation, the VDR capability SHALL derive its owner
DID, receive that DID's state at the operation's exact ordered position, require
the state to be active, locate the active secp256k1 `KeyUsage::Vdr` key named by
the signed envelope and verify the signature over exact nested operation bytes.
The indexer SHALL reduce earlier DID operations in the same transaction into
batch-local state before authorizing later storage operations.

#### Scenario: a forged or unauthorized storage operation is observed

- **WHEN** the owner is deactivated, the named key is absent, retired or has a
  different purpose, or signature verification fails
- **THEN** the complete transaction is rejected before the atomic commit port
  is invoked

#### Scenario: a DID update precedes storage in the same transaction

- **WHEN** a valid ordered DID operation changes the VDR-key set before a later
  storage operation
- **THEN** authorization uses the updated batch-local DID state rather than the
  pre-transaction snapshot
