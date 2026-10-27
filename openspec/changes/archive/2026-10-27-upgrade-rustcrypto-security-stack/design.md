# Design

## Dependency boundary

Update only the root workspace requirements and regenerated root/fuzz locks.
Adapt private `identus-crypto` implementation calls where RustCrypto 0.14
changed traits or encodings. Four release-candidate tuple fields already expose
`k256`/`p256` key types. Retaining 0.13 aliases would retain the rejected
dependency closure, so the next candidate records a narrow source migration to
the corresponding 0.14 types. No additional upstream type is exported.

## Compatibility proof

Treat the existing crypto suite and cross-language vectors as the behavioral
oracle. Add only regression assertions needed to pin a migration-sensitive
encoding or signature invariant. The HTTP resolver suite proves the Tokio
update without changing its abstraction.

The source migration is separately proven by the `0.1.0-rc.1` public API
snapshot, a consumer-change ledger record and a release migration note. SDK
constructors, encoders and verification methods remain the preferred stable
boundary; direct tuple-field consumers update their RustCrypto dependency and
type imports to 0.14 for the next candidate.

## Failure policy

Any vector, portable-target, audit, dependency-policy or protected-CI failure
blocks delivery. No advisory ignore, feature broadening or old/new curve-stack
coexistence is an acceptable workaround.
