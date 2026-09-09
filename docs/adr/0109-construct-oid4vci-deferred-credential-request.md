# ADR 0109: construct the OID4VCI Deferred Credential Request

- **Status:** Accepted for implementation
- **Date:** 2026-09-09
- **Decision authority:** standing product mandate and IDR-023 issue #250
- **Related work:** issues #7, #149, #241, and #250
- **Normative baseline:** OpenID4VCI 1.0 Final section 9.1

## Context

The SDK owns a bounded opaque transaction handle from a deferred Credential
Response and can discover the optional validated Deferred Credential Endpoint.
It does not bind those values or construct the mandatory unencrypted request,
leaving each headless consumer to reproduce sensitive JSON escaping, endpoint
presence and transport metadata.

## Decision

1. Construct a distinct non-Clone request from an existing deferred response
   and matched issuer metadata that advertises the endpoint.
2. Own the exact endpoint and deterministic JSON body; report POST,
   `application/json`, and the access-token requirement without retaining a
   token or executing transport.
3. Introduce a positive independent complete-body limit, defaulting to 16 KiB.
4. Reuse `serde_json` scalar serialization through a bounded writer so output
   is standards-correct and cannot allocate an unbounded encoded intermediate.
5. Keep the body in zeroizing byte storage and expose it only through an
   explicitly sensitive accessor. Debug and errors remain content-free.
6. Permit repeated structural construction while leaving interval, retry,
   replay, terminal invalidation and response correlation to later layers.

## Consequences

- Headless consumers receive one reusable Final request shape instead of
  hand-writing JSON and endpoint checks.
- Custom response limits cannot silently become unbounded request output.
- Existing response and metadata APIs remain compatible.
- No dependency, feature, unsafe, runtime or target surface is added.
- HTTP, TLS, bearer-token lifecycle, encryption and orchestration remain absent.

## Rejected alternatives

- Generic Serde on the public request would widen accidental logging and
  persistence paths for bearer-adjacent state.
- Hand-written JSON escaping duplicates a mature existing dependency and raises
  correctness risk.
- Storing the access token would combine independent secret and transport
  lifecycles.
- Consuming the response would prevent legitimate reconstruction for later
  polling without providing replay or invalidation guarantees.

## Rollback

Remove the additive request module, limit, transition, errors, tests and
canonical capability. Existing metadata and deferred response users remain
source and wire compatible.
