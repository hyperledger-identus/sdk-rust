# Design

## Context

The repository has the physical topology needed for agile development: one
required Linux `fast` workflow and a separate complete slow workflow. The gap
is semantic and operational. `production-ready` currently means only that the
PR requires `fast`, while `slowRecommended` is a list without a promotion
decision. Review budgets exist in JSON but agents can continue requesting and
fixing new findings indefinitely. Recent PRs demonstrated that repeated
exact-head invalidation dominates the underlying check duration.

## Decisions

### Two decisions, not two quality levels

The fast line answers: “may this bounded slice integrate into `develop` during
active development?” It keeps policy, OpenSpec, formatting, compilation,
strict Clippy, normal tests, and bounded first-party analysis. It does not
claim production portability or release fitness.

The slow line answers: “may this exact candidate be promoted toward production
or release?” It retains complete platform, target, binding, security,
conformance, coverage, performance, fuzz/sanitizer, deterministic artifact,
and receipt evidence. Slow is broader, not more correct; a failing fast line
can never be compensated by slow evidence.

### Local-first candidate batching

The supervisor performs focused local checks and a distinct local review before
the first candidate push. It batches related repairs and avoids using GitHub as
an edit-by-edit test runner. Initial guidance is one candidate push and no more
than one post-review remediation push. Exact-head evidence remains mandatory
for the final head.

### Review cutoff with severity escape hatch

One automatic discovery review runs only after local acceptance and the first
green head. The agent triages all findings together, then performs at most one
remediation round. Blocking severity, failed acceptance, introduced defects,
and security regressions remain in scope at any time. A new independent P2/P3
after the remediation cutoff receives a linked issue with evidence and does
not expand an otherwise eligible PR. This is a scope rule, not a defect waiver.

### Guidance rather than arbitrary size rejection

One slice owns one coherent behavior. Crossing 12 changed files or 1,000
non-generated changed lines requires a decomposition note in the target plan
or PR evidence. It does not automatically fail because migrations, fixtures,
or security changes can be cohesive. Metrics record the actual size so the
threshold can be tuned from evidence.

### Machine-readable plan and policy

The factory policy records lane purpose, required status, SLOs, included and
excluded evidence classes, review/remediation budgets, slice guidance, and
promotion invariants. `target-plan.mjs` emits separate `integration` and
`promotion` decisions while retaining existing `requiredPullRequestChecks`,
`slowRecommended`, and fail-closed behavior for compatibility. Focused tests
reject a second required PR lane, missing budgets, promotion without exact
evidence, and unknown-path relaxation.

The workflow jobs themselves are not expanded in this slice. A later measured
optimization may remove provably duplicate compilation or add safe target
selection only after it preserves the contract.

## Risks and mitigations

- Review cutoff could hide a defect: severity and acceptance failures override
  the budget, and follow-up applies only to independent non-blocking findings.
- Numeric guidance could be gamed: thresholds require explanation and metrics,
  not automatic approval.
- SLO variance could create noise: use execution time, a rolling sample, and a
  ten-minute trigger rather than failing individual jobs at eight minutes.
- Slow debt could be forgotten: promotion readiness remains false until an
  exact green receipt exists; weekly freshness remains separately audited.
- Two readiness states could confuse agents: target-plan output names both and
  operations documentation gives a single state transition.

## Rollback

Revert the additional policy fields, target-plan output/tests, ADR, and
handbook changes. Keep the physical fast and slow workflows and all historical
metrics/receipts unchanged.
