# Factory metrics

Metrics exist to tune throughput and reliability, not to record conversations.
The closed schema is
[`work-item-metrics-v1.schema.json`](work-item-metrics-v1.schema.json).

Records are keyed by issue and exact head under the repository's Git common
directory, outside tracked worktrees. They contain timestamps, bounded counters,
coverage, disk use and outcome. Unknown counters are `null`; they are never
estimated. Prompts, messages, transcripts, credentials, secret material and raw
tool output are forbidden.

```bash
scripts/factory metrics template --issue 123 > /tmp/metric.json
scripts/factory metrics validate --file /tmp/metric.json --current-head
scripts/factory metrics write --file /tmp/metric.json
scripts/factory metrics render --file /tmp/metric.json
scripts/factory metrics publish --file /tmp/metric.json --issue 123 --execute
```

Publication is explicit. It creates or updates only the authenticated GitHub
user's unique matching metrics comment, contains one visible aggregate and one
hidden canonical payload, and refuses ambiguity or payloads above 8 KiB. The
private store has a 90-day policy; pruning is a future bounded maintenance task,
not an implicit destructive bootstrap action.
