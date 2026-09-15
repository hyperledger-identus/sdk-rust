# Constraints and limitations

Impact class: routine
Decision status: not-required
Decision reference: not-required
Constraint blockers: none

## Existing entries affected

- `SDK-DELIVERY-001`: the repair remains issue-linked, specified before its
  implementation, reviewed, tested, and protected by CI.
- `SDK-LIM-009`: the repair restores the accepted manual/weekly macOS evidence
  path without changing cadence or required pull-request checks.
- `SDK-REPO-002`: publication, release, and ordinary `main` delivery remain
  prohibited.

## Introduced or changed constraints

None. Explicit resolution below the selected Android SDK enforces the existing
exact-input and fail-closed intent.

## Introduced or changed limitations

None. The weekly macOS gate continues depending on GitHub's declared Android
SDK layout and network access to Google's SDK repository.

## Consumer and product impact

No consumer or runtime SDK change. Maintainers regain the intended macOS
Android package/emulator evidence in slow CI.

## Activation and rollback

The routine repair activates with its issue-linked PR to `develop`. Rollback
restores the ambient `PATH` dependency and the observed failure; no external
consumer or artifact cleanup is required.

## Evidence

Planning preflight, offline workflow-policy mutation tests, YAML validation,
protected PR CI, and a successful exact-head manual canary form the evidence.
