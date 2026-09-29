# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/441
Constraint blockers: none

## Existing entries affected

ADR 0115 and code-health governance continue to require semantic ownership,
characterization before refactoring, and a touched-scope ratchet without metric
gaming. Crate-layer, dependency, input-resource, redaction, runtime-neutrality,
support, target, publication, MSRV, and unsafe-code constraints are unchanged.

## Introduced or changed constraints

No effective product constraint changes. Internally, external async list calls
remain in the scenario coordinator while bounded page/cursor/membership
evidence gains one private owner. The split must retain exact request order,
successful counts, fixture-derived bounds, and static failure projection.

## Introduced or changed limitations

None. The test kit still supplies no adapter, executor, storage, encryption,
cursor interpretation, ordering promise, durability proof, or production
performance guarantee.

## Consumer and product impact

Consumers observe no public path, type identity, signature, trait bound,
operation, count, result, error, diagnostic, dependency, feature, target,
runtime, certification, or publication change. Maintainers gain a separately
testable deterministic pagination-evidence boundary.

## Activation and rollback

Activation requires issue #441's signed/DCO delivery, committed planning, an
issue-bound preimplementation receipt, characterization before production
edits, public/source and code-health comparison, focused/workspace/factory/
portable/Nix evidence, distinct local review, and green exact-head protected
CI. Rollback restores the single private function without data or consumer
migration.

## Evidence

The issue, this change, canonical storage/code-health specifications, source
and public inventories, closed failure matrix, strict Rust/factory gates,
exact-diff review, hosted receipts, and published metrics are required.
