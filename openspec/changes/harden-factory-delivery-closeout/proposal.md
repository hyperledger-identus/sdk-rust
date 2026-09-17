# Harden factory delivery closeout

## Why

PR #319 passed implementation, local verification, and the required hosted
`fast` line, but the supervisor still paid for two avoidable pull-request body
policy retries, used an ad-hoc squash-message argument that retained literal
`\\n` text, manually reconstructed post-merge evidence, and could not close the
clean managed worktree for superseded PR #316. The Pi runtime and repository
policy audits pass, so the measured problem is the supervisor-owned delivery
edge rather than the worker shell.

## What changes

- Add one file-backed local pull-request metadata preflight that applies both
  hosted policy layers before remote pull-request creation.
- Add a guarded exact-head squash merge command that reads a real multiline
  commit body from a regular file, uses normal GitHub protection, and records
  an owner-private merge receipt after hosted verification.
- Add a recoverable superseded-worktree closeout mode. It removes only a clean
  registered worktree after proving the original head is preserved on its
  remote branch and a named replacement PR merged to `develop` while explicitly
  closing the original branch issue.
- Exercise the result through one bounded supervisor/Pi canary and record
  aggregate evidence without retaining worker content.

## Capabilities

### Modified capabilities

- `factory-operations`: closes the local/hosted PR metadata, protected merge,
  receipt, and superseded-worktree lifecycle.

## Non-goals

- No Pi, model, Node, Nix, Rust, npm package, dependency, workflow, or CI-lane
  version change.
- No branch-protection bypass, repository-setting mutation, publication,
  release, `main` activation, consumer mutation, or branch deletion.
- No inference that a replacement PR is semantically equivalent. The closeout
  proof is deliberately recoverability-based: the superseded head remains on
  the remote branch and only its clean local worktree is removed.

## Delivery

Issue #320 owns this reversible factory-only slice. It targets protected
`develop`, requires the ordinary exact-head `fast` status, and retains the
complete slow line as production-promotion evidence rather than per-PR CI.
