# ADR 0068: use Impierce OpenID4VC as a protocol oracle

- **Status:** Accepted
- **Date:** 2026-09-08
- **Issue:** [#175](https://github.com/hyperledger-identus/sdk-rust/issues/175)
- **Decision authority:** [discussion #174](https://github.com/hyperledger-identus/sdk-rust/discussions/174) and ADR 0061
- **Assessed source:** [`impierce/openid4vc@e9d99d21`](https://github.com/impierce/openid4vc/tree/e9d99d211036f61d46f2c5c56e26197f74290929), Apache-2.0
- **Research:** [repository portfolio](../research/rust-library-reuse/repository-portfolio.md)

## Context

Impierce tracks OID4VCI 1.0, OID4VP 1.0, and SIOPv2 and is valuable independent
behavior for the SDK's protocol work. The assessed workspace has no declared
MSRV or release line, uses Git-pinned IOTA dependencies and a forked SD-JWT
patch, and couples protocol models to IOTA types, Tokio, and Reqwest. Mobile and
WASM support are not demonstrated.

## Decision

Classify the repository as `oracle`. It may be used read-only and as a dev-only
protocol oracle or bounded interoperability PoC. It must not enter production,
build, public API, or release dependency graphs under this ADR.

## Consequences

The SDK retains transport-neutral owned wire/state types while comparing its
interpretation with an active independent implementation. Oracle behavior
never outranks the final specification or conformance suite.

## Reconsideration and rollback

Reconsider when relevant crates have maintained releases without Git/fork
patches, declare a toolchain, pass every effective Rust/toolchain and target
gate, and expose a narrow protocol-core seam. Rollback is documentation-only.
