# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-11
Source retrieval date: 2026-09-11
Research blockers: none

## Problem and existing implementation

At the exact base, the repository pins Pi 0.84.2, launches it through
`./bootstrap.sh --pi`, limits managed worktrees to two, and stores metrics v1
under the Git common directory. The worker instructions request a terminal
handoff, but no program validates it. Noninteractive Pi output is buffered, so
the supervisor has no independent liveness signal. Metrics v1 omits session,
turn, cache-token, phase, CI-attempt and separate disk counters.

The issue #259 live research canary launched through the repository bootstrap
with read-only tools and a private session directory. It completed in 14.31s
with a 275,808,256-byte maximum resident set. The persisted JSONL starts with a
Pi `session` record at format version 3, followed by message records. The JSON
event stream separately emits `turn_start`, tool lifecycle events and one
terminal assistant `message_end` per response. Usage uses non-overlapping
`input`, `output`, `cacheRead` and `cacheWrite` buckets. Counting terminal
assistant messages avoids double-counting streaming updates and `turn_end`.

## Normative sources

Repository authority is issue #259, ADR 0108, the accepted factory
specifications, `.factory-policy.json`, the preflight receipt and the pinned
Pi help/runtime. The Obsidian factory notes `02 Architecture and Runtime
Topology`, `05 Issue-to-PR Delivery Loop`, `10 Metrics Telemetry and
Retrospectives`, `12 Multi-session and Multi-engineer Operation`, `17 Runtime
Notes`, and the worker-handoff/metrics templates are guidance. They agree that
the supervisor owns admission, liveness, CI, publication and cleanup while the
worker owns one bounded task and terminal evidence.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Keep prose-only handoffs | `not-adopt` | Cannot reject stale identity, malformed evidence or buffered-worker ambiguity | Worker runtime supplies a separately authenticated typed handoff |
| Parse noninteractive stdout as the authoritative session | `not-adopt` | Bootstrap prefixes and streamed content mix control data with private messages | Pi removes the persisted session contract |
| Parse private persisted Pi v3 JSONL and retain counters only | `adopt` | Observed stable header and terminal usage records yield exact counters without retaining content | Pi changes the session major or terminal event semantics |
| Let the worker self-report counters | `not-adopt` | Prior runs reported unavailable values while persisted sessions had exact data | Runtime stops persisting exact usage |
| Use worker prose as heartbeat | `not-adopt` | Print mode buffers output and makes silence ambiguous | Pi offers a stable content-free liveness API |
| Supervisor-owned process heartbeat and deadline | `adopt` | Liveness does not depend on model output and is bounded by OS process state | A portable job supervisor replaces the local wrapper |
| Replace metrics v1 in place | `not-adopt` | Would invalidate existing private/public records | All retained v1 evidence expires |
| Validate both v1 and v2; create v2 by default | `adopt` | Preserves history while adding measured fields and explicit null reasons | A later versioned migration supersedes both |
| Claim an OS sandbox around implementation Pi | `not-applicable` | This slice constrains tools and validates effects but does not isolate an unrestricted shell kernel boundary | Remote workers or threat model require hostile-code isolation |

## Compatibility and dependency evidence

The implementation uses only Node 24 standard-library APIs already pinned in
the dev shell. It adds no Rust, npm or Nix dependency. Metrics v1 records and
their v1 public marker remain accepted and renderable; templates default to
v2. Pi session harvesting accepts only observed format version 3 and fails
closed on another major.

## Security, privacy and maintenance evidence

Invocation, heartbeat, session and handoff artifacts are private files below
the Git common directory with owner-only permissions. Inputs must be regular,
non-symlink files below the expected run directory and within byte bounds.
Session parsing reads content-bearing records but retains only counts; it never
emits message content, IDs, response IDs, cost, provider/model data, commands
or raw output. Metrics use a closed allowlist and explicit null reasons.

The wrapper verifies exact repository/issue/base/branch/head/receipt identity
before launch and validates changed paths and current head before accepting
handoff. Pi receives only its role's explicit tool list. This is an execution
and acceptance boundary, not a claim that allowed `bash` is an OS sandbox.

## Rejected or deferred candidates

MCP orchestration, remote runners, cross-host leases, a metrics database,
provider/model benchmarking and automated GitHub merge are deferred. Issue
#260 owns the Nix cache fast-lane correction; issue #261 owns retained
worktree classification.

## Open questions and blockers

No implementation blocker remains. Pi v3 is an observed runtime contract, so
unknown future formats must produce `null` with a reason rather than guessed
usage until separately researched.

## Evidence commands

Planned evidence includes strict OpenSpec/research/constraint readiness,
hermetic Node tests for envelope/handoff/session/timeout/privacy behavior,
metrics v1/v2 tests, `./bootstrap.sh --check`, file hygiene, a bounded
bootstrap-launched Pi canary and hosted `fast` CI.
