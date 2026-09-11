# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-11
Source retrieval date: 2026-09-11
Research blockers: none

## Problem and existing implementation

At the exact base, `bootstrap.sh --check` runs `scripts/factory check` and 14
Node operational tests inside `nix develop`. It does not run
`audit-pi.mjs`. The direct `scripts/factory audit` dispatch calls host `node`
without entering Nix, so the full audit correctly reports that host Pi and
`RUSTC_WRAPPER` do not match the pinned environment.

The explicit `./bootstrap.sh --audit-pi --json` path enters Nix and passes with
Pi 0.84.2, Node 24.19.0, npm 11.17.0, configured repository hooks, commit
signing, zero managed worktrees and a ready content-addressed package cache.
`./bootstrap.sh --pi --version` also returns 0.84.2 and leaves Git clean.

## Normative sources

The controlling sources are issue #257, retrospective discussion #256, ADR
0108, the accepted `ai-software-factory` and `factory-operations`
specifications, `.factory-policy.json`, and the exact runtime in `flake.lock`.
This is repository-local orchestration; no external implementation is adopted.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Document bootstrap-only audit usage | `not-adopt` | Leaves a documented portable factory command misleading outside Nix | Direct command is removed from the public facade |
| Make only bootstrap check call audit | `not-adopt` | Closes check coverage but retains direct-command friction | Factory audit becomes intentionally in-shell-only |
| Auto-enter Nix for direct audit and compose it into bootstrap check | `adopt` | Reuses the pinned environment, preserves one implementation and makes both public paths truthful | Nix devshell startup becomes a measured dominant cost |
| Reimplement runtime discovery on the host | `not-adopt` | Duplicates Nix resolution and could accept an unpinned Pi | Repository drops Nix as the harness authority |
| Upgrade Pi or packages | `not-applicable` | No version defect was observed | Separate dependency research finds a version-specific blocker |

## Compatibility and dependency evidence

The implementation does not change a public Rust API, dependency, pinned
runtime version or package closure. Nix is already the bootstrap authority and
is discovered through the existing fallback path, so current Linux and macOS
operator entrypoints retain their compatibility contract.

## Security, privacy and maintenance evidence

The implementation changes only shell routing. Existing audit checks, JSON
shape, exit status and privacy boundaries remain authoritative. Nix is already
required by bootstrap. No auth, prompt, transcript, provider or model data is
read.

Re-entry is bounded by the Nix shell marker: an outside invocation enters the
repository flake once; an invocation in any Nix shell runs the audit directly,
allowing the audit itself to report a wrong shell. Argument boundaries remain
an argv array and are not evaluated by an intermediate shell.

## Rejected or deferred candidates

Metrics schema expansion, automatic worktree cleanup, slow-evidence freshness,
Pi progress streaming and all runtime upgrades are deferred to separate
evidence-backed slices. The current three private metric records remain valid
and their unavailable counters remain `null`.

## Open questions and blockers

No implementation blocker remains. Nix startup latency is accepted as the
cost of a truthful full audit and should be measured rather than guessed.

## Evidence commands

Planned evidence includes strict OpenSpec validation, hermetic shell-routing
fixtures, the Node operational suite, `./bootstrap.sh --check`, direct
outside-shell and in-shell audit invocations, `./bootstrap.sh --pi --version`,
file hygiene, factory contract and hosted `fast` CI.
