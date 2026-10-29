# Constraints and limitations

Impact class: routine
Decision status: not-required
Decision reference: not-required
Constraint blockers: none

## Existing entries affected

- Every pull request still binds exactly one branch issue and records completed
  local review, constraint impact, limitations, and compatibility disposition.
- `develop` remains the only integration target; `main`, release, publishing,
  and repository administration authority are unchanged.
- Protected-base code remains the only executable authority in the privileged
  pull-request policy workflow.
- Pull-request bodies remain bounded by GitHub and the local file-backed
  delivery preflight.

## Introduced or changed constraints

- Pull requests explicitly classify branch-issue linkage as `closes` or
  `deferred`.
- Deferred linkage requires one standalone non-closing reference and one exact
  archived OpenSpec sidecar from the reviewed head.
- The sidecar uses a closed schema, a 16 KiB ceiling, duplicate-key-safe JSON,
  the matching issue number, one rationale, and one to sixteen named remaining
  acceptance gates.

## Introduced or changed limitations

- Deferred linkage does not prove the later gates; it only keeps their owning
  issue honestly open.
- The first version supports repository-local issue numbers and archived
  OpenSpec evidence only.
- Existing open pull requests must add the linkage field when they are next
  edited or synchronized after activation.

## Consumer and product impact

No SDK consumer or product behavior changes. Maintainers gain an auditable
post-merge acceptance path without false closure.

## Activation and rollback

Activation occurs when the protected `develop` merge makes the updated base
checker authoritative. Rollback reverts the checker, template, documentation,
and tests; ordinary closing linkage remains available throughout.

## Evidence

Issue #291, issue #388/PR #585, deterministic linkage/parser mutations, factory
validation, and exact-head protected CI provide the evidence.
