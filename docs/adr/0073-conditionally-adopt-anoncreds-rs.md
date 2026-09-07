# ADR 0073: conditionally adopt anoncreds-rs

- **Status:** Accepted
- **Date:** 2026-09-08
- **Issue:** [#175](https://github.com/hyperledger-identus/sdk-rust/issues/175)
- **Roadmap:** IDR-050, currently conditional
- **Decision authority:** [discussion #174](https://github.com/hyperledger-identus/sdk-rust/discussions/174) and ADR 0061
- **Assessed source:** [`anoncreds/anoncreds-rs@08317a74`](https://github.com/anoncreds/anoncreds-rs/tree/08317a7428afe81f7b710669dc64878f98a6447b), Apache-2.0
- **Research:** [repository portfolio](../research/rust-library-reuse/repository-portfolio.md)

## Context

AnonCreds Rust is the reference implementation for AnonCreds v1 issuer,
prover, verifier, predicate, and revocation behavior. Reimplementing its
CL-signature cryptography would create unacceptable correctness and review
cost. Its Rust library is not published on crates.io and its graph includes
OpenSSL/native code, optional C FFI, unsafe code, W3C conversion, and VDR/
secret-handling concerns. It has broad native/mobile CI but no WASM evidence.

## Decision

Classify the repository as `conditional-adopt`. When IDR-050 is explicitly
activated, use an exact release/tag behind an optional adapter with SDK-owned
credential, verifier, secret, error, and VDR ports. Disable default features
where practical. Do not reimplement the cryptographic protocol or leak
AnonCreds types into generic credential/FFI/WASM APIs.

Activation requires profile/version selection, conformance and revocation
vectors, native/OpenSSL provenance, reachable unsafe review, mobile packaging,
thread/cancellation behavior, secret redaction/zeroization, and proof that the
generic SDK remains usable without the feature.

## Consequences

The roadmap has a credible implementation path without pulling AnonCreds into
the default SDK. No capability or dependency is activated now.

## Reconsideration and rollback

Reconsider if maintenance, licensing, security, or adapter isolation fails.
Remove a future optional adapter by reverting its isolated PR/feature; this ADR
itself is documentation-only.
