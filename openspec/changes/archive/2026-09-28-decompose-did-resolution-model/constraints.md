# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/405
Constraint blockers: none

## Existing entries affected

`SDK-SEC-003` continues to require explicit resource limits.
`SDK-LIM-007` continues to assign outer allocation and caller-budgeted work
outside typed enforcement. ADR 0115 continues to require semantic ownership and
a touched-scope code-health ratchet. Issue #50 continues to prohibit an implied
single-flight requirement. Rust/MSRV, dependency, support, target, publication,
and protocol-version constraints are unchanged.

## Introduced or changed constraints

No effective product constraint changes. Internally, resolution values,
operation metadata, document metadata, result envelopes, dereferencing models,
and raw-wire preflight must each have one private owner. Sibling access must be
the minimum needed to preserve current validation and cleanup composition.

## Introduced or changed limitations

None. This refactor does not close or broaden existing transport, allocation,
protocol-version, native-content, or concurrent-fill limitations.

## Consumer and product impact

Consumers observe no API, type identity, accepted/rejected input, error,
serialized form, resource budget, cleanup, cache/cancellation, runtime,
dependency, feature, MSRV, target, performance-budget, or certification change.
Maintainers gain responsibility-cohesive review surfaces below the attention
threshold.

## Activation and rollback

Activation requires issue #405's signed/DCO PR, exact pre/post characterization,
base/head public and wire-contract comparison, error and code-health evidence,
distinct architecture/security/Rust review, relevant portable/Nix gates, and
green protected CI. Rollback recombines private modules without consumer or
data migration.

## Evidence

The issue, Discussion #399, this OpenSpec change, DID test corpus, public export
inventory, immutable error golden, source distribution, code-health report,
strict Rust/factory/Nix gates, exact-diff review, and hosted exact-head receipt
are required evidence.
