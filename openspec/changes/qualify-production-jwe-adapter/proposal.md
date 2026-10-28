# Qualify the production JWE adapter

## Why

The SDK has a bounded, engine-neutral P-256 ECDH-ES/A128GCM port but no
production implementation. The earlier candidate review did not execute an
independent encrypt/decrypt oracle and did not have authoritative current
RustSec evidence. Shipping an engine without both would turn an intentionally
narrow security boundary into an unreviewed dependency decision.

## What changes

- Repeat exact-version qualification for current Rust JOSE candidates.
- Execute public-recipient encryption and private-recipient decryption outside
  the production workspace graph as independent interoperability evidence.
- Record host, WASM, iOS, Android, MSRV, dependency-cone, unsafe-code and
  advisory results.
- Adopt no production engine when any mandatory gate fails; retain the existing
  SDK-owned port and make the blocker and reconsideration triggers durable.
- Add a bounded reproduction route and update the JWE knowledge/evidence debt.

## Capability

### Modified capability

- `jose-jwe-encryption-port`: production adapters require exact, independently
  reproducible qualification and fail closed when any mandatory gate fails.

## Non-goals

No new JOSE algorithm, local cryptographic implementation, OID4VP DTO change,
HTTP, trust, consent, wallet authentication, publication, or downstream
consumer mutation.

## Delivery

Issue #538 owns this qualification. A reasoned deferral is a valid result; it
does not satisfy or erase the production-adapter debt. Issue #566 owns the later
adoption and must repeat qualification against the exact version it proposes to
ship.
