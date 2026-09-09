# Factory recovery

Recovery favors inspection and resumption over deletion.

1. Run `git status --short --branch`, `scripts/factory status`, and
   `scripts/factory worktrees audit --json`.
2. Identify the issue, branch, active OpenSpec change, exact HEAD and any running
   process before starting another worker.
3. Validate `preimplementation.json`. If absent, finish and commit planning,
   then create it before implementation. If invalid, stop: do not rewrite it to
   conceal an earlier implementation edit.
4. Resume only in the worktree registered for that issue. A dirty tree belongs
   to its current owner until reviewed; never reset, clean or stash it merely to
   recover capacity.
5. Recompute target and test evidence after any rebase or code change. Old green
   output does not apply to a new head.
6. When a PR is merged, close the managed worktree only through
   `closeout-pr` with its exact merged head and explicit `--execute`.

If capacity is full, finish or safely close an existing slice. If Pi policy is
misaligned, inspect the reported keys and use the explicit configuration command
only when appropriate. Provider failure, unavailable credentials, ambiguous
product authority, unresolved security risk, or unverifiable provenance are
stop conditions; ordinary formatting, naming and recoverable CI failures are
not.

Never remove the primary checkout, the current worktree, a dirty or locked
worktree, an unknown path, or a worktree whose head differs from the merged PR.

## Pi package cache

If bootstrap reports that `.pi/npm` is operator-owned or points to an
unexpected location, stop and inspect it. Bootstrap will not move, replace or
delete that path. A directory created by a raw Pi run is ignored by Git; after
confirming no Pi process uses it, move it to a named recovery location or
remove that exact directory manually, then rerun
`./bootstrap.sh --prepare-pi-cache`.

Complete shared caches are reported under the hidden repository sibling
`.sdk-rust-factory/pi-packages/v1`. They contain public tooling packages, not
credentials or sessions, but remain operator-owned state. There is no automatic
retention deletion. To reclaim space, first confirm that no active worktree or
Pi process resolves `.pi/npm` to the exact cache, preserve it elsewhere if
recovery is desired, and remove only that reported digest directory. Never
delete the sibling root broadly.

An error during first population retains its uniquely named `.staging-*`
directory for inspection. It is never selected for a Pi launch. Once the cause
is understood and no process owns it, that exact staging directory may be
removed manually.
