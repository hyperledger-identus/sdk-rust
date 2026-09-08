# Constraint and limitation impact

Impact class: routine
Decision status: not-required
Decision reference: not-required
Constraint blockers: none

## Existing entries affected

`SDK-DELIVERY-001` remains effective and is strengthened at its archive
evidence edge: a successful receipt must reflect the promised filesystem state,
not only a child process exit code. No machine-readable constraint value,
authority, owner, activation rule or review trigger changes.

## Introduced or changed constraints

No cross-cutting constraint is introduced. The canonical factory contract gains
a local postcondition for its existing archive operation.

## Introduced or changed limitations

The wrapper detects but does not automatically repair partial OpenSpec
mutation. It also uses the local ISO date captured before mutation, so a command
that crosses midnight can fail conservatively despite a valid next-day archive.
Both cases preserve the fail-closed delivery posture and have explicit
reconsideration triggers in the research record.

## Consumer and product impact

There is no SDK consumer, protocol, API, dependency or data-format impact.
Repository agents receive a truthful nonzero result when an archive did not
complete or collided with an existing destination.

## Activation and rollback

The behavior activates when issue #194's pull request merges into `develop`.
Rollback restores the exit-code-only wrapper and its false-success risk; no
runtime or data migration is required.

## Evidence

Acceptance requires hermetic no-op, collision and successful-move contract
tests; shell lint; strict factory/OpenSpec validation; exact-diff review; full
local Nix evidence; and hosted Linux CI.
