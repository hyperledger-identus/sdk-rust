# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/452
Constraint blockers: none

## Existing entries affected

ADR 0014 continues to define the bounded generic dereferencer and ADR 0115
continues to require semantic ownership and a touched-scope code-health ratchet.
`SDK-SEC-003`, chain neutrality, redaction, Rust/MSRV, dependency, support,
target, publication, unsafe-code, and spec-driven delivery constraints remain
unchanged.

## Introduced or changed constraints

No product constraint changes. Internally, request preparation must retain the
exact phase ordering, decoded-name `BTreeMap` traversal, private failures,
resolver non-invocation, and prepared resolver inputs while one private owner
holds mutable preparation state. `PreparedRequest` remains private.

## Introduced or changed limitations

None. The slice does not close or broaden W3C at-risk behavior, method resource,
network retrieval, relative-reference, media, fragment, registry, cache,
transport, persistence, or trust limitations.

## Consumer and product impact

Consumers observe no API, accepted/rejected input, error, resolver call/input,
ordering, resource budget, debug form, dependency, feature, MSRV, target,
performance-budget, or certification change.

## Activation and rollback

Activation requires exact-head preflight, pre-change precedence and non-
invocation characterization, signed/DCO commits, public/source comparison,
code-health evidence, distinct architecture/security/Rust review, portable/Nix
gates, and green protected CI. Rollback inlines private state operations into
the constructor without consumer, adapter, or data migration.

## Evidence

The issue, Discussion #399, this change, focused DID dereferencing corpus,
public/source inventories, code-health report, strict Rust/factory/Nix gates,
review, protected receipt, and published metrics are required evidence.
