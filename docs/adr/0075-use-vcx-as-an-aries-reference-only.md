# ADR 0075: use VCX as an Aries reference only

- **Status:** Accepted
- **Date:** 2026-09-08
- **Issue:** [#175](https://github.com/hyperledger-identus/sdk-rust/issues/175)
- **Decision authority:** [discussion #174](https://github.com/hyperledger-identus/sdk-rust/discussions/174) and ADR 0061
- **Assessed source:** [`openwallet-foundation/vcx@5fa3cd50`](https://github.com/openwallet-foundation/vcx/tree/5fa3cd5023ed4a0dc42a36f48bb1834961c3812d), Apache-2.0
- **Research:** [repository portfolio](../research/rust-library-reuse/repository-portfolio.md)

## Context

VCX is an active and mature Aries state-machine and integration workspace. It
primarily represents DIDComm v1/Aries behavior and couples many unpublished
crates to Tokio, Reqwest, servers, OpenSSL, ZMQ, Git dependencies, VDRs, and a
wallet runtime. Current upstream CI is Linux-only.

## Decision

Classify VCX as `oracle`. Use it read-only for Aries protocol state machines,
AATH expectations, and historical AnonCreds/DIDComm v1 integration patterns.
Do not use the workspace as the SDK's DIDComm v2, credential, wallet, VDR, or
transport foundation. No VCX type enters public or production SDK surfaces.

## Consequences

Mature lifecycle lessons remain accessible without inheriting the VCX product
architecture or legacy protocol profile.

## Reconsideration and rollback

Reconsider only when a future component identifies one unique cohesive module
that can be isolated from the VCX wallet/runtime and passes exact profile,
publication, cone, target, and security gates. Rollback is documentation-only.
