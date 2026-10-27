# Add a bounded SIOPv2 protocol core

## Why

Oxid implements a useful self-issued DID authentication journey, but its
protocol parsing, signing bridge, in-process verifier and application lifecycle
are still product-owned. SDK-Rust already owns bounded DID values, JOSE signing
capabilities and reusable OpenID request retrieval. A separate
`identus-siopv2` crate can crystallize the standards-owned behavior without
copying Oxid orchestration or confusing SIOPv2 authentication with OpenID4VP
credential presentation.

SIOPv2 is not Final. The OpenID Foundation currently publishes draft 13 as the
editor draft while the AB/Connect page lists its first Implementer's Draft.
The SDK must therefore pin one profile explicitly and avoid a broad or stable
compatibility claim.

## What changes

- Activate `identus-openid4vc` only for bounded request-by-reference retrieval
  primitives shared by OID4VP and SIOPv2.
- Add an unpublished `identus-siopv2` crate pinned to draft 13 with a narrow
  signed-request, DID-subject, `subject_signed_id_token`, `direct_post` profile.
- Validate request signature lineage, client/audience, HTTPS redirect, nonce,
  state, time, subject syntax and algorithm negotiation under explicit limits.
- Construct a self-issued ID Token signing input through `identus-jose` with
  `iss == sub`, exact audience/nonce/time and a DID URL `kid`.
- Add a pure deterministic flow state machine for approval, refusal,
  cancellation, timeout, response readiness, accepted delivery and unknown
  delivery outcome. Caller-supplied event identities make retries idempotent.
- Add clean-room and attributed oracle vectors, E2E, negative, property, fuzz,
  benchmark and portable-target evidence plus an Oxid migration note.

## Capabilities

### Added capabilities

- `openid4vc-request-retrieval`: bounded runtime-neutral request-by-reference
  retrieval and exact response binding shared without protocol policy.
- `siopv2-did-authentication`: draft-13 DID-subject authentication request,
  self-issued ID Token response and deterministic lifecycle core.

### Modified capabilities

- `oid4vp-request-retrieval`: preserve the existing public contract while its
  shared mechanics delegate to `identus-openid4vc`.

## Non-goals

No browser/deep-link execution, HTTP client, DNS/TLS/redirect policy, verifier
trust, DID resolution, custody, UI, product consent semantics, persistence,
telemetry, OpenID4VP presentation, JWK-thumbprint subject, attester-signed ID
Token, authorization-code flow, dynamic OP discovery, federation, arbitrary
self-attested claims, consumer mutation, publication or certification claim.

## Delivery

Issue #514 owns this additive pre-release slice under IDR-031. Issue #541 owns
the remaining IDR-024 completion matrix. Oxid remains unchanged; its future
adapter may translate product lifecycle and the historical loopback fixture to
this protocol core only after a separate adoption issue.
