# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/459
Constraint blockers: none

## Existing entries affected

ADR 0115 continues to require semantic ownership and a touched-scope
code-health ratchet. DID registration module ownership continues to require the
same terminal/job, continuation, method, DID/document, metadata, handle, and
wait invariants. Redaction, resource, Rust/MSRV, dependency, chain-neutrality,
support, target, publication, unsafe-code, and spec-driven delivery constraints
remain unchanged.

## Introduced or changed constraints

No product constraint changes. Internally, one private borrowed result
validator must preserve the exact phase and first-error order. Lifecycle-specific
methods must remain cohesive and may not expose or retain private validation
state.

## Introduced or changed limitations

None. The slice does not add lifecycle states, recovery, polling, cancellation,
network behavior, public material, adapter policy, or new error detail.

## Consumer and product impact

Consumers observe no API, accepted/rejected result, retained value, exact error,
debug output, Serde, allocation, dependency, feature, MSRV, target,
performance-budget, certification, or adapter change.

## Activation and rollback

Activation requires exact-head preflight, pre-change combined-fault
characterization, signed/DCO commits, public/source comparison, code-health
evidence, distinct architecture/security/Rust review, portable/Nix gates, and
green protected CI. Rollback inlines the private validator methods into the
single function without consumer, adapter, wire, or stored-data migration.

## Evidence

The issue, Discussion #399, ADR 0115, this change, focused registration corpus,
public/source inventories, code-health report, strict Rust/factory/Nix gates,
review, protected receipt, and published metrics are required evidence.
