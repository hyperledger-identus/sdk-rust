# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/470
Constraint blockers: none

## Existing entries affected

ADR 0159 continues to define `identus-jose` as the small protocol-neutral JOSE
facade and to require separate evidence before any production JOSE dependency.
The JWS compact/header, JOSE error, dependency, redaction, unsafe-code,
Rust/MSRV, target, support, and spec-driven delivery constraints remain
unchanged.

## Introduced or changed constraints

No product constraint changes. Internally, the private collector must preserve
input-order errors during entry collection, then require `alg` before checking
exclusive key-reference correlation, and only then construct the raw header.

## Introduced or changed limitations

None. The slice does not close or broaden documented limits around algorithms,
JWE/JWT, trust, certificate paths, attestation, federation, profile policy, or
third-party JOSE engines.

## Consumer and product impact

Consumers observe no API, JSON, error, precedence, retained value, resource
budget, allocation class, dependency, feature, MSRV, target, performance, or
certification change.

## Activation and rollback

Activation requires exact-head preflight, pre-change combined-fault
characterization, signed/DCO commits, public/source comparison, code-health
evidence, distinct architecture/security/Rust review, portable/Nix gates, and
green protected CI. Rollback inlines the private member and field ownership into
the visitor without consumer, adapter, or data migration.

## Evidence

The issue, Discussion #399, this change, compact/header test corpus,
public/source inventories, code-health report, strict Rust/factory/Nix gates,
review, protected receipt, and published metrics are required evidence.
