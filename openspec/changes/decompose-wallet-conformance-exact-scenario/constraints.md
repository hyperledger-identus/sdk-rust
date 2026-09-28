# Constraint readiness

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/438
Constraint blockers: none

## Existing entries affected

ADR 0115 and code-health governance continue to require semantic ownership,
characterization before refactoring, and a touched-scope ratchet without metric
gaming. Crate-layer, dependency, resource, redaction, runtime-neutrality,
support, target, publication, MSRV, and unsafe-code constraints are unchanged.

## Introduced or changed constraints

No effective product constraint changes. Internally, exact-record lifecycle
phases and aggregate evidence projection must have named private owners. Their
composition must retain the exact successful transcript, static failures, and
consumer trait surface.

## Introduced or changed limitations

None. The test kit still does not provide storage, encryption, custody,
executor, synchronization, cancellation, rollback, adapter durability, or a
production performance guarantee. Failure can still interrupt cleanup, so the
documented disposable-fixture requirement remains.

## Consumer and product impact

Consumers observe no public path, type identity, signature, trait bound,
operation sequence, result, error, diagnostic, dependency, feature, target,
runtime, or certification change. Maintainers gain smaller responsibility-
cohesive review surfaces and an exact regression transcript.

## Activation and rollback

Activation requires issue #438's signed/DCO PR, committed pre-change planning,
an issue-bound preimplementation receipt, characterization before production
edits, public/source and code-health comparison, focused/workspace/factory/
portable/Nix evidence, distinct local review, and green exact-head protected
CI. Rollback recombines private modules and phases without data or consumer
migration.

## Evidence

The issue, this change, canonical storage/code-health specifications, source
and public inventories, exact transcript and failure tests, strict Rust/factory
gates, exact-diff review, hosted receipt, and published metrics are required.
