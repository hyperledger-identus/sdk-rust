# Design

## Context

The worker boundary is already closed and healthy. The missing abstraction is
the supervisor transition from a locally accepted candidate to a hosted PR,
protected squash merge, immutable evidence, and local worktree release.

## Decisions

### One delivery facade, existing validators

Add `scripts/factory delivery` backed by a focused Node module. `pr-preflight`
reads title/body/branch/base/draft inputs, requires the body to be a bounded
regular non-symlink file, invokes the existing shell PR checker with those
values, and calls the exported contribution-policy validator. No regex or
branch parser is duplicated.

### Dry validation before explicit mutation

`merge-pr` validates repository identity, exact head, open/ready state,
`develop` base, mergeability, body-file safety, multiline shape, DCO trailer,
and required checks. Without `--execute` it reports eligibility and stops.
With `--execute` it calls the ordinary `gh pr merge --squash` path with the
file contents as one argument, then re-reads the hosted object and writes an
owner-private JSON receipt below the Git common directory. The receipt is
immutable for an equal PR/head and rejects conflicts.

The merge body validator rejects the literal two-character sequence `\\n` so
an escaped command-line representation cannot masquerade as real lines. The
message must end with an exact DCO trailer matching the configured Git identity.

### Recoverability, not semantic-equivalence theater

Extend worktree lifecycle with `closeout-superseded`. It validates the clean
canonical target and original closed PR exactly as `closeout-pr` validates a
merged PR. It additionally requires `origin/<branch>` to equal the expected
head, a named replacement PR merged to `develop`, and an explicit closing
keyword for the original branch issue in the replacement body. It removes only
the registered worktree; local and remote branch refs remain.

This does not claim that arbitrary replacement code is equivalent. If the
replacement were wrong, the preserved branch can recreate the deleted clean
worktree exactly.

### Tests and canary

Export pure parsers/validators and inject bounded command runners for unit
tests. Integration fixtures cover unsafe files, metadata drift, readiness,
required checks, conflicting receipts, and every superseded-closeout reject
path. After the planning receipt, one Pi worker implements a bounded task under
the existing supervisor envelope; the supervisor independently validates Git
effects and harvests aggregate session evidence.

## Error and privacy contract

Errors identify the failed invariant without printing PR bodies, tokens,
hosted raw responses, or worker content. Receipt digests use SHA-256 over the
validated body bytes. Owner-private directories/files use `0700`/`0600` and
reject symlinks or conflicting existing records.

## Rollback

The new delivery subcommand and superseded mode are additive. Reverting them
restores the prior manual path. No SDK runtime, branch, GitHub setting, or
historical record requires migration.
