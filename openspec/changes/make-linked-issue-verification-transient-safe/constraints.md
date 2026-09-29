# Constraints and limitations

Impact class: routine
Decision status: not-required
Decision reference: not-required
Constraint blockers: none

## Existing entries affected

- Every non-draft PR to `develop` must reference a real repository issue.
- The privileged policy workflow must execute exact protected-base code only.
- Provider/API inability to establish required evidence must fail closed.

## Introduced or changed constraints

One initial lookup plus at most two retries is permitted only for recognized
transient transport/service failures. A successful retry must satisfy the same
exact repository issue identity contract as an immediate success.

## Introduced or changed limitations

The workflow may add up to four seconds before failing after repeated transient
GitHub API errors. It does not rerun jobs or recover from permanent API,
authentication, permission, missing-record, or identity errors.

## Consumer and product impact

None. This is repository factory behavior only.

## Activation and rollback

Merge the tested script and protected-base workflow reference together. Revert
both to restore single-attempt verification; no migration is required.

## Evidence

Issue #474, run `36526211467`, deterministic fake-provider contract tests,
strict factory validation, workflow lint, and exact-head hosted CI provide the
bounded evidence.
