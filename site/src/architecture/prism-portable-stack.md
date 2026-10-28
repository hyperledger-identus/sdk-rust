# Portable PRISM stack

SDK-Rust separates PRISM method semantics from the effects required to operate
a Cardano indexer or hosted service. The result is three cohesive, independently
selectable source packages:

![Portable PRISM stack](../diagrams/prism-portable-stack.svg)

The solid arrows are Cargo dependency direction. The dashed boxes are ports or
responsibilities that an adopter owns. These three PRISM crates do not open a
network connection, start a task, sleep, retry, write a database, submit a
transaction, or own private-key custody.

## Responsibilities

| Crate | Owns | Does not own |
| --- | --- | --- |
| [`identus-prism-did`](../crates/identus-prism-did.md) | Bounded short/long identifiers, exact signed operation bytes, create/update/deactivate construction, validation, deterministic state reduction, DID document projection | Ledger lookup/submission, persistence, custody, HTTP, runtime |
| [`identus-prism-vdr`](../crates/identus-prism-vdr.md) | Storage-entry wire semantics, VDR-key authorization, current-entry state, bounded ancestry traversal | Cardano sources, durable repository, scheduling, retry |
| [`identus-prism-indexer`](../crates/identus-prism-indexer.md) | Complete ordered transaction batches, transition digests, replay/idempotency, deterministic planning, one atomic commit contract | Source polling, channels, database transactions, process lifecycle |

Generic crates never depend on these method-specific packages. The machine
dependency guard fixes the allowed internal SDK edges to `indexer -> VDR ->
PRISM DID -> generic DID and crypto` and rejects prohibited donor/chain package
families. The current quality receipt separately proves with `cargo tree` that
the three resolved cones contain no async executor, network client, database
driver, Cardano client, or native system library.

## Executable flow

The repository retains one readable integration scenario that:

1. constructs and signs a DID create operation with caller-owned test keys;
2. creates a VDR entry owned by that DID in the same ordered transaction;
3. plans and commits both mutations atomically through an in-memory test port;
4. projects the committed DID state into a DID Core document;
5. resolves VDR root/head metadata through the lookup port and verifies the
   payload retained by the caller-owned test store;
6. recognizes an exact transaction replay without a second commit; and
7. applies and resolves a second VDR update transaction.

Run it from the repository root:

```bash
cargo test -p identus-prism-indexer --test indexing \
  portable_stack_e2e_creates_resolves_updates_and_replays
```

The in-memory adapter is test evidence, not a supported production store. A
real source must extract one complete transaction boundary; a real store must
repeat identity/digest and expected-cursor checks inside one atomic commit.

## Evidence and current adoption state

- [ADR 0174](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/adr/0174-separate-peer-and-prism-did-method-ownership.md) defines method ownership.
- [ADR 0176](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/adr/0176-allow-only-in-tree-portable-prism-method-package.md) defines the method package boundary.
- [ADR 0177](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/adr/0177-use-synchronous-atomic-prism-indexing-core.md) defines synchronous atomic indexing.
- [Quality baseline](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/architecture/prism-vdr-indexer-quality-baseline.md) records target, benchmark, property, fuzz, and slow-lane receipts.
- [NeoPRISM fixed-boundary summary](https://github.com/hyperledger-identus/neoprism/pull/327#issuecomment-6441647413) pins candidate revision `ffaeafe9af35dc420c446b5f99d708cd2f9fe9db`, Preprod slot `10,723,321`, both independent passing runs, and canonical manifest SHA-256 `1b4fd7cbaaf46fcb23e3c9878facb8a28b1b861374177b866fbb6d88bd39419f`.
- [Run 1](https://github.com/hyperledger-identus/neoprism/pull/327#issuecomment-6441647927) and [run 2](https://github.com/hyperledger-identus/neoprism/pull/327#issuecomment-6441648520) retain the exact image identities, drain receipts, semantic comparison, and diagnostic QoS results.

NeoPRISM PR #324 currently adopts generic SDK cryptography and DID components;
it does **not** yet replace its PRISM method, Cardano, persistence, or runtime
implementation with these three crates. Its parity evidence is an immutable
compatibility baseline for a later, separately reviewed adoption—not proof that
the extracted PRISM crates are already deployed.

## Status and limitations

The packages remain experimental source components at version `0.0.0` with
`publish = false`. Native, WASM, iOS, and Android checks are compile evidence,
not browser/device packaging or production-runtime support. The SDK does not
claim Cardano connectivity, durable storage, service availability, QoS
thresholds, certification, or stable SemVer for this stack.
