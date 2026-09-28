# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/408
Constraint blockers: none

## Existing entries affected

`SDK-SEC-003` continues to require explicit resource limits. ADR 0115 continues
to require semantic ownership and a touched-scope code-health ratchet. DID
Core neutrality, public-only verification material, redaction, iterative
cleanup, Rust/MSRV, dependency, support, target, publication, and unsafe-code
constraints are unchanged.

## Introduced or changed constraints

No effective product constraint changes. Internally, cardinality, recursive
extension data, verification material, services, and aggregate document
validation must have cohesive private owners. Sibling access must remain the
minimum needed for validated composition and iterative rejection cleanup.

## Introduced or changed limitations

None. This refactor does not close or broaden DID methods, suites, JSON-LD,
resolution, dereferencing, authorization, trust, persistence, transport, or
runtime limitations.

## Consumer and product impact

Consumers observe no API, wire, serde, accepted/rejected input, error, resource
budget, cleanup, debug form, dependency, feature, MSRV, target,
performance-budget, or certification change. Maintainers gain cohesive private
review surfaces below the attention threshold.

## Activation and rollback

Activation requires the merged issue #407 `develop` base, issue #408's
exact-head preflight, signed/DCO PR, pre/post characterization, public/wire/
source comparison, immutable error and code-health evidence, distinct review,
portable/Nix gates, and green protected CI. Rollback recombines private modules
without consumer, adapter, data, or persistence migration.

## Evidence

The issue, Discussion #399, this OpenSpec change, DID document/hardening corpus,
public export inventory, immutable error golden, source distribution,
code-health report, strict Rust/factory/Nix gates, exact-diff review, and hosted
exact-head receipt are required evidence.
