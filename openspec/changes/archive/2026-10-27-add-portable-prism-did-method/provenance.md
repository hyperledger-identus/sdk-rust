# File and fixture provenance

## Donor snapshot

- Repository: `https://github.com/hyperledger-identus/neoprism.git`
- Revision: `a2ddbc5a8a316841e2c8df3831e7904fef01c356`
- Revision date: `2026-10-20T11:54:24+07:00`
- License: Apache-2.0 (`LICENSE` at the pinned revision)
- Disposition: adapt behavior and tests; do not copy package boundaries or
  generated output without retaining notices and source attribution.

## Candidate source inventory

| Donor path | SHA-256 | Disposition |
| --- | --- | --- |
| `lib/did-prism/proto/prism.proto` | `412163eef867771e78418e25dd8a761c7a8ef2c390b190e82b413d0a0c7b8fdd` | adapt DID envelope tags; keep VDR tags opaque |
| `lib/did-prism/proto/prism-ssi.proto` | `dd365637988fdf61ec80038ba6f6ab1dfafb6556288fa368e2d0607fdbb78609` | pin and generate private prost messages |
| `lib/did-prism/proto/prism-storage.proto` | `958e1c8d13caa06b9a27539a1b37995b96c55f7d3c4f5d7d8db6de5b20a1053e` | pin imported VDR tags; keep payload semantics out of #521 |
| `lib/did-prism/proto/prism-version.proto` | `481ef353a57345a06afca7928de94cd5fefa522d46c1000e66a4bceb0de31f4d` | pin imported protocol-version tag; keep payload opaque |
| `lib/did-prism/src/did/mod.rs` | `d5c9d97bfbe65155597c4af9480a8314d6ddc8580c3d20500c6737c3866494ec` | adapt short/long syntax; replace donor DID types |
| `lib/did-prism/src/did/operation/ssi.rs` | `f7925bc376b7d74d274d468f45106a1ce9ffe7f6b5c606a0256065f5fc721857` | adapt operation rules; correct recorded deviations |
| `lib/did-prism/src/did/did_doc.rs` | `21f7c30f0a89de35c86cd41c1b05204c948b700ecccfc5d2ddad40b53caefda1` | adapt projection to `identus-did` |
| `lib/did-prism/src/protocol/mod.rs` | `fee7b958d19c9e9b82f537f0c8e367b899a527350362b1a2bb9114747fccbc28` | adapt DID-only state; exclude storage |
| `lib/did-prism/src/protocol/v1.rs` | `3d7c1b9218f71ca05d2a7987b524826a3b9de81f7a8edade9e91f3637d6f5b61` | adapt ordered transition behavior; fix active-count bug |
| `lib/did-prism/src/protocol/resolver.rs` | `743d7afd7847a30fb0a8fd7c9fb9ffb0bc03893b6dd9b03bcb727a9678265b42` | adapt local long-form behavior only |

## Candidate test inventory

| Donor path | SHA-256 | Disposition |
| --- | --- | --- |
| `lib/did-prism/tests/did_mod.rs` | `77da04df05838e030bbaaab156566aac4e53867ad074aa7cb60c9613935cd6b4` | adapt syntax/hash/round-trip cases |
| `lib/did-prism/tests/ssi_operation_parse.rs` | `271659ad12d7cd7e48b587a97ed1be25e35409412923831c5b12f84f08e6a885` | adapt parser cases and add strict negatives |
| `lib/did-prism/tests/protocol_processing.rs` | `096a0ea487d39cb3593d2b3cb975e38bd96142ce844a5007f4fec78cfd48672f` | adapt state traces; add tombstone/rollback/order vectors |
| `lib/did-prism/tests/did_doc.rs` | `ce8ce6e867be3da41f402e06818ce186472acb4d9e1be7c62993d4f4eda79d0c` | adapt projection cases and deterministic order |

## Excluded donor areas

`did/operation/storage.rs`, storage state, `dlt`, Cardano sources, node APIs,
database, HTTP, CLI, tracing and OpenAPI files are reference-only or excluded.
They must not enter the #521 crate dependency graph.

## Historical Preprod fixture

`docs/conformance/fixtures/did-prism/preprod-2026-09-10.json` preserves the two
public Preprod create operations from NeoPRISM issue #325 and PR #327's
synchronized baseline/candidate run. The run compared published image 0.14.2
with SDK-Rust adoption revision `757531e2d7e26ab5a0c950b619c5f5cdfb0a0735`
and reported zero operation differences.

The original ignored local CSV has SHA-256
`04a43c3ff8afde4218011e58bd02b246e4d9ab6194e356467dd2319aba5015f9`.
The checked-in packet carries the network, cursor, transaction hash, slot,
block, ordering, exact signed bytes, operation ID and DID suffix. Each operation
ID equals SHA-256 of the signed envelope. Implementation tests must additionally
prove that the DID suffix equals SHA-256 of the exact nested operation and that
the embedded master key verifies the envelope signature.

This packet is a small historical compatibility canary, not representative
volume or a substitute for #522's wider indexer differential suite.
