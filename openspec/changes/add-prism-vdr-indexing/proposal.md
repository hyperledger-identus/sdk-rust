# Add bounded PRISM VDR and deterministic indexing

## Why

`identus-prism-did` now preserves exact PRISM storage-operation payloads, but
SDK-Rust does not yet expose typed storage-entry semantics or a reusable way to
index ordered ledger observations. NeoPRISM implements both concerns together
with Tokio, Cardano sources and SQL-oriented repositories. Reusing that package
boundary would make a portable SDK responsible for infrastructure effects.

## What changes

- Add `identus-prism-vdr`, depending on `identus-prism-did`, for bounded typed
  storage operations, exact predecessor relationships and finite root lookup.
- Add `identus-prism-indexer`, depending on `identus-prism-vdr`, for pure batch
  planning, ordered replay, cursor comparison and one-call atomic commits.
- Make scheduling, cancellation, source polling and retry policy caller-owned.
  SDK entry points process one explicitly bounded batch and never spawn tasks.
- Provide in-memory conformance adapters and donor/specification vectors.
- Enforce the dependency direction indexer -> VDR -> PRISM DID -> generic DID.

## Capabilities

### Modified capabilities

- `prism-vdr-indexing`: replace the architectural placeholder with executable,
  bounded public contracts and evidence.

## Non-goals

No Cardano client, Blockfrost, db-sync, Oura, cardano-wallet, SQL schema, HTTP,
CLI, Tokio runtime, daemon lifecycle, universal VDR abstraction, downstream
mutation, or crates.io publication.

## Delivery

Issue #522 owns one cohesive extraction after #521. Public activation remains
independent per crate. NeoPRISM adoption and mainnet/preprod comparison are a
separate downstream slice after this SDK contract is stable.
