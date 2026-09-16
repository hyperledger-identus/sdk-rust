# ADR 0127: separate fast integration from slow production promotion

- **Status:** Accepted by project-sponsor direction
- **Date:** 2026-09-16
- **Issue:** [#303](https://github.com/hyperledger-identus/sdk-rust/issues/303)
- **Discussion:** [#302](https://github.com/hyperledger-identus/sdk-rust/discussions/302)
- **Refines:** ADR 0081 and ADR 0120
- **Review no later than:** 2026-12-08 and before any release candidate

## Context

ADR 0081 correctly removed three per-PR compiler/matrix classes in favor of
one Rust 1.98 Linux fast gate and weekly/manual complete evidence. ADR 0120 made
the slow scheduler executable from protected `develop`. Recent slices still
landed slowly because delivery behavior did not protect the split.

PR #300 used 33 commits, 13 fast attempts and 13 review events for a 25-file
change. Its successful fast executions took 6m31s–8m00s, but aggregate fast
critical paths consumed 95 minutes. PR #296 used 9 fast attempts and 18 review
events. A complete exact-SHA slow canary passed in 31m15s and previously found
real packaging, NDK and runtime-topology defects. The complete line is
valuable; it is simply the wrong inner-loop decision.

## Decision

1. `fast` is the active-development integration line. Exactly one Linux status
   remains the required Rust/factory PR gate for `develop`.
2. Fast retains policy, OpenSpec, formatting, normal workspace build, strict
   Clippy, tests and bounded first-party analysis in pinned Rust 1.98/Nix.
3. Fast has an initial observational SLO of p50 ≤ 6 minutes and p95 ≤ 8
   minutes. A comparable rolling p95 above 10 minutes creates a focused
   optimization issue; individual eight-minute runs do not fail solely for
   latency.
4. `slow` is the production-promotion line. It runs weekly from protected
   `develop`, manually for stable candidates, and before release or a
   production-support claim. It remains exact-SHA and artifact/receipt bound.
5. Slow failures block production promotion, publication and release
   preparation. They remain visible debt but do not add a complete matrix to
   every PR or retroactively invalidate unrelated green integrations.
6. Agents run focused local checks and local review before the first candidate
   push. Related fixes are batched rather than paying hosted setup per edit.
7. One automatic discovery review plus one remediation round is the normal
   budget. P0/P1 findings, security regressions, introduced defects and failed
   acceptance criteria override the budget. Later independent non-blocking
   P2/P3 findings become linked follow-up issues.
8. One slice owns one coherent behavior. More than 12 changed files or 1,000
   non-generated changed lines requires a decomposition note, not automatic
   approval or rejection.
9. Factory policy and target plans expose both integration and promotion
   readiness while retaining existing fields for compatibility.

## Consequences

- Prototype and feature delivery gets one clear merge signal in minutes.
- Production readiness gets stronger, exact-revision evidence without
  pretending every moving feature head is a release candidate.
- Review remains capable of blocking real defects but cannot expand forever
  through unrelated hardening discoveries.
- The process measures attempts, post-green pushes and review rounds alongside
  raw CI duration, so optimization targets the actual bottleneck.
- Large cohesive security/migration work is possible but must explain why it
  cannot be split.

## Alternatives rejected

### Run the complete matrix on every pull request

This would turn a roughly seven-minute integration decision into a 30-plus
minute platform decision and would repeat expensive evidence on unstable heads.

### Remove tests from fast

The observed fast duration is within the initial p95 target. Repeated pushes
and review expansion are the larger multiplier, so evidence removal is not
justified.

### Enforce a hard line/file cap

Numeric caps are easy to game and can split cohesive migrations or fixtures.
An explicit decomposition note plus measured outcomes is more truthful.

### Treat every new review observation as same-PR work

This produced unbounded review loops. Severity and acceptance relevance, not
discovery time alone, decide whether a finding blocks the current slice.

## Verification and rollback

Policy and target-plan tests bind lane names, one required PR status, budgets,
SLOs, promotion invariants and fail-closed routing. Documentation/factory checks
bind the agent flow. Rollback removes the additional semantics and restores the
prior target-plan shape; physical fast/slow workflows and historical receipts
remain unchanged.
