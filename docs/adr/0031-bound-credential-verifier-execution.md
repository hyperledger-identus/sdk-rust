# ADR 0031: bound credential verifier execution by exact format

- **Status:** Accepted
- **Date:** 2026-09-05
- **Issue:** #87
- **Decision scope:** second `IDR-009` verification slice

## Context

The SDK has canonical policy-neutral credential verification reports but no
way to invoke a format implementation. Oxid proves the operation is
asynchronous, while Midnight and Cardano prove concrete verification may need
chain, DID, status, proof-runtime and clock dependencies. Passing a full holder
envelope would also expose private material that a verifier does not need.

## Decision

1. Add one object-safe async `CredentialVerifier` port to
   `identus-credentials`, using the repository's boxed borrowing-future and
   `#[identus::port]` conventions.
2. Pass a borrowed request view containing only format, payload and optional
   detached proof. Exclude format-private holder material by construction.
3. Return the existing canonical `VerificationReport` for every completed
   validity evaluation. Reserve data-free errors for unsupported format,
   unavailable dependency and internal inability to produce a report.
4. Bind one exact `CredentialFormat` to one shared verifier through a builder;
   reject duplicates and more than 64 bindings before immutable construction.
5. Make the clone-cheap registry implement the port through exact format-token
   lookup. Do not probe payload bytes, fall back, retry or select by policy.
6. Keep parsing, crypto, DID/status/schema/clock/network dependencies, trust,
   scheduling, storage, wire protocols and concrete adapters downstream.
7. Measure ready-future dispatch through the production registry without a
   wall-clock correctness threshold.

## Consequences

- Midnight, Cardano, W3C and dummy adapters can share one injection and
  dispatch seam without becoming SDK dependencies.
- Invalid credential evidence cannot be confused with infrastructure outage or
  unsupported format; relying-party trust remains a separate product decision.
- Verifiers cannot receive holder private material through this API, and the
  hot path copies no artifact buffers.
- Boxed futures allocate per call and the registry performs bounded
  `O(log n)` lookup. Both are accepted for runtime neutrality and deterministic
  setup; future alternatives require evidence and a compatibility decision.
- Concrete adapters, stage-specific execution, downstream adoption and release
  remain issue-first follow-up work.
