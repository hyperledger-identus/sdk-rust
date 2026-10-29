# ADR 0195: version the JWE error contract independently

- **Status:** Accepted for issue #539 implementation
- **Date:** 2026-10-29
- **Decision authority:** issue #539 under ADRs 0004, 0118, and 0181
- **Related:** ADRs 0118, 0137, 0181, and 0192
- **Applies to:** `identus-jose` only

## Context

`JoseError` has an immutable historical golden and an append-only live suffix.
The later engine-neutral JWE port introduced a separate public non-exhaustive
`JweEncryptionError` enum with eight categories. Its codes and messages are
currently checked by focused tests, but they are not part of independently
versioned compatibility evidence.

Appending JWE rows to the `JoseError` fixture would couple two public enums,
their ordering, and their release evolution. Rewriting the existing golden
would also let new implementation state redefine historical evidence.

## Decision

1. Keep the `JoseError` v1/v2 fixtures and their append-only policy unchanged.
2. Give `JweEncryptionError` an independent immutable v1 golden captured from
   `develop@c66d506690d3cd6f119f9ae6430b285f98115d10`.
3. Pin all eight ordered variants, stable codes, kinds, capability, exact local
   and public messages, full display, empty source behavior, and the deliberate
   absence of public code constants.
4. Retain one wildcard-free mapping from every JWE variant to its private
   compile-time contract. Tests validate the same complete inventory against
   the independent golden.
5. Future additive JWE errors append after the v1 ordered prefix and receive
   feature-owned tests. They do not rewrite the v1 fixture.
6. Extend the existing immutable-golden factory mechanism with a bounded JWE
   binding and exact preimplementation blob identity.
7. Add no production JWE engine, dependency, feature, algorithm, OID4VP wire
   behavior, public constant, serialization, FFI, or downstream mutation.

## Consequences

JWS/JOSE and JWE compatibility can evolve without sharing an artificial enum
or fixture version. Existing JWE codes become durable release evidence without
expanding the public API. A variant cannot silently inherit a mapping, and
redaction or ordering drift fails before release.

The fixture intentionally duplicates static behavior. It is an independent
oracle, not a generated source of truth. A deliberate breaking change requires
a later compatibility decision and a new versioned contract.

## Alternatives rejected

Appending to the `JoseError` golden couples unrelated enum order and makes the
fixture name misleading. Rewriting an existing golden destroys its historical
independence. Publishing eight code constants expands API without a consumer
need. Relying only on ad hoc assertions lacks immutable provenance and factory
enforcement.

## Verification and rollback

Verification requires exact fixture hash/blob identity, eight unique ordered
rows, wildcard-free mapping, stable-code uniqueness, redaction/source checks,
unchanged public API/dependencies, focused JOSE tests, factory mutation tests,
workspace quality gates, independent CLI review, and protected CI. Rollback
removes only the additive compatibility evidence and private contract helper;
runtime and wire behavior remain unchanged.
