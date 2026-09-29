# Constraint impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/484
Constraint blockers: none

## Existing entries affected

- `SDK-COMPAT-002` through `SDK-COMPAT-005`: selected compilers and candidate
  dependency bytes remain unchanged.
- `SDK-DELIVERY-001`: refresh is local review preparation, not a new CI line or
  slow-workflow trigger.
- `SDK-REL-001` and `SDK-REL-002`: DID remains candidate-only and unpublished.
- `SDK-LIM-003`: portable rows remain compile-only.

## Introduced or changed constraints

Each lane and the archive receipt must attest the digest actually installed,
and every Cargo command operating on staged candidate sources must be locked.
Ordinary evidence cannot generate dependency resolution. Only the distinct
extracted-package closure and explicit local refresh mode may generate a lock,
through one policy-visible capability boundary.

## Introduced or changed limitations

Refresh proposes bytes and a dependency-cone diff but deliberately does not
edit or approve the repository lock or descriptor. Registry availability and
freshness are operator inputs during refresh; ordinary evidence continues to
use the committed snapshot. Final cross-host evidence remains pending #388.

## Consumer and product impact

No SDK runtime consumer changes. Release reviewers gain direct provenance from
installed bytes to receipts and one reproducible way to inspect a future lock
update before editing tracked state.

## Activation and rollback

Builder, checker, mutations, refresh mode, and specification activate together
on protected `develop`. A normal revert restores the reviewed #483 state but
reopens documented attestation and refresh gaps, so it cannot be used for final
M5 evidence.

## Evidence

Planning receipt, focused mutation tests, no-drift refresh, candidate build,
primary/MSRV lanes, factory/Nix gates, distinct review, and exact-head CI are
required. Publication and slow dispatch remain unauthorized.
