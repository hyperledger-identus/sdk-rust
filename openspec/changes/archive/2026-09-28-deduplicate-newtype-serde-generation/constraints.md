# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/402
Constraint blockers: none

## Existing entries affected

`SDK-SEC-001` continues to forbid unsafe first-party code, and ADR 0088 keeps
generated output behind the syntax-aware safety validator. `SDK-DEP-001`, the
Rust 1.98.1 support contract, target support, and issue-first delivery remain
unchanged. ADR 0115 requires a real semantic ratchet rather than cosmetic file
movement.

## Introduced or changed constraints

No effective constraint changes. The private helper must preserve all emitted
tokens that define public behavior and must not become a public macro or
runtime abstraction. String/numeric category-specific behavior stays in its
own module.

## Introduced or changed limitations

None. The existing macro supports only its documented string, bytes, and
numeric categories. This slice neither expands nor narrows that set and does
not claim that unrelated derive hotspots are resolved.

## Consumer and product impact

Consumers observe no public API, wire, error, dependency, feature, MSRV,
target, or performance change. Maintainers receive one ownership point for
scalar serde templates and stronger regression evidence.

## Activation and rollback

Activation requires issue #402's signed/DCO PR, exact generated-behavior
evidence, code-health refresh, distinct local review, and green protected CI.
Rollback restores the two private copies atomically; no migration is required.

## Evidence

The issue, ADR 0161, OpenSpec delta, positive expansion/runtime tests,
trybuild suite, output-safety checks, code-health report, strict Rust checks,
and hosted exact-head receipt are required evidence.
