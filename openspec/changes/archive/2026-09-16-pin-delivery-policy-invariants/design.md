# Design

## Decision

Define the canonical blocker and slice-guidance values beside the validator in
`scripts/ci/target-plan.mjs` and compare arrays and scalar fields exactly.
Retain the existing validation order and public target-plan shape. Extend the
focused mutation suite so each independently meaningful drift is rejected.

Exact ordered array comparison is intentional. The order is part of stable
machine output and exact comparison rejects both missing and unexpected policy
semantics. Scalar equality pins the accepted advisory thresholds without
turning the thresholds into hard delivery gates.

## Risks and mitigations

- A future intentional policy adjustment initially fails: that is the desired
  issue/OpenSpec reminder, and the policy, validator, specification, and tests
  must move together.
- Tests could cover only one direction of threshold drift: mutations cover
  both lower and higher values for both fields.
- Tooling could change emitted plans: existing plan-shape assertions remain
  unchanged and focused tests verify the canonical policy still passes.

## Rollback

Revert the validator constants/comparisons and mutation cases atomically. The
checked-in policy, workflows, public SDK, and historical receipts are not
modified by rollback.
