# Constraints and limitations

Impact class: routine
Decision status: not-required
Decision reference: not-required
Constraint blockers: none

## Existing entries affected

ADRs 0003/0004 and the issue-#320 factory contract continue to require
exact-head protected delivery and immutable evidence. Rust 1.98.1 and the
fast/slow CI policy are unchanged.

## Introduced or changed constraints

None. The repair makes existing hosted receipt validation accept GitHub's
canonical seconds form.

## Introduced or changed limitations

The local receipt grammar remains UTC-only and deliberately rejects equivalent
numeric offsets. Recovery still requires the exact merge body file.

## Consumer and product impact

No SDK or downstream consumer impact. Supervisors can retain truthful receipts
after GitHub returns either supported UTC precision.

## Activation and rollback

Activation requires planning preflight, deterministic negative tests, live
recovery of #321, local review, signed/DCO PR, exact-head fast CI, and discovery
review. Revert restores fail-closed behavior but makes seconds-form recovery
unavailable again.

## Evidence

Issue #322 and PR #321 provide the exact hosted failure. This is reversible
repository-local maintenance under existing factory authority.
