# ADR 0077: do not adopt the DIDKit facade

- **Status:** Accepted
- **Date:** 2026-09-08
- **Issue:** [#175](https://github.com/hyperledger-identus/sdk-rust/issues/175)
- **Decision authority:** [discussion #174](https://github.com/hyperledger-identus/sdk-rust/discussions/174) and ADR 0061
- **Assessed source:** [`spruceid/didkit@57a3b451`](https://github.com/spruceid/didkit/tree/57a3b45111f8b1003ef1a50cf7cf21c81cd59cb1), Apache-2.0
- **Research:** [repository portfolio](../research/rust-library-reuse/repository-portfolio.md)

## Context

DIDKit is archived and its deprecation notice directs users to maintained
Spruce SSI libraries and newer mobile products. The assessed facade uses old
SSI and binding generations, edition 2018, no declared MSRV, and unconditional
JNI-era coupling. It adds no unique standards engine that justifies reviving
or forking it for SDK-Rust.

## Decision

Classify DIDKit as `not-adopt`. Do not add it as a dependency, fork it, port its
facade, or use it as a current conformance authority. Historical CLI/FFI
integration may be studied read-only. Standards mechanics are evaluated in the
maintained underlying Spruce SSI crates; SDK bindings remain SDK-owned.

## Consequences

The project avoids anchoring new APIs and mobile bindings to an archived
facade. ADR 0066 is the decision path for current Spruce components.

## Reconsideration and rollback

Reconsider only if the repository becomes maintained and unarchived with a
release that provides unique value absent from its underlying libraries. A
later ADR must supersede this record. Rollback is documentation-only.
