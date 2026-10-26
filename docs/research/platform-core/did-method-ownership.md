# Peer and PRISM DID ownership research

**Issue:** [#493](https://github.com/hyperledger-identus/sdk-rust/issues/493)

**Date:** 2026-10-26
**Decision:** [ADR 0174](../../adr/0174-separate-peer-and-prism-did-method-ownership.md)

## Pinned evidence

| Source | Revision/version | Role |
| --- | --- | --- |
| SDK-TS | `4bf86ebf69d5e96616a148e4c973f831f95fa38e` | current Identus peer-numalgo-2 and PRISM consumer contract |
| NeoPRISM | `a2ddbc5a8a316841e2c8df3831e7904fef01c356` | Rust PRISM method, state reducer, indexer, VDR, and adapter donor |
| PRISM DID method specification | `fdd9007e23dc08d94d1a2b66b2dc26d1db499422` | normative PRISM syntax, operations, processing, and limits |
| DIF peer DID draft | `c6a1f4c079cb4816ce88a1ea33991fcf17b3f4c8` | legacy numalgo definitions and fixtures |
| DID Methods WG peer proposal | `90850c3e700c4714afd7e91816c4530e4631ebb3` | current proposal selecting numalgo 4 only |
| `did-peer` | 0.7.5, tag commit `3bca5b6358d05b07773ee6445834427b0411d083` | maintained numalgo-0/2 differential oracle |
| `equs-vcx-did-peer` | 0.1.0, tag commit `7c4b1f849027f18408dd42bcc044503db061b7ef` | numalgo-0/1/2/3/4 differential oracle |

All repositories and crates are Apache-2.0. Source reuse still requires
file-level provenance and transformation records.

## SDK-TS capability inventory

The pinned SDK-TS tree exposes two distinct method families under a historical
Castor namespace.

| Capability | Public/behavioral evidence | Normalized disposition |
| --- | --- | --- |
| peer DID create | Ed25519 authentication, X25519 agreement, DIDComm service compression, numalgo 2 | migrate through `identus-did-peer::numalgo2_compat` |
| peer DID resolve | SDK-TS uses a numalgo-2-only resolver with 32-byte key checks and JWK projection | compatibility vectors; Rust DTO/errors remain canonical |
| PRISM create | long-form DID from a protobuf create operation | implement in `identus-prism-did` |
| PRISM update | add/remove key, add/remove/update service, previous-operation hash | implement and compare with the PRISM spec and NeoPRISM |
| PRISM deactivate | signed deactivate operation | implement in `identus-prism-did` |
| PRISM publish | signed Atala object serialization | portable codec in SDK; transport remains an adapter |
| PRISM resolve | local long-form projection or HTTP proxy for short form | local method resolver in SDK; HTTP host adapter remains separate |
| key-path index | wallet derivation and persistence coordination | retain wallet/platform policy; reuse `identus-crypto` derivation |

The donor currently imports Apollo-labelled key wrappers and TypeScript domain
objects. These are source aliases only and are not target crate boundaries.

## Peer DID dependency assessment

| Candidate | Strengths | Blocking coupling/gaps | Decision |
| --- | --- | --- | --- |
| `did-peer` 0.7.5 | maintained, MSRV 1.88, Apache-2.0, numalgos 0 and 2, WASM packaging | Affinidi DID/key/secret family, unconditional WASM crates, no numalgo 4, production `unwrap` sites | numalgo-2 differential oracle and selective fixture adaptation |
| `equs-vcx-did-peer` 0.1.0 | includes numalgo 4 and older algorithms, typed modules | seven VCX-family crates, no declared MSRV, early 0.1 release, broad cone | numalgo-4 oracle and fixtures only |
| direct SDK implementation | exact `identus-did` types, narrow cone, explicit bounds/profiles | SDK owns maintenance and conformance | selected, using both oracles and official vectors |

The first crate supports only explicit `numalgo4` and `numalgo2-compat`
profiles. It rejects unknown algorithms before decoding, bounds DID/service/key
counts and decoded bytes, preserves redacted errors, and has no secret-store,
network, database, executor, or product-agent dependency.

## NeoPRISM decomposition

Authored nonblank Rust lines at the pinned revision provide scale, not target
package boundaries:

| Donor surface | Approximate size | Reusable invariant | Target |
| --- | ---: | --- | --- |
| `lib/did-prism` | 2,618 | PRISM wire operations and deterministic state transition | adapt into `identus-prism-did` |
| `lib/did-prism-indexer` | 4,153 | ordered ingestion, atomic indexing, and separable VDR ancestry | split into `identus-prism-vdr` and `identus-prism-indexer` |
| `lib/did-prism-ledger` | 131 | source/sink composition and in-memory proof | split ports to VDR/indexer; keep a conformance adapter |
| `lib/did-prism-submitter` | 721 | publication adapters | keep concrete Cardano effects in NeoPRISM initially |

### SDK-owned portable behavior

- short/long DID syntax and canonical identity;
- pinned protobuf wire compatibility;
- create/update/deactivate and storage-entry operation builders;
- signature and previous-operation authorization;
- operation validation and atomic state reduction;
- DID document and VDR result projection;
- ordered operation records, cursor/value objects, repository/source/sink
  ports, transaction-batch boundaries, replay/idempotency, and bounded root
  traversal;
- in-memory adapters used for conformance and consumer-shaped tests.

### NeoPRISM-owned effects

- Blockfrost, db-sync, Oura, Cardano wallet, and embedded-wallet clients;
- SQL schemas, migrations, repositories, and transaction management;
- HTTP/UI/CLI, process execution, configuration, secrets, deployment, and
  node supervision;
- mainnet/preprod selection and operational finality policy.

## Dependency direction

```text
identus-did-peer ─────── depends on ─────> identus-did + identus-crypto
identus-prism-indexer ── depends on ─────> identus-prism-vdr
identus-prism-vdr ────── depends on ─────> identus-prism-did
identus-prism-did ────── depends on ─────> identus-did + identus-crypto
identus-did + identus-crypto depend on ──> identus-core

NeoPRISM adapters and node ── consume ──> PRISM method/VDR/indexer crates
language bindings ────────── consume ──> selected public SDK crates
```

Every arrow is labelled from consumer to dependency. No generic crate depends
on a method crate, and SDK-Rust never depends on a consumer repository.

## Delivery and evidence plan

1. Finish the generic DID-domain/port prerequisite under #5.
2. Resolve #524's measured generic DID and PRISM long-form resource policy.
3. Deliver peer DID profiles in an isolated crate with official, SDK-TS, and
   two-oracle differential vectors.
4. Deliver PRISM method CRUD/state logic from the pinned spec and NeoPRISM,
   replacing `identus-apollo` with `identus-crypto`.
5. Deliver PRISM VDR and deterministic indexer ports with replay, rollback,
   ancestry-depth, batching, property, fuzz, differential, and benchmark
   evidence.
6. Rehearse SDK-TS through a thin adapter; do not reproduce TypeScript DTOs in
   Rust.
7. Pin released SDK crates in NeoPRISM and compare old/new preprod snapshots,
   including operations, terminal state, duration, CPU, and peak RAM.

No planning artifact claims target support, publication, consumer adoption, or
downstream deletion. Those claims require their own immutable receipts.
