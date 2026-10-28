# Validate OID4VP Request Object security evidence

## Why

`identus-oid4vp` verifies a bounded signed Request Object and correlates its
`client_id` and optional `wallet_nonce`, but it currently exposes typed routing
before validating the OpenID4VP audience or JWT freshness and replay evidence.
That leaves every adopter to rebuild security-sensitive Final and RFC 7519
logic, with no common clock or atomic replay boundary.

## What changes

- Capture bounded `aud`, `iat`, optional `exp`, `nbf`, and `jti` claims while
  scanning the already verified payload.
- Add an SDK-owned security policy with an explicit discovery audience,
  maximum age, and allowed clock skew.
- Consume `VerifiedRequestObject` through a caller-owned `WallClock` and atomic
  replay-decision port before exposing DCQL or authorization-routing successors.
- Preserve exact client-id and wallet-nonce correlation, redacted diagnostics,
  finite parser work, and runtime/target neutrality.
- Add clean-room conformance, property, fuzz, benchmark, E2E, and portability
  evidence without adopting another JWT framework.

## Modified capability

- `oid4vp-request-object-jar`: add the security-evidence transition required
  after cryptographic verification and before typed request routing.

## Non-goals

No verifier trust store, key authorization, persistence, transport, system
clock, discovery fetch, consent/UI, proof generation, credential verification,
Digital Credentials API, SIOPv2, or consumer DTO/error compatibility.

## Delivery

Issue #544 owns this SDK-only slice under parent #541. Verifier-key trust stays
with #545. The pre-release source transition is declared in the consumer change
ledger and must be evidenced before activation.
