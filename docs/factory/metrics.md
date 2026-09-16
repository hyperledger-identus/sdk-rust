# Factory metrics

Metrics tune throughput and reliability; they do not record conversations. The
closed schemas are:

- [`work-item-metrics-v1.schema.json`](work-item-metrics-v1.schema.json) for
  retained historical records; and
- [`work-item-metrics-v2.schema.json`](work-item-metrics-v2.schema.json) for new
  records.

Records are keyed by issue and exact head in a versioned, owner-only store below
the repository Git common directory. Version 1 validation, rendering, storage
and its `sdk-rust-factory-metrics:v1` public marker remain unchanged. Templates
default to version 2; `--schema-version 1` creates a historical-shape template.

```bash
scripts/factory metrics template --issue 123 > /tmp/metric.json
scripts/factory metrics template --issue 123 --schema-version 1 > /tmp/metric-v1.json
scripts/factory metrics validate --file /tmp/metric.json --current-head
scripts/factory metrics write --file /tmp/metric.json
scripts/factory metrics render --file /tmp/metric.json
scripts/factory metrics publish --file /tmp/metric.json --issue 123 --target auto --execute
# Explicit issue receipt or historical backfill:
scripts/factory metrics publish --file /tmp/metric.json --issue 123 --target issue --execute
```

Version 2 distinguishes:

- total, planning, implementation, review, validation, hosted-CI and blocked
  durations;
- CI critical-path queue/execution durations, per-check attempt history,
  failed/canceled attempts, retries and post-CI pushes; parallel check
  durations are not summed into the critical path;
- Pi sessions, turns, tool calls and non-overlapping input, output, cache-read
  and cache-write tokens; and
- coverage plus separate worktree, target, cache and process-RSS peaks.

Every version 2 measurement is `{ "value", "unavailableReason" }`. An exact
value has a null reason. An unavailable value is null and uses one closed reason:
`not-measured`, `source-missing`, `source-malformed`, `source-unsafe`,
`unsupported-version`, `incomplete-usage`, `incomplete-events` or
`not-applicable`. Zero is valid only when it was measured exactly.

Input files are byte-bounded regular non-symlink JSON with duplicate fields
rejected. Closed objects reject extra fields, and attempt counters are checked
against per-check history. `write` requires the current exact head. Publication
of a PR-backed record instead verifies the issue, PR, the PR's authoritative
closing reference to that issue, and the exact hosted PR head, so a retained
record can be published after merge without checking out its old commit. A
record without a PR remains current-head-only.

Public output contains allowlisted aggregates and one bounded hidden canonical
payload using the record version's marker. Publication defaults to the recorded
PR and falls back to the issue; `--target issue` is the explicit issue-level or
backfill override, while `--target pull-request` requires a recorded PR. Before
the first remote mutation, the command renders and size-checks the public
payload, then atomically retains or confirms the exact private record. It then
creates or updates only the authenticated publisher's
unique comment with that same marker, so a v2 publication never overwrites
retained v1 evidence. Multiple matching owned comments fail closed.
Templates and `in-progress` records may be validated or rendered while work is
underway, but they cannot be written to the immutable private store or
published. Finalize the closed outcome and exact counters before retention.
For compatibility with the former overwriting writer, one pre-existing valid
draft may transition atomically to terminal evidence only when its schema,
repository, issue, head, profile and start time match; a previously absent PR
may be bound during that transition. An exclusive same-store claim serializes
the transition, and a competing closeout fails without overwriting the winner.
Terminal evidence never transitions.

A comment create or update receives at most one immediate retry. If both
attempts fail, the local record remains authoritative and the command reports
visible telemetry debt. The supervisor records that debt at closeout without
turning otherwise independent build, test or review evidence into a failure.
Re-running the same command is idempotent: equal local content is confirmed and
the publisher's unique versioned comment is updated rather than duplicated.

Prompts, messages, transcripts, session/response/tool identifiers, credentials,
secret material, commands, raw output, provider/model data and cost or billing
data are forbidden. Pi content-bearing files remain private and are reduced to
[`pi-usage-v1`](pi-usage-v1.schema.json) counters before metrics ingestion.
Unknown counters are null; they are never estimated as zero. The private store
has a 90-day policy, but pruning remains a separate explicit maintenance task.

## Terminal closeout

For every completed production-ready item, validate the closed metric file,
publish it with `--target auto`, and keep the returned public comment as the
derivative receipt. Use `--target issue` when an issue is the requested durable
reporting surface. Historical backfill reads only the bounded metric JSON; it
must never inspect or publish Pi sessions, events, prompts or transcripts.

## Delivery-line interpretation

For the fast integration line, use CI execution rather than queue time for the
latency SLO and retain queue time separately. The initial comparable target is
p50 at most 360 seconds and p95 at most 480 seconds. A rolling p95 above 600
seconds starts a focused optimization issue. Also inspect attempt count,
post-CI pushes and review duration: PR #300 demonstrated that thirteen healthy
seven-minute attempts are a larger delay than one slightly slower attempt.

For the slow production-promotion line, duration is descriptive rather than an
inner-loop SLO. Record exact candidate SHA, total/critical-path duration, job
outcomes and receipt/artifact identity. A green run applies only to that
unchanged candidate. Faster execution never compensates for missing production
evidence, and a slow failure remains promotion debt.
