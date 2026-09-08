# Constraint and limitation impact

Impact class: routine
Decision status: not-required
Decision reference: not-required
Constraint blockers: none

## Existing entries affected

`SDK-DELIVERY-001` remains effective and is corrected at its archive receipt
edge. Qualifying work still uses the same issue-linked, spec-driven, reviewed
and CI-gated lifecycle; the receipt no longer assumes that the host and
OpenSpec share a calendar date.

## Introduced or changed constraints

No cross-cutting constraint is introduced. The existing archive success
condition is made timezone-independent and unambiguous.

## Introduced or changed limitations

The facade still detects rather than repairs partial OpenSpec mutation. It also
does not support concurrent archive mutations in one worktree: zero or multiple
new matching entries fail closed. These are existing operational boundaries,
not new SDK consumer limitations.

## Consumer and product impact

No SDK consumer, product, chain, protocol, API, dependency, target or data
format changes. Repository agents avoid a false failure after a valid archive
at a timezone or date-rollover boundary.

## Activation and rollback

The correction activates when issue #218's pull request merges to `develop`.
Rollback restores the host-local predicted path and its reproduced false
failure risk; no runtime or data migration is involved.

## Evidence

Acceptance requires deterministic local/OpenSpec date-disagreement coverage,
the existing collision/no-op/incomplete/canonical protections, a new symlink
and ambiguity assertion where applicable, shell lint, strict OpenSpec and
factory validation, exact-diff review and hosted Ubuntu fast CI.
