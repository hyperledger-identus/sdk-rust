# Constraints and limitations

Impact class: routine
Decision status: not-required
Decision reference: not-required
Constraint blockers: none

## Existing entries affected

- `SDK-RELEASE-001`: canonical DID publication denial remains effective.
- `SDK-COMPAT-002` through `SDK-COMPAT-005`: compiler/target promises remain
  unchanged and are only described, not activated.
- ADR 0132 continues to govern reproducible static site publication.
- ADRs 0153/0154 continue to govern DID train identity and first API origin.

## Introduced or changed constraints

None. The site is reconciled with already accepted repository state.

## Introduced or changed limitations

No new limitation is introduced. Existing candidate-only, first-API-origin,
local-SBOM, platform, provenance, registry and publication limitations become
more visible on the public review surface.

## Consumer and product impact

Engineers receive accurate review and source-evaluation guidance. No consumer,
product, public API, wire behavior, dependency, target or external artifact is
changed.

## Activation and rollback

Activation requires exact-base OpenSpec/preflight, deterministic site build,
diagram/link validation, local claim-boundary review and exact-head hosted CI.
Rollback reverts additive site content and reconciled prose only.

## Evidence

Issues #381/#386, closed #382/#384, ADRs 0132/0153/0154, the DID descriptor,
archived receipts, site build output and protected PR CI provide evidence.
