# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/473
Constraint blockers: none

## Existing entries affected

The DID Resolution HTTP, DID query-option, input-resource, error/redaction,
dependency, unsafe-code, Rust/MSRV, target, support, and spec-driven delivery
constraints remain unchanged. Issue #456 retains ownership of DID URL
dereferencing request preparation; this slice is limited to the separate DID
Resolution HTTP adapter.

## Introduced or changed constraints

No product constraint changes. Internally, collection must remain source-order,
decode name before value, reject structural/control/duplicate failures before
known-option conversion, reject the version pair before final construction,
and complete before resolver invocation.

## Introduced or changed limitations

None. This slice does not add POST, DID URL dereferencing, server/runtime,
middleware, authorization, generic form parsing, new query options, or support
for query-controlled `accept`.

## Consumer and product impact

Consumers observe no API, HTTP, W3C error/status, query, representation,
resolver-call, retained value, resource budget, allocation class, dependency,
feature, MSRV, target, performance, or certification change.

## Activation and rollback

Activation requires exact-head preflight, pre-change combined-fault
characterization, signed/DCO commits, public/source comparison, code-health
evidence, distinct architecture/security/Rust review, portable/Nix gates, and
green protected CI. Rollback inlines the private query-field ownership into the
coordinator without consumer, resolver, adapter, or data migration.

## Evidence

The issue, Discussion #399, this change, DID Resolution HTTP tests, public/source
inventories, code-health report, strict Rust/factory/Nix gates, review,
protected receipt, and published metrics are required evidence.
