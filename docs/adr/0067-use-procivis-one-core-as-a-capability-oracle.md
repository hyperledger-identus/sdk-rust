# ADR 0067: use Procivis ONE Core as a capability oracle

- **Status:** Accepted
- **Date:** 2026-09-08
- **Issue:** [#175](https://github.com/hyperledger-identus/sdk-rust/issues/175)
- **Decision authority:** [discussion #174](https://github.com/hyperledger-identus/sdk-rust/discussions/174) and ADR 0061
- **Assessed source:** [`procivis/one-core@e66ec8c5`](https://github.com/procivis/one-core/tree/e66ec8c533acb919611df75a38c7d23c2ccf658a), Apache-2.0
- **Research:** [repository portfolio](../research/rust-library-reuse/repository-portfolio.md)

## Context

ONE Core implements a wide, current identity surface and demonstrates a useful
provider architecture. Its core packages are unpublished workspace components
inside an application/backend with runtime, HTTP, database, configuration,
build-script, and UniFFI coupling.

## Decision

Classify ONE Core as `oracle`. Agents may study its provider seams and use
properly attributed fixtures or dev-only differential behavior for mdoc,
SD-JWT VC, status, OID4VC, DID, and Data Integrity. It is not a production,
build, or public-model dependency, and code extraction is not authorized by
this ADR.

A future extraction requires a separate ADR proving that one cohesive,
published or immutably vendorable component is isolated from product policy,
transport, persistence, Tokio/Reqwest, SQL, configuration, and framework types.

## Consequences

The SDK benefits from production architecture and interoperability evidence
without importing an application stack. Equivalent mechanics may remain
SDK-owned or use a different narrow crate.

## Reconsideration and rollback

Reconsider when a stable narrow component meets the SDK facade, profile,
target, cone, provenance, and security gates. Rollback is a documentation
revert; no runtime dependency changes.
