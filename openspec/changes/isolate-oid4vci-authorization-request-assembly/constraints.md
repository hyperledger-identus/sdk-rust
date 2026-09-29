# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/467
Constraint blockers: none

## Existing entries affected

ADRs 0083, 0102 and 0141 continue to define local strict form mechanics,
dependency disposition, and Authorization Request ownership. ADR 0115 continues
to require semantic ownership and a touched-scope code-health ratchet.
`SDK-SEC-003`, chain neutrality, redaction, Rust/MSRV, dependency, support,
target, publication, unsafe-code, and spec-driven delivery constraints remain
unchanged.

## Introduced or changed constraints

No product constraint changes. Internally, selected endpoint and existing-query
validation must complete before Authorization Details construction; both must
precede checked final sizing and exact rendering. The public method continues
to consume its validated predecessor exactly once.

## Introduced or changed limitations

None. The slice does not close or broaden PAR/JAR, scope/resource, browser,
HTTP, callback, token exchange, client authentication, trust, or other
documented limitations.

## Consumer and product impact

Consumers observe no API, accepted/rejected input, error, precedence, request
byte, parameter-order, resource budget, allocation class, zeroizing/debug form,
dependency, feature, MSRV, target, performance-budget, or certification change.

## Activation and rollback

Activation requires exact-head preflight, pre-change combined-fault
characterization, signed/DCO commits, public/source comparison, code-health
evidence, distinct architecture/security/Rust review, portable/Nix gates, and
green protected CI. Rollback inlines the private assembly operations without
consumer, adapter, or data migration.

## Evidence

The issue, Discussion #399, this change, focused Authorization Request corpus,
public/source inventories, code-health report, strict Rust/factory/Nix gates,
review, protected receipt, and published metrics are required evidence.
