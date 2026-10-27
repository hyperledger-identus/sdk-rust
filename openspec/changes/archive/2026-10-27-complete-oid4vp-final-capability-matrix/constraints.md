# Constraints and limitations

Impact class: routine
Decision status: not-required
Decision reference: not-required
Constraint blockers: none

## Existing entries affected

The change follows the repository's issue-first, OpenSpec, bounded input,
consumer-ledger, offline evidence, and quality-routing constraints. It changes
no public Rust API, production behavior, dependency, persisted data, wire
format, target support, or consumer repository.

## Introduced or changed constraints

- Matrix statuses are closed: `implemented`, `partial`, `unsupported`, and
  `not-applicable`.
- Every implemented or partial row names existing implementation, canonical
  specification, and executable test evidence.
- Every missing required capability is represented as `partial` or
  `unsupported` only when a real bounded subset or explicit profile decision
  exists; otherwise it fails validation and names one focused issue.
- Evidence paths are repository-relative, regular, non-symlink files.
- The checker is offline, deterministic, standard-library-only, and part of
  the factory fast line.
- Consumer-shaped canaries translate at the adapter boundary and cannot define
  Rust DTOs or public errors.
- IDR-024 cannot become delivered while the matrix reports a required missing
  capability.

## Introduced or changed limitations

The matrix is an engineering traceability artifact, not an OpenID
certification, interoperability score, security audit, product readiness
claim, or promise to implement every optional profile. Source-bound tests do
not prove HTTP origin/TLS behavior, verifier trust, credential authenticity,
presentation proof validity, consent, privacy policy, or downstream adoption.

## Consumer and product impact

No consumer repository, public SDK contract, protocol behavior, dependency,
or release claim changes. Consumers gain a navigation/report artifact and
test-only examples of thin adapter translation. IDR-024 remains in progress;
IDR-031 is reconciled to its already delivered draft-core status.

## Evidence

Issue #541 and child issues #544 through #547 are the durable ownership record.
The OpenSpec scenarios, checker tests, Rust reference harness, factory check,
exact-head CLI review, and protected CI are the activation evidence.

## Activation and rollback

Activation requires strict OpenSpec validation, issue #541 preflight, checker
unit tests, reference E2E and adapter canaries, factory gates, CLI review, and
protected CI. Rollback deletes the additive evidence and checker; production
behavior and wire contracts are unaffected.
