# Design

## Context

The metric renderer is already privacy-bounded and comments are already
idempotent by authenticated publisher and version marker. The missing invariant
is dual retention: `write` and `publish` are independent, current-head-only
commands, and publication always targets the issue. Post-merge evidence is the
normal case, so current checkout identity is the wrong proof for history.

## Decisions

### Local authority before remote derivative

The publication command validates the record, verifies its hosted identity,
renders and size-checks the public payload, then atomically persists or
confirms the exact private record before invoking a comment mutation. An
existing byte-equivalent semantic record is idempotent; a different record at
the same issue/head/version path fails closed. Draft `in-progress` records may
be validated and rendered but cannot enter immutable retention.

### Hosted historical identity

When a metric names a PR, GitHub's PR record must match the repository, PR
number, exact `headRefOid`, and an authoritative closing reference to the
recorded issue. This proof applies after merge and does not depend on the
operator's current checkout. The issue target must also exist in the
authoritative repository. Records without a PR remain current-head-only until
a separately specified hosted commit/issue binding exists.

### PR-first target selection

`--target auto` is the default: recorded PR when present, otherwise issue.
`--target issue` supports requested historical backfill and durable issue-level
reporting. `--target pull-request` fails if the record has no PR. The unique
owned comment is selected on the resolved target with the existing versioned
marker, so v1 and v2 never overwrite each other.

### Bounded failure behavior

Remote comment create/update receives at most one immediate retry. If both
attempts fail, the command reports telemetry debt without deleting the local
record. Supervisors record that debt in handoff/closeout evidence; product
checks remain independently interpretable.

### Historical backfill

After merge, the supervisor publishes the five existing retained work-item
records to issues #246, #247, #250, #257, and #259 using only metric JSON.
Existing owned marker comments are updated rather than duplicated.

## Risks and mitigations

- Public payload may leak private content: reuse the closed renderer and
  forbidden-field validation; never ingest raw sessions during publication.
- Historical record may be forged: require exact hosted PR-head evidence,
  matching issue/repository identity, and authoritative PR-to-issue closing
  linkage before local persistence or mutation.
- Comment retries may duplicate: select/update the unique owned marker and
  fail on ambiguity; GitHub issue comments have stable identities.
- Public comments may become stale: exact head and schema marker remain in the
  visible and hidden payload; later versions use distinct markers.

## Rollback

Revert the CLI/policy/docs/tests. Retained private records and already-created
comments remain valid historical evidence; no SDK or consumer migration exists.
