# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/456
Constraint blockers: none

## Existing entries affected

ADR 0017 continues to define the bounded XML Schema 1.1 intersection and ADR
0115 continues to require semantic ownership and a touched-scope code-health
ratchet. `SDK-SEC-003`, chain neutrality, redaction, Rust/MSRV, dependency,
support, target, publication, unsafe-code, and spec-driven delivery constraints
remain unchanged.

## Introduced or changed constraints

No product constraint changes. Internally, one private lexical owner must
validate the exact bounded grammar before one private calendar owner validates
month/day/time semantics. Both must preserve one static failure projection and
must not expose parsed fields.

## Introduced or changed limitations

None. The slice does not support fractions, numeric offsets, leap seconds,
timezone lookup, normalization, arithmetic, or broader XML Schema values.

## Consumer and product impact

Consumers observe no API, accepted/rejected input, exact retained spelling,
error, Serde, metadata/query, allocation, dependency, feature, MSRV, target,
performance-budget, or certification change.

## Activation and rollback

Activation requires exact-head preflight, pre-change grammar characterization,
signed/DCO commits, public/source comparison, code-health evidence, distinct
architecture/security/Rust review, portable/Nix gates, and green protected CI.
Rollback inlines the private parsed value and calendar check into the validator
without consumer, adapter, wire, or stored-data migration.

## Evidence

The issue, Discussion #399, ADR 0017, this change, focused DID value corpus,
public/source inventories, code-health report, strict Rust/factory/Nix gates,
review, protected receipt, and published metrics are required evidence.
