# `identus-prism-did`

Portable, chain-neutral PRISM DID method semantics.

## Responsibility

- parse bounded short-form and long-form PRISM identifiers;
- preserve exact inbound signed operation bytes for hashing and verification;
- construct and validate create, update, and deactivate operations;
- reduce operations into deterministic method state atomically; and
- project active state into generic `identus-did` documents.

Typed constructors validate keys, services, contexts, and ordered actions
before deterministic protobuf encoding. Callers sign
`RawPrismOperation::as_bytes()` through their own custody implementation and
assemble a `SignedPrismOperation`; this crate never owns entropy or private
keys.

## Dependency direction

The crate depends inward on `identus-did`, narrowly featured
`identus-crypto`, `identus-core`, and private `prost` wire types. Generated
protobuf values are not public API. VDR and indexer crates depend on this crate;
the generic layers do not depend back on PRISM.

## Limits and non-goals

The supported profile bounds operation/envelope bytes, identifiers, keys,
services, contexts, projection amplification, and update work. It does not
perform ledger lookup/submission, persistence, HTTP, scheduling, custody, or
product policy.

- [Source and crate README](https://github.com/hyperledger-identus/sdk-rust/tree/develop/crates/prism-did)
- [Portable stack walkthrough](../architecture/prism-portable-stack.md)
- [Long-form bounds decision](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/adr/0175-bound-generic-did-for-prism-long-form-state.md)
