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
scripts/factory metrics publish --file /tmp/metric.json --issue 123 --execute
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
rejected. Closed objects reject extra fields, attempt counters are checked
against per-check history, and publication requires the current exact head. A
non-null PR is also checked at its exact hosted head before publication.

Public output contains allowlisted aggregates and one bounded hidden canonical
payload using the record version's marker. Publication creates or updates only
the authenticated publisher's unique comment with that same marker, so a v2
publication never overwrites retained v1 evidence. Multiple matching owned
comments fail closed.

Prompts, messages, transcripts, session/response/tool identifiers, credentials,
secret material, commands, raw output, provider/model data and cost or billing
data are forbidden. Pi content-bearing files remain private and are reduced to
[`pi-usage-v1`](pi-usage-v1.schema.json) counters before metrics ingestion.
Unknown counters are null; they are never estimated as zero. The private store
has a 90-day policy, but pruning remains a separate explicit maintenance task.
