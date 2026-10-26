# Design

## Crate graph

`identus-did-peer` depends on the generic DID and crypto layers. The PRISM
chain is `identus-prism-indexer` -> `identus-prism-vdr` ->
`identus-prism-did` -> generic DID/crypto layers. VDR owns bounded root
traversal; the indexer owns ordered ingestion, batching, replay, and cursor
semantics without duplicating VDR rules. NeoPRISM consumes those public crates
through adapters. No generic layer points upward to a method crate. The SDK
names intentionally do not collide with NeoPRISM's `identus-did-prism*`
packages, allowing old and new paths in one workspace during migration.

## Peer profiles

The public API names the selected profile in types or constructors. Numalgo 4
is the current-profile surface. Numalgo 2 is feature-gated compatibility for
existing Identus consumers. Shared codecs are private and bounded. Unknown or
disabled algorithms fail before decode/allocation work.

## PRISM method core

The method crate owns deterministic values and transformations: syntax,
protobuf wire compatibility, operation construction/signing/validation,
previous-operation lineage, atomic state transition, long-form resolution,
and DID-document projection. Network, persistence, clocks, and executor
behavior enter only through higher-layer ports.

## VDR and indexer

VDR is explicitly PRISM-specific. It owns storage-entry operation and state
semantics plus iterative bounded root traversal; cycles, missing parents, and
depth exhaustion have stable outcomes. The indexer consumes the VDR layer and
an ordered bounded stream, then commits complete transaction batches through
repository ports. It exposes replay/cursor and rollback semantics without
knowing Blockfrost, db-sync, Oura, SQL, or a node runtime.

## Migration

SDK crates land first. A TypeScript adapter and NeoPRISM adapter then bind
their existing DTOs/effects. NeoPRISM old/new paths process the same preprod
snapshot and compare ordered operations, terminal DID/VDR state, errors,
duration, CPU, and peak RAM. Only the covered implementation may be removed.
