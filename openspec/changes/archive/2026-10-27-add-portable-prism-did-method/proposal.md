# Add the portable PRISM DID method

## Why

SDK-Rust owns generic DID values and reusable cryptography but cannot yet parse,
construct, verify or resolve `did:prism` operations. NeoPRISM has working Rust
behavior, while SDK-TS carries current consumer evidence, but neither repository
is the correct reusable crate boundary. Copying either package graph would also
retain Apollo, Cardano, node, storage, tracing and generated-protobuf coupling.

## What changes

- Add `identus-prism-did` for bounded short/long-form syntax, exact protobuf
  operation bytes, typed create/update/deactivate operations, signature checks,
  deterministic state reduction and DID document projection.
- Use `identus-crypto` and `identus-did`; do not depend on Apollo or a node,
  ledger, database, HTTP client, executor or configuration framework.
- Keep checked-in private `prost` messages and a deterministic maintainer-only
  regeneration check. Generated messages are not public SDK types and consumer
  builds do not invoke `protoc`.
- Preserve exact inbound operation bytes for signatures and hashes. Reject
  duplicate security-relevant singular fields before typed decoding.
- Provide a local long-form resolver and a `DidMethodBinding`. Published
  short-form history lookup remains a port supplied by issue #522.

## Capabilities

### New capabilities

- `did-prism`: portable method syntax, operations, state and projection.

## Non-goals

No Cardano scanning, transaction metadata, VDR payload semantics, SQL, HTTP,
CLI, hosted registrar/publication workflow, SDK-TS change, or NeoPRISM removal.

## Delivery

Issue #521 owns the cohesive extraction. Issue #524 must merge first so the
generic DID layer admits the finite long-form profile. Release the crate
independently before removing downstream duplicate code.
