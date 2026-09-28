# Validate rebased pull-request commits

## Why

The local pre-push hook currently validates `remote_sha..local_sha`. After a
feature branch is rebased onto a newer protected `origin/develop`, the old
remote head is no longer an ancestor of the local head. Git then includes the
new protected-base squash commits in that range, and local signature
verification fails when the contributor does not have GitHub's web-flow key.
The hook blocks a correct force-with-lease update while adding no protection to
the feature commits that actually need local verification.

## What changes

- Resolve the exact fetched `refs/remotes/origin/develop` commit before any
  contribution range is selected.
- Retain the remote branch head for an ordinary fast-forward update only when
  it is an ancestor of the local head and is rooted in the current protected
  base.
- Fall back to current `origin/develop` after a legitimate rebase, while
  rejecting missing, stale, or unrelated ancestry evidence.
- Add hermetic Git-graph tests for first push, fast-forward, rebase, unsigned
  feature commits, unrelated history, and missing protected-base state.

## Capabilities

### Modified capabilities

- `factory-operations`: make local outgoing-commit validation select a trusted
  contribution range after rebases without verifying protected-base commits.

## Non-goals

- No hosted contribution-policy, branch protection, signature, DCO, commit
  grammar, GitHub keyring, workflow, SDK, crate, dependency, or release change.
- No exemption for unsigned feature commits and no trust in an arbitrary ref.
- No automatic fetch, push, force-push, rebase, or remote mutation.

## Delivery

Issue #333 owns this factory-only slice. It targets protected `develop`; the
existing single Linux `fast` gate remains the integration decision.
