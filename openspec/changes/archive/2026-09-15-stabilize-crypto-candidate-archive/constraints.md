# Constraints and limitations

Impact class: routine
Decision status: not-required
Decision reference: not-required
Constraint blockers: none

## Existing entries affected

- `SDK-DELIVERY-001`: the defect repair remains issue-linked, spec-driven,
  reviewed, tested, and CI-gated.
- `SDK-LIM-009`: the repair restores the already-effective manual/weekly slow
  evidence path without changing cadence, merge gates, or release policy.
- `SDK-REPO-002`: publication and `main` delivery remain prohibited.

## Introduced or changed constraints

None. The existing byte-identical candidate-archive requirement is enforced as
originally intended for outputs located inside the repository.

## Introduced or changed limitations

None. Build scratch remains host-local and ephemeral; the candidate remains
unpublished evidence rather than registry, release, target-support, or natural
schedule evidence.

## Consumer and product impact

No consumer or product changes. Maintainers regain a truthful reproducible
candidate artifact in the weekly/manual slow lane.

## Activation and rollback

The routine repair activates when its issue-linked PR merges to `develop`.
Rollback reverts the scratch/output separation and its tests, but would restore
the observed canary failure; no external artifact or consumer data changes.

## Evidence

Planning preflight, focused scratch-boundary tests, double Cargo assembly,
candidate package evidence, full local factory/Rust checks, protected PR CI,
and a successful manual canary form the evidence.
