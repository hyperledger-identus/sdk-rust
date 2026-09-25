# Constraints and limitations

Impact class: routine
Decision status: not-required
Decision reference: not-required
Constraint blockers: none

## Existing entries affected

- The PR body remains bounded to 64 KiB at the file-backed delivery boundary.
- Issue linkage, completed local review, constraint impact and explicit
  limitations remain mandatory.
- Hosted and local validation continue to use the repository-owned checker.
- The single fast PR line and the weekly/manual slow line are unchanged.

## Introduced or changed constraints

None. This change removes input-size/match-position sensitivity from an
existing validation contract.

## Introduced or changed limitations

None.

## Consumer and product impact

No SDK consumer or product behavior changes. Contributors receive stable
validation for valid bounded descriptions.

## Activation and rollback

Revert the checker and regression tests. No data, API, dependency, workflow or
consumer migration is involved.

## Evidence

Issue #374, the observed #373/#379 false failures, deterministic long-body
tests, factory validation and exact-head hosted CI provide the evidence.
