# Constraints and limitations

Impact class: routine
Decision status: not-required
Decision reference: not-required
Constraint blockers: none

## Existing entries affected

- `SDK-NAME-001`: exact public names are `identus-did` and
  `identus-did-resolver-http`; no alias is created.
- `SDK-RELEASE-001`: canonical workspace publish denial remains effective;
  candidate staging is not release activation.
- `SDK-LIM-004`: declared MSRV 1.89.0 and etalon preparation on 1.98.1 remain.
- ADR 0134 and active first-train OpenSpec remain authoritative for the
  existing `v0.1.0-rc.1` publication.

## Introduced or changed constraints

Every new independent train tag must be prefixed by its primary exact Cargo
package name. A train descriptor is closed, unique, ordered, and candidate-only
until a separate release decision activates canonical package metadata and the
protected publisher.

## Introduced or changed limitations

The DID candidate is local patched-closure evidence, not a registry dry run,
publication approval, SemVer stability promise, or foreign-language package.
The first train retains a historical unprefixed tag exception.

## Consumer and product impact

None in this slice. Consumers continue using the published crypto train or
exact Git revisions for DID packages.

## Activation and rollback

Activation requires issue-first OpenSpec, ADR, deterministic and mutation
tests, local review, factory/cleaned-source checks and exact-head hosted CI.
Rollback removes the new index, DID descriptor/builder/checker and docs; no
remote artifact or consumer needs migration.

## Evidence

Issues #381/#382, existing first-train receipts, source-distribution evidence,
candidate output, mutation tests and protected PR CI provide the evidence.
