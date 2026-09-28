# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/404
Constraint blockers: none

## Existing entries affected

`SDK-SEC-003` continues to require explicit resource limits.
`SDK-LIM-007` continues to assign outer preallocation, caller-budgeted work,
and named compatibility surfaces outside typed enforcement. ADR 0115 continues
to require semantic ownership and a touched-scope ratchet. Rust/MSRV,
dependency, target, publication, and protocol-support entries are unchanged.

## Introduced or changed constraints

No effective constraint changes. Inside `identus-oid4vci`, numeric default
policy must have one private definition site. Public type mechanics may be
separated only by existing protocol lifecycle, and the crate-root surface
must remain unchanged.

## Introduced or changed limitations

None. The refactor neither closes nor broadens `SDK-LIM-007`, and makes no new
claim about transport allocation, deserializer work, or unsupported inputs.

## Consumer and product impact

Consumers observe no API, type identity, default value, validation, error,
wire, allocation, dependency, feature, MSRV, target, performance-budget, or
certification change. Maintainers gain a central numeric-policy review surface
and smaller lifecycle-cohesive type modules.

## Activation and rollback

Activation requires issue #404's signed/DCO PR, exact pre/post
characterization, base/head API and numeric-policy comparison, code-health
evidence, distinct architecture/security review, relevant Nix gates, and green
protected CI. Rollback recombines private modules and inlines unchanged values;
no consumer migration or data transition is required.

## Evidence

The issue, Discussion #399, this OpenSpec change, pre/post OID4VCI suites,
immutable OID4VCI error golden, exact public export/signature and default-value
inventory, code-health report, strict Rust/factory/Nix gates, exact-diff
review, and hosted exact-head receipt are required evidence.
