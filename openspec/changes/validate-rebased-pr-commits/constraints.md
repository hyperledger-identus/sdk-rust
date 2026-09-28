# Constraints and limitations

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/333
Constraint blockers: none

## Existing entries affected

- The accepted local/hosted contribution-provenance requirement remains
  unchanged.
- Required Conventional Commit, DCO, accepted signature-envelope, issue-branch,
  secret-safety, and hosted verification rules remain unchanged.
- Protected `develop`, the single fast PR line, and human-controlled repository
  administration are unchanged.

## Introduced or changed constraints

The local pre-push hook SHALL resolve the fixed current fetched
`refs/remotes/origin/develop` commit and prove it is an ancestor of the outgoing
local head. It MAY use the remote feature-branch head as the range base only
for a normal fast-forward update when that commit is both an ancestor of the
local head and a descendant of the current protected base. A first push or
legitimate rebase SHALL validate the feature contribution relative to current
`origin/develop`. Missing objects, missing base, unrelated history, or failed
ancestry proof SHALL fail closed.

## Introduced or changed limitations

The hook uses the contributor's fetched `origin/develop`; it does not fetch or
claim that the local remote-tracking ref is server-current. Repository doctor
and normal delivery synchronization remain responsible for refreshing it.
Only the repository's fixed `develop` topology is supported.

## Consumer and product impact

No SDK consumer or product behavior changes. Contributors can force-with-lease
an honestly rebased, signed branch without importing GitHub's signing key,
while unsigned authored commits and ambiguous histories remain blocked.

## Activation and rollback

Activation requires issue-bound preflight, hermetic graph/signature tests,
factory/Nix checks, signed+DCO local review, hosted exact-head CI, and protected
merge. Rollback reverts the resolver, tests, and spec together; no data or
consumer migration exists.

## Evidence

Issue #333, the current hook behavior, exact Git graph fixtures, and the
canonical contribution-provenance requirement are sufficient. No material
constraint blocker or protected authority is involved.
