# Optimize CI delivery lines

## Why

The repository already requires one Linux `fast` status and runs complete
weekly/manual production evidence, but recent slices still took hours to land.
PR #300 paid for thirteen fast attempts and thirteen review events; PR #296
paid for nine fast attempts and eighteen review events. The fast check itself
was bounded. Oversized slices, granular pushes, and review scope expansion
multiplied it. Issue #303 turns discussion #302 into an executable delivery
contract without adding per-PR matrices.

## What changes

- Name `fast` as the active-development integration line and define its scope,
  latency SLO, local-first batching, slice guidance, and optimization trigger.
- Name `slow` as the exact-revision production-promotion line and define the
  evidence required before release or a production-support claim.
- Bound automatic review to one discovery round and one remediation round;
  route later independent P2/P3 discoveries to linked follow-up issues.
- Make lane semantics and budgets machine-readable and expose integration and
  promotion readiness separately in target plans.
- Update the supervisor and one-slice operating contract so a sound green PR
  is not expanded indefinitely.

## Capabilities

### Modified capabilities

- `factory-operations`: makes the existing fast/slow topology operational for
  review, push, slice, metrics, and promotion decisions.

## Non-goals

- No new required PR compiler, platform, binding, coverage, fuzz, sanitizer,
  performance, or release job.
- No waiver of correctness, security, issue linkage, OpenSpec, DCO, signature,
  exact-head, or blocking-review requirements.
- No release, publication, `main` activation, or duplicate scheduler work from
  issue #276.

## Delivery

Issue #303 owns the change. Discussion #302 is the retrospective record. The
slice targets protected `develop` and must preserve the single required `fast`
status while keeping slow failures visible production debt.
