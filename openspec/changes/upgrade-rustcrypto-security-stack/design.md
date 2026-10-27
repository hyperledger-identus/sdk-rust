# Design

## Dependency boundary

Update only the root workspace requirements and regenerated root/fuzz locks.
Adapt private `identus-crypto` implementation calls where RustCrypto 0.14
changed traits or encodings. Do not export upstream types or compatibility
shims.

## Compatibility proof

Treat the existing crypto suite and cross-language vectors as the behavioral
oracle. Add only regression assertions needed to pin a migration-sensitive
encoding or signature invariant. The HTTP resolver suite proves the Tokio
update without changing its abstraction.

## Failure policy

Any vector, portable-target, audit, dependency-policy or protected-CI failure
blocks delivery. No advisory ignore, feature broadening or old/new curve-stack
coexistence is an acceptable workaround.
