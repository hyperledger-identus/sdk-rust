# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-17
Source retrieval date: 2026-09-17
Research blockers: none

## Problem and existing implementation

The hosted pull-request policy is split between `scripts/check-pr-policy.sh`
and `scripts/ci/contribution-policy.mjs`. Both accept environment variables
from GitHub, but the repository has no file-backed local command that applies
the two layers together before `gh pr create`. PR #319 therefore discovered
two deterministic body-shape failures only after remote creation.

Merges are currently supervisor commands outside the tracked facade. Passing a
shell-escaped body to `gh pr merge --body` allowed literal backslash-newline
characters into the squash commit message. Verification of the merged head,
merge commit, signature, and message remained manual and produced no closed
private receipt.

`scripts/worktree-lifecycle.mjs closeout-pr` correctly removes only an exact
worktree whose own PR merged to `develop`. It intentionally rejects PR #316
because that PR closed unmerged when replacement PR #319 integrated the owning
issue. The clean worktree is still registered even though its exact head is
preserved by `origin/codex/fix/issue-297`. Removing a worktree does not delete
that local or remote branch, so a closed proof can safely optimize local
capacity without claiming patch equivalence.

The effective Pi audit, `scripts/factory backlog-live`, and the content-
addressed package cache pass at `develop@81593db`. No runtime or package drift
supports an upgrade.

## Normative sources

Issue #320 and the sponsor direction in the current task define the tuning
outcome. ADRs 0003, 0004, 0108, and 0127, `CONTRIBUTING.md`, the canonical
`factory-operations` specification, `.factory-policy.json`, GitHub protected
merge behavior, and the existing supervisor/worktree/metrics contracts govern
implementation.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Upgrade Pi/Nix/Node/packages | `not-adopt` | Runtime and policy audits pass; no measured runtime defect exists. | A reproducible audit, launch, cache, or worker-protocol failure. |
| Duplicate PR regexes in a new script | `not-adopt` | Separate copies would drift from hosted behavior. | Hosted policy becomes an independently versioned external service. |
| Invoke both existing PR policy implementations locally | `adopt` | Reuses the exact code and diagnostics that CI runs. | The hosted flow changes to a single authoritative implementation. |
| Inline merge body shell text | `not-adopt` | Shell escaping already produced incorrect literal newline text. | Never; file-backed input is simpler and auditable. |
| File-backed guarded merge plus receipt | `adopt` | Preserves normal protection and closes exact-head evidence locally. | GitHub supplies a stronger immutable receipt automatically. |
| Prove replacement semantic equivalence automatically | `not-adopt` | Patch inclusion is not generally decidable after conflict resolution or extension. | A future content-manifest contract explicitly defines equivalence. |
| Recoverable superseded closeout | `adopt` | Exact remote-head preservation makes local worktree removal reversible without deleting refs. | Git hosting no longer preserves the validated ref. |

## Compatibility and dependency evidence

Public SDK API, wire format, persisted data, crate graph, features, toolchains,
targets, lockfiles, and consumer behavior are unchanged. The implementation is
repository-local Node/shell tooling using existing `git` and `gh` dependencies.
Rollback removes the new facade command and closeout mode while retaining all
historical PR, branch, metrics, and merge evidence.

## Security, privacy and maintenance evidence

Remote mutation remains opt-in through `--execute`; dry validation performs no
merge or cleanup. Merge eligibility requires the exact hosted PR head, open and
ready state, `develop` base, normal GitHub required-check success, and a safe
regular body file. The receipt contains identifiers, digests, timestamps, and
verification outcomes only—no token, prompt, transcript, raw check output, or
provider/model data. Superseded cleanup rejects dirty, current, primary,
locked, symlinked, wrong-head, wrong-base, unpreserved, or ambiguously linked
targets and never deletes a branch.

## Rejected or deferred candidates

Runtime upgrades, automatic semantic-equivalence claims, branch deletion,
repository settings, and release automation are rejected for this slice.
Release-candidate planning is assessed separately against `RELEASING.md`; no
publication or 0.1.0 promise is activated here.

## Open questions and blockers

No implementation blocker remains. GitHub is a network dependency for hosted
preflight and merge/closeout execution, so deterministic tests use bounded
fake `gh`/`git` fixtures and production commands fail closed on unavailable or
malformed hosted evidence.

## Evidence sources

- PR #319, superseded PR #316, issues #315/#297, and their exact hosted state.
- `scripts/check-pr-policy.sh`, `scripts/ci/contribution-policy.mjs`,
  `scripts/worktree-lifecycle.mjs`, and supervisor/metrics contracts at
  `develop@81593db3418d5b70990ec3c31ae8ddc3a4bc6edf`.
- `RELEASING.md`, ADR 0081, issue #276, and the support policy for the separate
  release-readiness assessment.

## Evidence commands

Commands run before implementation: `scripts/factory doctor`,
`./bootstrap.sh --audit-pi`, `scripts/factory backlog-live`, managed-worktree
audit, exact Git/PR inspection for #316/#319, and source inspection of the
policy/lifecycle scripts. Unrun before implementation: mutation tests, the
supervisor canary, and the compatible Nix closure; they are delivery tasks.
