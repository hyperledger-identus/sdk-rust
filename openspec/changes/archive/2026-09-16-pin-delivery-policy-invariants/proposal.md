# Pin delivery-policy invariants

## Why

PR #304 established the fast integration and slow production-promotion
contract, but exact-head review found that the validator accepts reordered or
additional slow blockers and accepts any positive decomposition thresholds.
The checked-in policy is correct; its validator does not yet prevent drift.

## What changes

- Require the complete ordered slow-blocker set: `production-promotion`,
  `publication`, and `release-preparation`.
- Require decomposition guidance of exactly 12 changed files and 1,000 changed
  text lines while retaining `decomposition-note` as the advisory action.
- Add mutation tests for missing, reordered, additional, lower, and higher
  values.

## Capabilities

### Modified capabilities

- `factory-operations`: makes the already accepted delivery-policy values
  fail-closed invariants.

## Non-goals

- No workflow, Rust, Nix, public SDK, dependency, review-budget, or lane change.
- No hard rejection based only on slice size.
- No production promotion, publication, or `main` activation.

## Delivery

Issue #305 owns this tooling-only slice. It targets protected `develop` and
retains the single required Linux `fast` gate.
