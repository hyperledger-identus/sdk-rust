# ADR 0072: use DIDComm Rust as a reference only

- **Status:** Accepted
- **Date:** 2026-09-08
- **Issue:** [#175](https://github.com/hyperledger-identus/sdk-rust/issues/175)
- **Roadmap:** IDR-041, currently conditional
- **Decision authority:** [discussion #174](https://github.com/hyperledger-identus/sdk-rust/discussions/174) and ADR 0061
- **Assessed source:** [`sicpa-dlab/didcomm-rust@4388350d`](https://github.com/sicpa-dlab/didcomm-rust/tree/4388350def84b6d7f6b65cf4a451607200035d8d), Apache-2.0
- **Research:** [repository portfolio](../research/rust-library-reuse/repository-portfolio.md)

## Context

SICPA DIDComm Rust implements useful DIDComm v2 packing, authcrypt/anoncrypt,
routing, `fromPrior`, resolver seams, WASM, and historical UniFFI integration.
Its 0.4.1 release is from 2023, its CI/dependency generations are stale, it has
no declared MSRV, and it pins an old Askar crypto Git revision. The roadmap
requires a specifically pinned DIDComm 2.1 capability.

## Decision

Classify the repository as `not-adopt`. Use it read-only for protocol
architecture and historical interoperability comparisons. Do not add it to
normal/build dependencies, fork it, or treat its vectors, supported algorithms,
and DID resolution behavior as current conformance authority.

## Consequences

IDR-041 remains conditional. A future DIDComm component will either use a
maintained narrow engine or implement the selected profile behind SDK-owned
ports, with standards and independent vectors outranking this implementation.

## Reconsideration and rollback

Reconsider after a maintained DIDComm 2.1 release removes stale Git/native
dependencies and passes every effective Rust/toolchain and target gate plus
mobile/WASM interoperability evidence. Rollback is documentation-only.
