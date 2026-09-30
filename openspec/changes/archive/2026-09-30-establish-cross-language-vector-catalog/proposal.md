# Establish the cross-language vector catalog

## Why

SDK-Rust has strong crate-local tests and several pinned donor-SDK inventories,
but it does not yet have a language-neutral, provenance-preserving way to share
one behavioral case across Rust and downstream adapters. Copying fixtures would
lose authority, licensing, revision, and mutation intent; treating repeated
SDK-TS/Swift/KMP cases as normative would let historical consumers dictate the
Rust contract.

Issue #420 therefore establishes the first A1 evidence contract: an offline,
versioned catalog of immutable vector packets with explicit authority and exact
test selectors. A bounded DID/DID URL packet proves the contract without adding
a DID method, consumer adapter, protocol engine, or runtime dependency.

## What changes

- Add a closed TOML catalog for sources, packets, vectors, provenance,
  authority, licensing, redistribution, boundaries, selectors, and lifecycle.
- Add a language-neutral DID/DID URL JSON packet with positive, negative,
  boundary, redaction, and pinned consumer-regression cases.
- Add an offline validator and mutation tests that fail closed on unknown or
  inconsistent metadata, altered payloads, unsafe paths, and dangling records.
- Add a Rust conformance test that loads the same packet and proves current
  `identus-did` parsing, limits, stable error codes, and redaction behavior.
- Integrate the contract and its tests into the repository factory gate.

## Capabilities

### Added capabilities

- `cross-language-vector-catalog`: store reusable compatibility evidence with
  immutable provenance and deterministic local validation.

### Modified capabilities

None. Existing crate-local and domain-specific vector inventories remain their
authoritative owners and may be referenced rather than copied.

## Non-goals

This change does not implement SDK-TS, SDK-Swift, SDK-KMP, peer DID, Prism DID,
DID documents, resolution, credentials, DIDComm, language bindings, or network
behavior. It does not claim language adoption or general DID conformance. It
does not retrieve donor repositories during CI.

## Delivery

Issue #420 owns this change under parent #504 and milestone A1. Its stable IDs
feed #501 and #422. Issue #505 may proceed independently. Downstream SDK-TS
canary #492 remains blocked until all A1 contracts close.
