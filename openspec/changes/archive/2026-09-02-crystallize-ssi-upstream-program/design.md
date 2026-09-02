## Context

The source CSV was supplied as an uncommitted planning artifact in an Oxid
roadmap worktree. It mixes thirty SDK-owned `IDR-*` rows with seventeen
Midnight-owned `MID-*` rows. Its `state` column describes portfolio commitment
(`Foundation`, `Committed`, `Planned`, or `Conditional`), not implementation
progress. Treating that value as delivery status would overstate the current
SDK.

The inspected references are:

| Repository | Revision | Evidence role |
| --- | --- | --- |
| `hyperledger-identus/apollo` | `ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` | Kotlin compatibility behavior and vectors |
| `hyperledger-identus/neoprism` | `8becb225132efb1d9302b2c5f6ed4d87b84e8685` | Rust crypto, DID, PRISM and Cardano implementation |
| `MediaNoxLabs/midnight-identity` | `427f8571950c42967a18726cbcbefecc19ef8d79` | Rust generic DID/VC overlap and Midnight adapters |
| `input-output-hk/lace-id-portal` | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | Rust issuer/verifier and protocol evidence |
| `MediaNoxLabs/oxid` | `685f9670af4846d52697a4cfeb94779758ae1075` | Rust holder, wallet and port evidence |

## Goals / Non-Goals

**Goals:**

- preserve every SDK-owned outcome and acceptance statement;
- make actual state, issue ownership, predecessors and evidence sources
  machine-checkable;
- prevent generic, chain-family and product responsibilities from collapsing;
- enable one bounded component slice at a time;
- make a future Apollo retirement and NeoPRISM reduction possible without
  claiming either before compatible releases and downstream adoption exist.

**Non-Goals:**

- copying source or fixtures in this change;
- choosing final public crate names;
- publishing crates, changing `main`, or changing repository settings;
- changing Apollo, NeoPRISM, midnight-identity, Lace ID Portal or Oxid;
- treating a planning commitment as conformance or release evidence.

## Decisions

### 1. Keep one SDK-owned canonical CSV

The SDK copy contains only `IDR-*` rows. Midnight rows stay in the portfolio
source and are referenced as downstream dependencies rather than duplicated as
SDK deliverables. The canonical schema adds `delivery_status`, `issue`,
`owner_surface`, `predecessors`, `normative_source` and
`source_repositories` while preserving the original outcome fields.

### 2. Model commitment and delivery independently

`commitment` preserves the source values. `delivery_status` is limited to
`delivered`, `in_progress`, `specified`, `queued`, or `conditional`. A row can
be called `delivered` only when its acceptance evidence is immutable and linked
from its issue. Rows without a component issue point to #20 and cannot move
beyond `queued` or `conditional` until a child issue exists.

### 3. Classify evidence instead of bulk-porting repositories

Each source surface is classified as:

- `extract`: generic Rust behavior suitable for a focused SDK component;
- `adapt`: useful behavior that must be reshaped to meet the SDK boundary;
- `conformance-only`: vectors or behavior used without copying its API;
- `remain-downstream`: chain, application or product responsibility.

Every later port records the exact repository, revision, path, license and
transformation in its own issue and fixtures.

### 4. Keep capability ownership neutral

The CSV names owner surfaces such as `crypto`, `did-core` and `oid4vci`, not
unapproved public crate brands. In particular, Apollo is a cross-language
compatibility source while NeoPRISM `lib/apollo` is the Rust source. Whether a
future crate or compatibility family uses the Apollo brand remains a separate
public naming decision.

### 5. Validate with a dependency-light repository command

A small repository script parses the CSV with a standards-compliant parser and
fails on schema drift, duplicate or malformed IDs, invalid enumerations,
non-SDK ownership, missing issue links, or unknown source aliases. It is wired
into the existing structural check path so backlog drift fails locally and in
CI without building donor repositories.

## Risks / Trade-offs

- The source artifact is not committed, so its provenance cannot be claimed as
  an immutable Oxid revision. The SDK PR becomes the first durable copy and
  records that limitation truthfully.
- A single CSV cannot carry all component design detail. Issues and OpenSpec
  changes remain the authoritative slice contracts.
- Existing issues #5, #6 and #9 are broader than ideal. The backlog maps to
  them without authorizing monolithic implementation; child issues may narrow
  them before code moves.
- Source repositories will continue to evolve. Updating a pinned source SHA is
  an explicit reviewed backlog change, not automatic tracking.

## Migration Plan

Add and validate the canonical program, update current documentation, sync the
new capability specification and archive this change. Subsequent work selects
the highest-priority unblocked row, creates or narrows its issue and completes
a separate OpenSpec lifecycle. Rollback is a normal revert; no runtime or
consumer data is affected.
