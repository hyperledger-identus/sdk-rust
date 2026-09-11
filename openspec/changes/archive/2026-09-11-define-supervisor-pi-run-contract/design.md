## Context

Pi is already reproducible and repository-wrapped, but the supervisor cannot
currently launch a typed run or consume a typed result. The control plane must
stay outside the model session, while private content-bearing runtime artifacts
must never leak into tracked files or public metrics.

## Decisions

### Use a private run directory and three closed records

Each run lives below the Git common directory and contains a supervisor
invocation envelope, external heartbeat, worker handoff, persisted Pi session
and counter-only harvest. The envelope is immutable after preparation. The
heartbeat and handoff use atomic owner-only replacement. All input paths are
canonical, regular, non-symlink and byte-bounded.

### Bind launch to the preflight receipt

Preparation validates the current repository, issue branch, exact
`origin/develop` merge base/head and active change receipt. It rejects a stale
or mismatched identity before Pi starts. The task and allowed path prefixes are
explicit in the private envelope; no issue body or chat history is inferred.

### Keep liveness outside Pi output

The runner spawns `./bootstrap.sh --pi` as one process group, writes a heartbeat
from supervisor time/process state, and applies a hard deadline plus bounded
termination grace. Pi JSON output and sessions remain private. A terminal
artifact does not imply acceptance until the handoff validates.

### Harvest only terminal persisted-session usage

The collector accepts Pi session JSONL version 3, counts assistant message
records once, and sums their non-overlapping usage buckets. Turn and tool counts
come from the content-free event log when available. It discards all content,
identifiers, cost, provider/model and raw event data. Unsupported or absent
sources map to explicit null reasons.

### Version metrics without erasing history

Validation and rendering dispatch by `schemaVersion`. Version 1 remains closed
and unchanged. Version 2 retains the stable identity fields and adds total and
phase durations, CI queue/execution timings, failed/canceled/retry/push counts,
session/turn/tool/token counters, separate resource peaks and an enum-bounded
reason for every unavailable value. Publication chooses a versioned marker and
updates only the publisher's matching comment.

## Risks and mitigations

- Session drift: reject unknown format versions and preserve null/reason.
- Content leakage: closed aggregate objects, no free-form metric strings,
  secret-term checks and public size bounds.
- False completion: worker exit and handoff acceptance are distinct states.
- Stale/out-of-scope work: compare envelope, handoff, current Git state and
  changed paths before acceptance.
- Lingering processes: process-group deadline, termination grace and terminal
  heartbeat state.
- Double-counted tokens: sum terminal assistant message usage once and ignore
  streaming/turn duplicates.
- Scope inflation: no package upgrade, remote runner, GitHub mutation or OS
  sandbox claim.

## Rollback

Remove the supervisor facade/tool and v2 default. Direct bootstrap Pi launches
and metrics v1 remain valid. Private artifacts are inert and stay outside Git.
