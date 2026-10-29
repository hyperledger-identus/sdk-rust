# Version the JWE error contract

## Why

The engine-neutral compact-JWE port exposes eight stable public error
categories, but its compatibility evidence is limited to focused assertions.
Before a production adapter or release can rely on those categories, the SDK
needs an immutable, exhaustive, independently versioned contract.

## What changes

- Decide that `JweEncryptionError` owns a separate v1 golden rather than
  extending or rewriting the historical `JoseError` golden.
- Capture the exact existing ordered codes, kinds, capability, messages,
  displays, source behavior, and lack of public code constants.
- Bind the planning golden to exact-base preimplementation evidence and the
  existing fail-closed factory checker.
- Replace the repeated mapping tuple with a private compile-time record while
  preserving every public and runtime behavior.
- Add focused contract, append-only, redaction, and compatibility evidence.

## Capability

### Modified capability

- `jose-error-contracts`: independently version the JWE error family while
  preserving the existing JOSE family and its historical fixture.

## Non-goals

No production JWE engine, cryptographic implementation, algorithm expansion,
OID4VP wire change, public code constants, dependency, feature, FFI, transport,
consumer repository, publication, or release activation.

## Delivery

Issue #539 owns this behavior-neutral hardening slice. Production engine
selection remains with #566 under ADR 0192.
