# ADR 0174: separate peer and PRISM DID method ownership

- **Status:** Accepted for implementation
- **Date:** 2026-10-26
- **Decision authority:** maintainer direction recorded on issue
  [#493](https://github.com/hyperledger-identus/sdk-rust/issues/493#issuecomment-6420422983)
- **Related:** ADRs 0110, 0162, 0164, 0169, and 0170; issues #5, #20,
  #420, #492, #521, #522, #523, #524, and NeoPRISM #321
- **Constraint impact:** material; changes the initial NeoPRISM disposition in
  ADR 0110 while preserving its reusable-module gates

## Context

SDK-TS revision `4bf86ebf69d5e96616a148e4c973f831f95fa38e`
implements peer DID numalgo 2 and PRISM long-form resolution plus create,
publish, update, and deactivate operation construction. Those behaviors are
useful compatibility evidence, but their historical Castor/Apollo DTOs and
errors are not canonical SDK-Rust architecture.

NeoPRISM revision `a2ddbc5a8a316841e2c8df3831e7904fef01c356`
contains the most complete Rust PRISM implementation: method syntax,
protobufs, operation validation, deterministic state reduction, VDR storage
operations, indexing, ledger ports, Cardano sources and sinks, persistence,
and node composition. ADR 0110 initially left all PRISM/Cardano behavior in
NeoPRISM. That was a safe extraction default, but it prevents SDK-Rust from
providing the reusable PRISM CRUD, indexing, and VDR foundation now required
by the product roadmap and keeps NeoPRISM larger than necessary.

Peer DID has a different lifecycle. The current DID Methods WG proposal at
revision `90850c3e700c4714afd7e91816c4530e4631ebb3` nominates only numalgo 4.
The broader DIF draft at revision
`c6a1f4c079cb4816ce88a1ea33991fcf17b3f4c8` and existing Identus products
still use numalgo 2. A single unversioned implementation would either abandon
existing consumers or silently make legacy behavior normative.

## Decision

### Generic DID contracts remain method-neutral

`identus-did` continues to own DID/DID URL/document values and resolver,
dereferencer, registrar, cache, and method-registry ports. It does not gain
peer, PRISM, Cardano, protobuf, key-generation, network, database, or node
policy.

### Peer DID is an independently versioned method crate

SDK-Rust will add `identus-did-peer` with two explicit profiles:

- `numalgo4` is the current normative profile for new integrations;
- `numalgo2-compat` is an opt-in Identus compatibility profile bound to the
  pinned DIF draft and SDK-TS/Mediator vectors.

The profiles share bounded multibase, multicodec, service, and DID-document
projection internals without accepting an ambiguous generic algorithm number.
Numalgos 0, 1, and 3 are out of the first release. Creation receives public
key material and services; secret generation/custody stays behind caller-owned
crypto and entropy ports.

The maintained `did-peer` 0.7.5 crate is a differential oracle, not a direct
dependency: it implements numalgos 0 and 2 but exposes Affinidi DID/key/secret types,
unconditional WASM dependencies, and fallible production paths containing
`unwrap`. `equs-vcx-did-peer` 0.1.0 is also an oracle: it covers numalgo 4 but
pulls a tightly coupled VCX DID/document/resolver/public-key family and has no
declared Rust version. Their Apache-2.0 fixtures may be adapted with exact
provenance.

### Portable PRISM capabilities move into SDK-Rust

SDK-Rust will own separately selectable crates, using capability names rather
than NeoPRISM repository boundaries:

1. `identus-prism-did` owns short/long DID syntax, pinned protobuf wire
   messages, operation builders and signatures, create/update/deactivate
   validation, DID-document projection, and deterministic state reduction.
   It depends inward on `identus-did`, `identus-crypto`, and their lower
   generic layers, never `identus-apollo`.
2. `identus-prism-vdr` owns PRISM storage-entry create/update/deactivate
   codecs, authorization/state rules, resolve metadata, bounded VDR-root
   traversal, and source/sink/store ports. It depends on `identus-prism-did`,
   is method-specific, and must not be presented as a universal VDR.
3. `identus-prism-indexer` owns deterministic ingestion and indexing of an
   ordered stream of PRISM objects, typed cursor and repository ports, atomic
   transaction batches, and replay/idempotency conformance. It depends on
   `identus-prism-vdr` and does not duplicate VDR traversal or open a network
   or database.
4. A small in-memory conformance adapter may live in SDK-Rust. Concrete
   Blockfrost, db-sync, Oura, Cardano wallet, embedded wallet, SQL, HTTP, CLI,
   process, deployment, and node-lifecycle adapters remain in NeoPRISM until a
   separately justified adapter issue proves independent demand and targets.

The package names intentionally differ from NeoPRISM's existing
`identus-did-prism*` packages so both paths can coexist during measured
adoption. The dependency chain is `identus-prism-indexer` ->
`identus-prism-vdr` -> `identus-prism-did` -> generic DID/crypto layers. No
generic SDK crate may depend on a PRISM crate. Each crate has its own feature,
target, benchmark, fuzz/property, release, and rollback evidence.

These are planned names, not current workspace or registry-ownership claims.
The SDK-ARCH-001 guard continues to deny the entire `identus-prism-*` family
until #521/#522 introduce actual crates together with exact dependency-graph
and transitive effect-cone enforcement. Those issues must also verify crates.io
availability and release authority before enabling publication; an unavailable
name requires a superseding naming decision before public activation.

The current generic 2,048-byte DID ceiling is not large enough for a PRISM
long-form identifier at the pinned method limits. Issue #524 therefore gates
the method implementation: it must derive finite generic DID/DID URL and
method-operation budgets, retain denial-of-service protection, and keep PRISM
encoding separate from the unrelated bounded crypto text codec.

### Adoption is SDK-first and measured

Implementation uses NeoPRISM and SDK-TS as immutable donors/oracles. SDK-Rust
lands and releases one capability before NeoPRISM pins it. NeoPRISM then runs
old and SDK-backed paths over the same preprod snapshot and compares ordered
operations, DID/VDR terminal state, errors, scan duration, CPU, and peak RAM.
Only behavior covered by that receipt may be removed downstream.

## Consequences

- Peer and PRISM no longer share a historical donor module or release cycle.
- SDK-Rust gains reusable PRISM CRUD/index/VDR behavior while generic DID
  contracts stay chain-neutral.
- NeoPRISM can shrink to Cardano adapters, persistence, service composition,
  and operations without losing an independently runnable node.
- Numalgo 2 compatibility remains available without being mislabeled as the
  current peer DID profile.
- Initial duplication is deliberate until immutable SDK releases and
  differential preprod evidence authorize downstream deletion.

## Alternatives rejected

- **Keep all PRISM code in NeoPRISM:** preserves unnecessary duplication and
  prevents the required reusable Rust foundation.
- **Move the NeoPRISM crates unchanged:** leaks Apollo naming, Cardano effects,
  persistence, and node lifecycle into SDK package boundaries.
- **One `identus-did-methods` crate:** couples unrelated standards, dependency
  cones, targets, and release cadence.
- **Adopt either peer DID crate wholesale:** neither matches both the current
  numalgo-4 direction and the bounded Identus numalgo-2 migration contract.
- **Treat SDK-TS behavior as normative:** contradicts ADR 0169 and freezes
  donor DTO/error choices.

## Verification and rollback

The planning change must pin every source, publish the ownership matrix,
update A2 and IDR-040, create focused implementation issues, and pass factory
and OpenSpec validation. Each implementation then needs official/donor
vectors, negative and resource-boundary tests, property/fuzz/differential
evidence, target and dependency receipts, and an independent review.

Before implementation, reverting this ADR restores ADR 0110's conservative
disposition. After publication, a superseding ADR and compatibility ledger are
required; no crate or downstream behavior is silently renamed or deleted.
