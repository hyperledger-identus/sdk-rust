# Factory retrospective: M5 and NeoPRISM adoption handoff

**Date:** 2026-10-29

**SDK candidate:** `develop@c429ac494a6eaeb2e22c0b38a5b1d70c0eb6ff62`

**Slow run:** [40888621367](https://github.com/hyperledger-identus/sdk-rust/actions/runs/40888621367)
**NeoPRISM gate:** [PR #348](https://github.com/hyperledger-identus/neoprism/pull/348)

## Outcome

The SDK candidate passed a complete exact-SHA slow line on attempt 1 after two
source-distribution defects were reproduced and repaired locally. All eleven
hosted jobs passed; the four DID matrix lanes were clean and used one frozen
staged lock. NeoPRISM's prerequisite packaging fix is independently green.
Both promotion paths are now waiting on human review rather than machine
evidence.

## What worked

| Practice | Evidence | Keep |
| --- | --- | --- |
| Local-first source-distribution reproduction | The missing crypto fixture and incompatible API-origin path were repaired before the final slow run. | Run candidate archives and removal mutations locally before expensive promotion evidence. |
| Exact identity receipts | SHA, run attempt, receipt hash, matrix hash, source cleanliness, and staged lock are bound in issue #388. | Keep exact-head receipts as the promotion boundary. |
| Fast/slow separation | Ordinary integration continued while the exhaustive candidate line remained a release gate. | Keep one fast PR line and weekly/manual slow promotion evidence. |
| Codex fallback review | Review continued when another provider was quota-limited. | Keep Claude-or-Codex fallback as an availability rule. |
| Quiet human-gate monitoring | Unchanged approval polls produced no user notification. | Notify only on approval, failure, or required action. |

## Friction and disposition

| Friction | Root cause | Disposition |
| --- | --- | --- |
| Issue #388 closed and had to be reopened after PR #585 | PR policy required the implementation PR to close its branch issue even though slow evidence and approval were post-merge acceptance. | Issue #291 / ADR 0198 adds explicit evidence-bound deferred linkage. |
| First slow attempt exposed an archive-only missing fixture | Repository tests passed while the extracted unpublished package lacked a required conformance packet. | Repaired by #586/#588; retain isolated candidate archive tests before promotion. |
| API-origin compatibility needed a synthetic-removal proof | Additive current API and published-origin compatibility were not independently demonstrated. | Repaired by #587/#588; retain positive and removal mutation evidence. |
| Two human approvals serialize otherwise-green work | Branch and release governance deliberately require independent human judgment. | Keep the gate; move independent work locally and back off unchanged polling. |
| The primary checkout follows intentionally sparse `main` and has no factory entrypoint | Managed worktree creation currently assumes another current `develop`-derived checkout exists. | Record as a separate bootstrap/control-plane follow-up; do not widen #291. |

## Factory change from this retrospective

Ordinary pull requests keep closing their issue. A genuinely post-merge
acceptance PR can instead declare `deferred`, reference the issue, and bind one
strict archived OpenSpec sidecar containing the matching issue, rationale, and
remaining acceptance IDs. Protected-base code validates the exact-head blob as
untrusted bounded data. The merge does not close the issue and the sidecar does
not waive its remaining gates.

## Next measurements

- Count avoidable close/reopen transitions after activation; target zero.
- Record deferred PRs and time from merge to final evidence without treating
  approval wait as implementation time.
- Measure unchanged approval polls and increase backoff when no transition
  occurs.
- Exercise factory bootstrap from a fresh sparse-primary environment before
  changing the intentionally empty `main` policy.

## Non-actions

No workflow was dispatched or rerun, no branch protection changed, no release
or package publication was authorized, and no downstream repository was
mutated by this retrospective.
