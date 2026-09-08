## Context

The guarded archive facade must translate an upstream mutation into a truthful
repository receipt. Its current postcondition predicts one destination with the
host-local calendar date. The completed issue #214 lifecycle demonstrated that
OpenSpec can choose another date while still performing every required
mutation. Retrying after that false failure is unsafe and confusing because the
active change has already moved.

## Goals and non-goals

Goals:

- discover the archive created for the requested change without assuming a
  timezone or a stable clock date;
- distinguish the newly created entry from pre-existing archives;
- reject absent, ambiguous, non-directory and symlink results;
- retain all existing preservation and validation checks;
- prove behavior with a deterministic fake mutator.

Non-goals:

- change OpenSpec or rename valid archives;
- parse human-readable OpenSpec output;
- support concurrent archive commands in the same worktree;
- implement rollback for arbitrary partial canonical mutation;
- change SDK runtime or downstream consumers.

## Decisions

### Snapshot exact matching archive entries

Before invoking OpenSpec, enumerate immediate archive-root entries matching the
fixed `YYYY-MM-DD-<change>` shape. After OpenSpec returns, enumerate again and
compare exact quoted paths. The receipt requires exactly one entry absent from
the first snapshot. Pre-existing matching archives are therefore evidence, not
candidates for the completed transition.

This avoids both host-timezone coupling and a race when a calendar boundary is
crossed during the command. It also permits historical archives with the same
suffix without broadening collision semantics.

### Validate identity before contents

Require the single new entry to be a real directory and not a symlink. Then
retain active-change removal, mandatory regular-file artifacts, the regular
`specs` directory and final whole-factory validation before printing success.
Zero or multiple new entries, a pre-existing destination that OpenSpec cannot
replace, and a new symlink all fail without a success marker.

### Keep an early collision check only when deterministic

The before snapshot remains the source of truth. An existing exact destination
that the wrapper can determine before mutation may still be rejected early,
but correctness does not depend on predicting the mutator's date. A collision
that cannot be predicted produces no new entry and therefore fails closed
after the upstream command.

### Parameterize only the hermetic mutator's date

The fake OpenSpec archive command accepts a fixture-only archive date. The
regression chooses a fixed date distinct from the host's current local date,
proving the receipt without setting the system clock or environment timezone.
Production code receives no test hook.

## Risks and mitigations

- **Concurrent mutation:** two matching new entries are ambiguous. The facade
  fails closed and documents that one worktree is a single-writer surface.
- **Filename convention drift:** no candidate is found. The contract test and
  failure force an explicit update instead of silently accepting a wrong path.
- **Special path characters:** repository root and candidates remain quoted;
  OpenSpec constrains the change identifier.
- **Partial mutation:** validation stops and leaves evidence. Automatic repair
  remains outside this bounded change.

## Rollback

Revert the factory, tests, documentation and canonical delta. No package or
consumer migration is required. The known local-date false failure returns.
