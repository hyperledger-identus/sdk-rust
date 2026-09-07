# ADR 0076: use OWF SD-JWT Rust as a legacy oracle

- **Status:** Accepted
- **Date:** 2026-09-08
- **Issue:** [#175](https://github.com/hyperledger-identus/sdk-rust/issues/175)
- **Decision authority:** [discussion #174](https://github.com/hyperledger-identus/sdk-rust/discussions/174) and ADR 0061
- **Assessed source:** [`openwallet-foundation-labs/sd-jwt-rust@146546cc`](https://github.com/openwallet-foundation-labs/sd-jwt-rust/tree/146546cc20682b6ddee0fcc270211aac9b0bc83b), MIT OR Apache-2.0
- **Normative target:** [RFC 9901](https://www.rfc-editor.org/rfc/rfc9901.html)
- **Research:** [repository portfolio](../research/rust-library-reuse/repository-portfolio.md)

## Context

The repository is a useful independent Rust implementation, but release 0.7.1
and the assessed source explicitly implement SD-JWT draft-07. The SDK roadmap
targets RFC 9901 and a separately pinned current SD-JWT VC profile. Upstream
mobile/WASM evidence is absent and the core uses `jsonwebtoken`/`ring`.

## Decision

Classify the repository as `not-adopt`. It may be used read-only as a legacy
draft-07 reference and for negative migration research. It is not a production
backend, current-profile conformance authority, or source of wire defaults.
SDK implementation follows RFC 9901 and current profile vectors behind Identus
credential/presentation states.

## Consequences

Historical interoperability and regression behavior can be tested without
mistaking a draft implementation for the final standard.

## Reconsideration and rollback

Reconsider after a maintained published release explicitly implements RFC 9901
and the selected SD-JWT VC profile and passes disclosure, key-binding,
duplicate/bounds, and every effective Rust/toolchain and target gate. Rollback
is documentation-only.
