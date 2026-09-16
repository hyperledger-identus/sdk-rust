# Constraints and limitations

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/305
Constraint blockers: none

## Existing entries affected

- `SDK-RUST-001`, `SDK-COMPAT-005`, and the Rust 1.98.1 pin are unchanged.
- `SDK-LIM-004` continues to describe the temporary fast/slow split.
- `SDK-SUPPLY-001` remains unchanged; no dependency or action pin changes.
- ADR 0127 remains the decision authority for the three slow blockers and the
  advisory 12-file/1,000-line decomposition thresholds.

## Introduced or changed constraints

The policy validator SHALL require exactly, in order,
`production-promotion`, `publication`, and `release-preparation` as slow
blockers. It SHALL require exactly 12 changed files, 1,000 changed text lines,
and `decomposition-note` as slice guidance. Missing, reordered, additional,
higher, or lower values SHALL fail closed.

## Introduced or changed limitations

The thresholds remain guidance rather than a merge gate and do not determine
correctness. Exact equality intentionally requires a future accepted policy
change to update validation and tests.

## Consumer and product impact

No SDK consumer or product capability changes. Repository contributors receive
an earlier fail-closed signal when machine policy drifts from accepted values.

## Activation and rollback

Activation requires focused mutation tests, repository health checks, signed
review, and merge into `develop`. Rollback reverts the comparisons and tests
together; no consumer migration or retained-data change is involved.

## Evidence

Evidence is issue #305, the PR #304 review findings, ADR 0127, the checked-in
policy, target-plan validator and focused tests. No constraint blocker remains.
