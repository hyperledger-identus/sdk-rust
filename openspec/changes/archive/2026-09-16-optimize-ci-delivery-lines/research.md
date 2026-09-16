# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-16
Source retrieval date: 2026-09-16
Research blockers: none

## Problem and existing implementation

ADR 0081 created one Rust 1.98 Linux fast gate and a complete weekly/manual
slow matrix. ADR 0120 activated the slow scheduler from protected `develop`.
The machine policy already says `maximumAutomaticReviewRounds: 1`, but neither
the operating contract nor target plan turns that value into a stop rule.
`scripts/ci/target-plan.mjs` distinguishes required PR checks from recommended
slow targets but does not distinguish integration readiness from promotion
readiness or expose latency/slice budgets.

## Normative sources

Project-sponsor direction in issue #303 and discussion #302 controls the
delivery objective. ADRs 0081 and 0120, the canonical `factory-operations`
specification, `.factory-policy.json`, the protected `develop` workflow set,
and exact GitHub PR/Actions metadata are the current implementation and
normative repository evidence.

## Measured evidence

GitHub PR and Actions metadata were retrieved for exact repository objects on
2026-09-16:

| Slice | PR shape | Delivery evidence |
| --- | --- | --- |
| #300 | 25 files, +3,784/-1,377, 33 commits | 3h47m elapsed; 13 fast attempts, 12 green and 1 cancelled; 13 review events; fast duration 6m31s to 8m00s; 95m aggregate fast critical paths. |
| #296 | 29 files, +2,609/-56, 15 commits | 10h43m elapsed; 9 fast attempts, 8 green and 1 cancelled; 18 review events; 50m22s aggregate fast critical paths. |
| #274 | 69 files, +4,720/-105, 30 commits | 2h26m elapsed; 20 review events. |
| slow run 35006444994 | exact `develop@66ec2b9` | Complete candidate, coverage, performance, browser, Linux/macOS, Apple/Android and receipt evidence passed in 31m15s. |

The fast workflow duration is material but not the dominant multiplier. A
seven-minute check executed thirteen times costs more elapsed and compute time
than a single check improved by one minute. Review and push policy therefore
has higher immediate leverage than weakening test content.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| One required Linux fast line | `retain-local` | Existing exact Rust/Nix gate gives a 6.5–8 minute signal and protects ordinary integration. | p95 exceeds ten minutes for two rolling samples or misses material regressions. |
| Complete matrix on every PR | `not-adopt` | Adds tens of minutes and multiple platforms to unstable feature heads. | Published compatibility promise requires per-change platform proof. |
| Path-selected reduced tests | `spike` later | Current full normal workspace suite remains within the initial p95 target; selection errors add risk and complexity. | Workspace growth breaks the ten-minute trigger. |
| Local-first batching and bounded review | `adopt` | Directly addresses 9–13 attempts and open-ended review without reducing technical evidence. | Blocking defects escape because review stopped too early. |
| Exact-SHA slow promotion line | `retain-local` | A 31-minute complete run found real packaging/platform defects and is appropriate at stable candidate boundaries. | Evidence becomes non-reproducible or too stale for release use. |
| Hard file/line rejection | `not-adopt` | Generated migrations and cohesive security changes can legitimately exceed a numeric threshold. | Repeated oversized slices ignore non-blocking decomposition guidance. |

## Compatibility and dependency evidence

Public and wire compatibility impact is none. No dependency, feature, target,
MSRV, facade, license, provenance, or resolved dependency cone changes. The
implementation edits repository tooling and documentation only and rolls back
atomically without consumer migration.

## Security, privacy and maintenance evidence

This change adds no dependency, runtime code, network permission, public API,
wire format, persisted data, unsafe code, target, or compiler. It retains all
security and exact-head gates. The risk is process ambiguity: a review cutoff
must not hide a regression. The contract therefore keeps P0/P1, acceptance
failures, introduced defects, and security regressions blocking regardless of
round count; only independent non-blocking discoveries move to follow-up.
No unsafe or native code, credential, telemetry, private content, or new
supply-chain input is introduced. Maintenance, release, and security posture
remains governed by exact Nix/Cargo/action pins and the slow promotion receipt.

## Rejected or deferred candidates

Complete per-PR matrices and hard size rejection are rejected for the reasons
above. Diff-selected Rust tests are deferred until measured workspace growth
crosses the optimization trigger and a safe selector can be independently
proved. Changing Rust, Nix, cache write authority, or issue #276 scheduling is
out of scope.

## Open questions and blockers

No blocker remains. The initial fast SLO is an observed baseline, not a
release promise. Production promotion freshness remains governed by the exact
slow receipt and issue #276's natural-schedule acceptance; this slice does not
claim that scheduled acceptance early.

## Evidence sources

- GitHub PRs #274, #296, and #300 and their review/check metadata.
- GitHub Actions run 35006444994 and issue #276 canary report.
- ADRs 0081, 0111, and 0120; `.factory-policy.json`; factory operations and
  metrics contracts; target-plan implementation and tests.
- Retrospective discussion #302 and implementation issue #303.

## Evidence commands

Commands run: `gh pr view` for #274/#296/#300, `gh run list` for their exact
branches and the `slow` workflow, repository searches over workflows/policy,
and inspection of ADRs/specs/tests at `develop@1d52831`. Unrun before
implementation: mutation tests and full fast-equivalent Nix checks; they are
required delivery evidence after the planning receipt.
