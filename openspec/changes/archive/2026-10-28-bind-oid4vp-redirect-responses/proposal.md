# Bind OID4VP redirect responses and direct-post acknowledgement

## Why

`identus-oid4vp` currently owns request-bound `direct_post` and
`direct_post.jwt` construction, but the Final specification also permits OAuth
redirect response modes and defines an optional redirect returned after a
successful direct-post submission. Leaving these shapes to adapters duplicates
DCQL/state serialization and risks losing the verified request lineage.

## What changes

- Add inert, request-bound `fragment`, `query`, and `form_post` success/error
  material over the same DCQL and state rules as direct post.
- Require one exact bounded `redirect_uri` for redirect modes and reject
  `response_uri` or mixed destinations.
- Bind status, media type, bounded JSON, cache evidence, and optional absolute
  redirect URI returned by a direct-post response endpoint.
- Preserve an optional unique `response_code` as opaque verifier output; never
  claim entropy, redeem it, navigate, or perform HTTP.
- Add clean-room E2E, boundary, property, fuzz, benchmark, portable, and
  differential evidence plus adopter guidance.

## Modified capability

- `oid4vp-authorization-request-routing`: broaden the signed Final wallet-core
  routing state to the three standard OAuth redirect modes while preserving
  the existing direct-post modes and verifier authorization gate.

## Added capability

- `oid4vp-response-endpoint-acknowledgement`: consume sent direct-post material
  into a bounded optional post-submission redirect instruction.

## Non-goals

No browser/deep-link navigation, HTML rendering, HTTP execution, response-code
redemption, verifier storage, entropy proof, trust/consent policy, Digital
Credentials API, SIOPv2, or downstream mutation.

## Delivery

Issue #546 owns this SDK-only slice under #541 and IDR-024. The implementation
remains chain-agnostic, effect-neutral, portable, and unpublished.
