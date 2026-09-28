# ADR 0167: require monotonic contribution policy

- **Status:** Accepted under issue #339
- **Date:** 2026-09-29
- **Issue:** [#339](https://github.com/hyperledger-identus/sdk-rust/issues/339)
- **Depends on:** ADR 0166

## Context

Protected-base evaluation prevents a pull request from changing the checker
that judges that same pull request. It does not prevent a relaxed policy from
becoming authoritative for later work after merge.

## Decision

The base-owned checker will read the proposed policy from the exact imported
head Git object as bounded, duplicate-rejecting JSON. A closed version-1 schema
defines a partial order: allowances/exemptions may shrink, enforcement flags
may strengthen, and numeric ceilings may decrease. Inert descriptive arrays
remain set-equivalent. Unknown or malformed content and every relaxation fail
closed. Proposed code is never loaded or executed.

There is no single-PR relaxation waiver. Intentional relaxation requires a
separate two-integration ADR that first establishes an exact base-owned approval
mechanism, preventing the relaxation from authorizing itself.

## Consequences

Policy strengthening remains routine and deterministic. Relaxation becomes an
explicit governance transition. New schema fields cannot silently bypass the
order because unknown fields fail.

## Verification and rollback

Per-field mutation tests, malformed/resource-bound vectors, exact-object
ingestion tests, workflow source assertions, factory validation, and natural
hosted evidence are required. Rollback removes the additive comparison step;
no SDK or consumer contract changes.
