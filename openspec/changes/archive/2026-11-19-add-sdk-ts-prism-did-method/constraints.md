# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/610
Constraint blockers: none

## Existing entries affected

- `SDK-ARCH-001` remains effective: portable Rust owns PRISM semantics and
  language/target adapters depend inward.
- `SDK-SEC-003` remains effective: all new structured, encoded, and host-returned
  input is bounded before semantic use.
- `SDK-COMPAT-002`, `SDK-COMPAT-004`, and `SDK-COMPAT-005` remain effective for
  Rust 1.99.1, strict language mapping, and target evidence.
- `SDK-LIM-002` remains effective: the TypeScript package is private,
  unpublished, and not a stable binding commitment.
- `SDK-LIM-003` remains effective: browser WASM and native targets are not
  runtime-certified.
- `SDK-LIM-009` remains effective: selected target failures do not erase
  completed Rust-core evidence, while terminal aggregation fails closed.
- ADRs 0163, 0174 through 0176, 0199, and 0201, the canonical PRISM method
  specification, and TS1 SC-01 through SC-09 remain controlling.

## Introduced or changed constraints

`identus-wasm-did` gains an inward dependency on the exact in-tree
`identus-prism-did` package. Generated and TypeScript surfaces use capability
names and explicit canonical/long-form/history operations. No raw generated
owner, chain client, ledger coordinate, database, transport, runtime, or
historical SDK component name crosses the reviewed public boundary.

ADR 0201 narrows ADR 0174's generic-crate prohibition for this exact outer
target-assembly edge. The conformance rulebook accepts only
`identus-wasm-did -> identus-prism-did`; generic crates, native adapters, VDR,
indexer, and chain-family dependencies remain denied.

Long-form construction accepts caller-owned public key, service, and context
material only. Rust owns operation encoding, identifier derivation, state
reduction, signature verification, and DID Document projection. TypeScript
owns closed structural validation and host-effect orchestration only.

Canonical resolution uses an injected effect returning a finite ordered list
of exact signed operation encodings. The effect owns observation, transport,
cache, persistence, retry, authentication, ledger finality, and time policy.
The SDK validates and reduces the returned portable history; it never performs
ambient network or Cardano access and never accepts replacement DID authority.

## Introduced or changed limitations

The train is a private, opt-in, non-default Browser/Node/React-web facade.
React Native requires a separately qualified native package. No support,
performance budget, certification, publication, stable API, external SDK-TS
activation, or production claim follows from local target success.

The facade does not create private keys, sign operations, submit transactions,
interpret Cardano metadata or VDR storage payloads, index a ledger, persist
state, compose a node, or choose trust/finality/retry policy. Public test key
material and signed fixture bytes are evidence, not custody interfaces.

## Consumer and product impact

Existing package APIs remain compatible and the external SDK-TS route remains
unchanged. The additive provider can be selected explicitly before WASM
initialization. NeoPRISM keeps Cardano, persistence, indexing, VDR, and node
ownership; its merged adoption evidence is read-only input.

## Activation and rollback

Repository-local activation requires issue-bound planning, exact donor and
source records, focused and complete local evidence, distinct review,
signed+DCO commits, green required CI, and protected merge. A selected provider
or history effect failure is terminal and cannot invoke a legacy parser,
resolver, network path, or alternate provider automatically.

Rollback selects an injected legacy/generic provider at a new
pre-initialization boundary or removes the unreleased PRISM capability. It does
not alter canonical Rust semantics, persisted consumer state, NeoPRISM, or the
external SDK-TS package.

## Evidence

Issue #610; SDK-Rust base `0482b7066a8806e588cd9494e903f5aee1e4c324`;
`identus-prism-did`; the canonical Preprod/vector packet; SDK-TS revision
`4bf86ebf69d5e96616a148e4c973f831f95fa38e`; NeoPRISM merged revisions
`41b7f1991a2665c3cb9d0d994d861f9a32f07727` and
`83df4e4fba0070e946ed40b190d8d92c401614c9`; the language mapping,
quality, consumer-change, and TS1 registries; and exact target receipts are the
atomic evidence owners.
