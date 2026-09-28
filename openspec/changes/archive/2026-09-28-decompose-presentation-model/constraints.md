# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/407
Constraint blockers: none

## Existing entries affected

`SDK-SEC-003` continues to require explicit resource limits. ADR 0115
continues to require semantic ownership and a touched-scope code-health
ratchet. Format-neutrality, value-free claim intent, redaction, Rust/MSRV,
dependency, support, target, publication, and unsafe-code constraints are
unchanged.

## Introduced or changed constraints

No effective product constraint changes. Internally, scalar/request vocabulary,
candidate/disclosure validation, and generated artifact/receipt behavior must
have cohesive private owners. Sibling access must remain the minimum needed to
preserve exact request binding and validated-state projection.

## Introduced or changed limitations

None. This refactor does not close or broaden format, protocol, discovery,
ranking, consent, trust, persistence, proof, wire, or runtime limitations.

## Consumer and product impact

Consumers observe no API, type identity, accepted/rejected input, error,
resource budget, retained value, debug form, dependency, feature, MSRV, target,
performance-budget, or certification change. Maintainers gain cohesive review
surfaces below the attention threshold.

## Activation and rollback

Activation requires rebasing onto the merged `develop` tip, issue #407's
exact-head preflight, signed/DCO PR, pre/post characterization, public/source
comparison, error and code-health evidence, distinct review, relevant
portable/Nix gates, and green protected CI. Rollback recombines private modules
without consumer, adapter, or data migration.

## Evidence

The issue, Discussion #399, this OpenSpec change, presentation test corpus,
public export inventory, immutable error golden, source distribution,
code-health report, strict Rust/factory/Nix gates, exact-diff review, and hosted
exact-head receipt are required evidence.
