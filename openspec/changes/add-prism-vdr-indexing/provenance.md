# File and fixture provenance

## Donor snapshot

- Repository: `https://github.com/hyperledger-identus/neoprism.git`
- Revision: `a2ddbc5a8a316841e2c8df3831e7904fef01c356`
- License: Apache-2.0 (`LICENSE` at the pinned revision)
- Disposition: adapt behavior and tests; redesign package and effect boundaries.

## Candidate source inventory

| Donor path | SHA-256 | Disposition |
| --- | --- | --- |
| `lib/did-prism-indexer/src/indexing.rs` | `99c4347d5f338731a7dd7259c937b7cbe3e1b79d1591232035040fad33010569` | adapt classification, transaction grouping and local-before-store root lookup; reject silent ignore and partial continuation |
| `lib/did-prism-indexer/src/repo.rs` | `457879c0e868648a3c1518e5e8d03fdb5241829d5c86fa3574daa80442855006` | reference only; replace UUID/Cardano/async repository DTOs with SDK-owned ports |
| `lib/did-prism-indexer/src/dlt/common.rs` | `0dfda6171a617b17058829ca16dd66ba00336963b4c8cf36d40c0da29c320f9c` | reference ordering/cursor evidence; exclude Tokio channels and Cardano metadata |
| `lib/did-prism-indexer/src/dlt/mod.rs` | `e402a926b147c48ac7cb8b9066a1789eade81d78b82eef83b6f64acbd384c4f8` | reference only; exclude concrete sources |
| `lib/did-prism-ledger/src/lib.rs` | `097c458096eb332adc18a5c3b373f5cd13b567f14ff49808a990a13f7b8c39b3` | reference in-memory behavior; redesign without task spawning, time or synthetic transactions |
| `lib/did-prism/proto/prism-storage.proto` | `958e1c8d13caa06b9a27539a1b37995b96c55f7d3c4f5d7d8db6de5b20a1053e` | already pinned in `identus-prism-did`; adapt private payload decoding |

The digests were computed from `git show <revision>:<path>` before the
implementation commit. History for `indexing.rs` includes the original VDR
implementation and later transaction-ID, SQLite and resolver work; those later
effect-specific additions are not evidence of SDK ownership.

## Excluded donor areas

Blockfrost, db-sync, Oura, Pallas, submitter, node-storage, SQL migrations,
HTTP/OpenAPI, CLI, tracing and node lifecycle remain downstream. No source from
those areas is copied into production SDK crates.
