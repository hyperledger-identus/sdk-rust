# ADR 0069: use IOTA Identity as a DID and VC oracle

- **Status:** Accepted
- **Date:** 2026-09-08
- **Issue:** [#175](https://github.com/hyperledger-identus/sdk-rust/issues/175)
- **Decision authority:** [discussion #174](https://github.com/hyperledger-identus/sdk-rust/discussions/174), ADR 0008, and ADR 0061
- **Assessed source:** [`iotaledger/identity@7dd52708`](https://github.com/iotaledger/identity/tree/7dd527087b96bf26ea97d486225eff8278c9a89c), Apache-2.0
- **Research:** [repository portfolio](../research/rust-library-reuse/repository-portfolio.md)

## Context

IOTA Identity provides mature generic DID, document, credential, JOSE,
resolution, storage, status, and SD-JWT components with host and WASM evidence.
Its current development line is beta and combines method-neutral components
with IOTA-specific networking and exact workspace/Git coupling. Its types would
create a second public DID/VC model beside the Identus facade.

## Decision

Classify the repository as `oracle`. Use its behavior and attributed fixtures
for DID, document, credential, JOSE, resolution, and parser differential tests.
Do not expose or depend on IOTA framework types in production SDK surfaces.

A narrow component may be proposed only when it provides a unique capability
not supplied by a smaller engine and preserves exact Identus public states,
errors, limits, targets, and chain-neutral dependency direction.

## Consequences

The SDK gains a strong independent implementation without importing IOTA chain
or framework assumptions. ADR 0008 continues to govern the current DID parser.

## Reconsideration and rollback

Reconsider one named crate after stable publication and complete cone/target/
semantic parity evidence. Rollback is a documentation revert.
