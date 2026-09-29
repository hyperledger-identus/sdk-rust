# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/444
Constraint blockers: none

## Existing entries affected

ADR 0115 continues to require semantic ownership and a touched-scope
code-health ratchet. `SDK-SEC-003`, format neutrality, value-free claim intent,
redaction, Rust/MSRV, dependency, support, target, publication, unsafe-code,
and spec-driven delivery constraints remain unchanged.

## Introduced or changed constraints

No product constraint changes. Internally, disclosure validation must retain
the exact phase and caller-order error priority while one private owner borrows
the request, candidate set, and selection slice. Construction remains public;
the validation context remains private.

## Introduced or changed limitations

None. The slice does not close or broaden format, query, proof, consent, trust,
policy, persistence, wire, or runtime limitations.

## Consumer and product impact

Consumers observe no API, accepted/rejected input, error, retained value,
ordering, resource budget, debug form, dependency, feature, MSRV, target,
performance-budget, or certification change.

## Activation and rollback

Activation requires exact-head preflight, pre-change precedence
characterization, signed/DCO commits, public/source comparison, code-health
evidence, distinct architecture/security/Rust review, portable/Nix gates, and
green protected CI. Rollback restores validation to the constructor without
consumer, adapter, or data migration.

## Evidence

The issue, Discussion #399, this change, focused presentation corpus, immutable
error golden, public/source inventories, code-health report, strict
Rust/factory/Nix gates, review, protected receipt, and published metrics are
required evidence.
