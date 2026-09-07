# ADR 0066: conditionally adopt narrow Spruce SSI crates

- **Status:** Accepted
- **Date:** 2026-09-08
- **Supersedes:** ADR 0061's Spruce-specific oracle-only disposition; ADR
  0061's general narrow-engine/owned-facade rule remains effective
- **Issue:** [#175](https://github.com/hyperledger-identus/sdk-rust/issues/175)
- **Decision authority:** [discussion #174](https://github.com/hyperledger-identus/sdk-rust/discussions/174) and ADR 0061
- **Assessed source:** [`spruceid/ssi@89630368`](https://github.com/spruceid/ssi/tree/89630368438c81b55335362b21621d3aadd48d93), Apache-2.0
- **Research:** [repository portfolio](../research/rust-library-reuse/repository-portfolio.md)

## Context

Spruce SSI is the broadest maintained Rust standards suite in the assessment.
Its modular crates cover DID, VC, JOSE, SD-JWT, Data Integrity, JSON-LD,
status, and verification methods. The umbrella facade also creates a large,
policy-rich dependency graph and another public SSI object model.

## Decision

Classify the repository as `conditional-adopt`. A component issue may evaluate
one named narrow crate with minimal features as a private engine. The `ssi`
umbrella facade is not an SDK dependency, and Spruce types do not cross public,
FFI, WASM, persistence, error, or lifecycle boundaries. The repository is also
approved as a dev-only differential oracle under #164.

Activation requires exact profile parity, every effective Rust/toolchain and
target gate at integration time, a measured dependency cone, reachable unsafe/native review,
security/provenance checks, and proof that the Identus facade retains input
limits, trust, transport, storage, consent, and chain policy.

## Consequences

The SDK can reuse mature closed mechanics without converging on a framework.
Adapter work and type mapping are accepted costs. No dependency is added by
this ADR.

## Reconsideration and rollback

Reconsider the umbrella only through a superseding public-model ADR with two
consumer proofs. Revert this documentation decision to roll it back; each
future crate integration remains independently reversible.
