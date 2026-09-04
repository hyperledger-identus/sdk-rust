# ADR 0024: derive policy-neutral verification reports

- **Status:** Accepted
- **Date:** 2026-09-05
- **Issue:** #73
- **Decision scope:** first `IDR-009` verification slice

## Context

Credential holders need portable evidence describing which verification steps
succeeded, failed, or were not checked. Donor models demonstrate the need, but
Oxid includes product trust in its verification pipeline and accepts an
aggregate outcome independently from its stages. Midnight and Cardano status
implementations also prove that chain-specific verification execution must
remain behind adapters.

## Decision

1. Add verification evidence to the existing experimental
   `identus-credentials` crate without adding dependencies or wire types.
2. Define six canonical policy-neutral stages: structural, issuer/key, proof,
   temporal, status, and schema. Trust and acceptance remain product policy.
3. Require bounded machine reason codes for Failed and NotChecked; forbid
   reasons for Passed and exclude dynamic diagnostics from the report.
4. Store every report as a canonical six-element array and derive its outcome
   with Invalid over Indeterminate over Valid precedence. Do not accept a
   caller-provided outcome.
5. Use enum-indexed direct lookup and a bounded six-stage construction pass.
   Record release-mode throughput without a wall-clock correctness threshold.
6. Keep metadata/schema descriptors, verifier execution, evidence payloads,
   status bindings/backends, crypto, trust, storage, protocols, and consumers
   in separate issue-first slices.

## Consequences

- Reports cannot be incomplete, duplicate, reordered after construction, or
  contradictory with their aggregate outcome.
- Valid evidence remains independent from relying-party trust; Indeterminate
  explicitly represents incomplete evidence without claiming invalidity.
- Future normative stages require an explicit compatibility decision.
- Oxid, midnight-identity, Lace ID Portal, and NeoPRISM stay unchanged.
- The change is independently revertible before publication.
