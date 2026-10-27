# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/493
Constraint blockers: none

ADR 0174 and its accepted issue decision authorize the separate method, VDR
and indexer ownership. Issue #522 supplies the concrete bounded contract.

## Existing entries affected

SDK-ARCH-001, SDK-ARCH-002 and SDK-ARCH-003 require inward dependencies,
effect isolation and reusable crate ownership. SDK-SEC-003 requires finite
untrusted-input work. SDK-LIM-005 through SDK-LIM-007 keep chain clients,
persistence and service runtimes outside the SDK core. Existing generic DID,
crypto, error, target, unsafe-code and release constraints remain unchanged.

## Introduced or changed constraints

- `identus-prism-vdr` depends on `identus-prism-did`; only the indexer may
  depend on the VDR crate. Generic crates cannot depend on either.
- Every public operation, object, batch, ancestry walk and retained in-memory
  collection has a finite public limit checked before proportional allocation.
- The indexer processes one complete caller-supplied batch. It validates and
  plans the entire batch before invoking one atomic commit port call.
- Source polling, cancellation, scheduling, backpressure and retry are caller
  responsibilities. The SDK neither spawns tasks nor retains an unbounded
  channel. Batch capacity is the SDK backpressure boundary.
- A child root lookup checks batch-local predecessors before an immutable store
  snapshot and terminates on missing parents, cycles or depth exhaustion.
- Cursor advance and mutations commit atomically using an expected-cursor
  compare-and-set contract. Replay of the same committed batch is idempotent;
  conflicting replay fails closed.
- Public errors and `Debug` output omit payload bytes, DIDs, transaction IDs,
  signatures, keys, cursors supplied by an adapter and repository diagnostics.
- The implementation uses no unsafe code, native dependency, network client,
  database driver or async executor.

## Introduced or changed limitations

- The SDK defines effect-neutral storage-entry and indexing contracts, not a
  universal VDR model and not a Cardano ingestion implementation.
- Cancellation is observed between bounded calls. A caller that requires
  interruption inside I/O implements that policy in its source/store adapter.
- A batch is an atomic visibility unit. Oversized transactions are rejected,
  not split into partially visible commits.
- Wall-clock timestamps and chain-finality policy are adapter metadata and do
  not participate in deterministic SDK planning.
- Crate names were absent from `cargo search` on 2026-10-27, but publication
  authority is not inferred and must be reverified by the release workflow.

## Consumer and product impact

Rust adopters gain portable VDR and deterministic indexer primitives. Existing
crates do not change behavior. NeoPRISM can later adapt its Cardano and SQL
edges to the ports, while SDK-TS/Swift/KMP adapters remain language-owned and
cannot dictate Rust DTO or error shapes.

## Activation and rollback

Activation requires the exact issue #522 preflight, complete ADR/provenance and
quality declarations, target and dependency evidence, independent CLI review,
green protected CI and an issue-linked PR. Claude is preferred when available;
if unavailable or quota-limited, Codex CLI is used immediately. The reviewer,
command and exact commit are recorded. Before publication, rollback removes the
two additive crates and their registries. Published breaking changes require a
superseding compatibility decision.

## Evidence

Evidence authorities are issue #522, ADR 0174, the new concrete API ADR, the
pinned NeoPRISM donor revision, PRISM protobuf sources already pinned by
`identus-prism-did`, OpenSpec validation, conformance/fuzz/property tests,
dependency and target receipts, independent review and protected CI.
