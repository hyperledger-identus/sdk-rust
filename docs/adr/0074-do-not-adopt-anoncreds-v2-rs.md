# ADR 0074: do not adopt anoncreds-v2-rs

- **Status:** Accepted
- **Date:** 2026-09-08
- **Issue:** [#175](https://github.com/hyperledger-identus/sdk-rust/issues/175)
- **Decision authority:** [discussion #174](https://github.com/hyperledger-identus/sdk-rust/discussions/174) and ADR 0061
- **Assessed source:** [`anoncreds/anoncreds-v2-rs@691297a7`](https://github.com/anoncreds/anoncreds-v2-rs/tree/691297a7f9ffcc1f51a5d30741086402d64544c9), Apache-2.0
- **Research:** [repository portfolio](../research/rust-library-reuse/repository-portfolio.md)

## Context

AnonCreds v2 explores BBS/PS credentials, predicates, composite proofs, and
new revocation. The assessed repository has no stable release or declared
MSRV, uses a pre-release cryptography dependency and native `blst`, lacks
mobile/WASM evidence, and states that its normative data model and
interoperability profile remain future work.

## Decision

Classify the repository as `not-adopt`. It may inform read-only research and
cryptographic architecture, but it is not yet a normative conformance oracle,
production dependency, source donor, or fixture authority. No v2 capability is
implied by the AnonCreds v1 conditional decision.

## Consequences

The SDK avoids committing to experimental cryptography and wire semantics while
retaining visibility into the likely ecosystem direction.

## Reconsideration and rollback

Reconsider after a stable specification/profile, interoperable vectors,
reviewed cryptography, maintained release, declared Rust floor, and mobile/WASM
evidence exist. Rollback is documentation-only.
