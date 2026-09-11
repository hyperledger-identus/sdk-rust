# Constraints and limitations

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/259
Constraint blockers: none

## Existing entries affected

- `SDK-AGENT-001` and `SDK-AGENT-002`: the supervisor admits one bounded
  worker in one managed worktree and retains protected authority.
- `SDK-COMPAT-002`, `SDK-COMPAT-004` and `SDK-COMPAT-005`: Rust 1.98.1 and the
  pinned Nix/Pi/Node runtime remain unchanged.
- `SDK-LIM-006`: release, publication and `main` remain inactive.

## Introduced or changed constraints

An accepted worker run is bound to one exact repository, issue, `origin/develop`
base, branch/head, active OpenSpec receipt, role, task and path/tool allowlist.
The supervisor owns process deadlines, liveness, handoff validation, metrics,
CI observation and any later GitHub mutation. Unknown Pi session formats and
unavailable counters fail closed to explicit `null` values with reasons.

## Introduced or changed limitations

The local wrapper does not claim hostile-code or kernel isolation. A role that
needs `bash` can execute commands available to that local operator; acceptance
therefore also validates Git head, status and changed paths. Cross-host leases,
remote execution and automated handoff repair remain future work.

Pi session format version 3 is the only harvested format in this change.
Metrics v1 remains supported but cannot express v2 detail.

## Consumer and product impact

No Rust API, wire behavior, feature, target, compiler floor, dependency cone or
downstream repository changes. Maintainers gain a deterministic local boundary
for Desktop-supervised Pi work and more truthful factory telemetry.

## Activation and rollback

The behavior activates when the issue #259 PR merges to `develop`. Existing
`./bootstrap.sh --pi` remains usable directly. Rollback removes the new
supervisor command and defaults metrics templates back to v1; retained v1
records stay valid and private run artifacts require no migration. `main`
remains reserved.

## Evidence

Tests will prove exact identity, stale-head rejection, allowed-path enforcement,
oversize/symlink/malformed rejection, timeout/heartbeat state, session counter
deduplication, explicit unavailable reasons and v1/v2 metrics compatibility.
