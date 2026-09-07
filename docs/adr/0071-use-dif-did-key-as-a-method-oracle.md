# ADR 0071: use DIF did-key.rs as a method oracle

- **Status:** Accepted
- **Date:** 2026-09-08
- **Issue:** [#175](https://github.com/hyperledger-identus/sdk-rust/issues/175)
- **Decision authority:** [discussion #174](https://github.com/hyperledger-identus/sdk-rust/discussions/174) and ADR 0061
- **Assessed source:** [`decentralized-identity/did-key.rs@eb00da60`](https://github.com/decentralized-identity/did-key.rs/tree/eb00da6074d8bc61e5d4c8129fbdd9dc05735cbf), Apache-2.0
- **Research:** [repository portfolio](../research/rust-library-reuse/repository-portfolio.md)

## Context

DIF did-key.rs is focused and supports useful Ed25519, X25519, P-256,
secp256k1, and BLS12-381 variants. The assessed crate has no declared MSRV,
uses nightly Ubuntu CI, carries old crypto dependency generations, and lacks a
current release/target evidence path suitable for a new SDK dependency.

## Decision

Classify the repository as `oracle`. Agents may use its method behavior and
properly attributed vectors for differential testing. The SDK retains an
Identus-owned `did:key` adapter or may separately evaluate a maintained narrow
Spruce method crate. This repository is not a production dependency and is not
approved for signing or key generation.

## Consequences

Useful multi-key behavior remains available as evidence without importing old
cryptography or nightly assumptions.

## Reconsideration and rollback

Reconsider after active maintenance produces a stable release with modern
dependencies and exact key-family, effective Rust/toolchain, target, and
conformance parity. Rollback is documentation-only.
