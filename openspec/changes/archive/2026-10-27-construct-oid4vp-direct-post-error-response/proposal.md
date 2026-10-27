# Construct bounded OID4VP direct-post error responses

## Why

`identus-oid4vp` now constructs a successful plain `direct_post` response from
validated request and DCQL lineage, but a wallet cannot construct the Final
Authorization Error Response through the same bounded, runtime-neutral API.
Leaving that form to callers invites misspelled protocol codes, detached state,
invalid developer text, unbounded output, and accidental disclosure of internal
SDK errors or holder information.

## What changes

- Add an SDK-owned closed enum for the RFC 6749 and OpenID4VP 1.0 Final
  authorization-error codes applicable to this protocol.
- Add bounded, redaction-safe optional developer description and URI-reference
  values with the exact RFC character grammar.
- Consume a validated authorization request into deterministic inert
  `application/x-www-form-urlencoded` error-response material.
- Preserve the validated response URI and return exact state only when it was
  present in the signed request.
- Keep protocol-error choice explicit: internal SDK/transport errors are never
  translated automatically and user-consent/privacy policy remains outside the
  primitive.

## Capabilities

### Added capabilities

- `oid4vp-direct-post-error-response`: bounded construction of the OpenID4VP
  1.0 Final plain direct-post Authorization Error Response.

### Modified capabilities

- `oid4vp-direct-post-response`: share its deterministic form-envelope and
  validated request-lineage boundary with the mutually exclusive error branch.

## Non-goals

No automatic SDK-error mapping, HTTP/DNS/TLS execution, retry policy,
`direct_post.jwt`, JWE, DC API, consent UI, verifier trust, credential
availability inference, presentation generation/verification, localization,
telemetry, persistence, consumer mutation, or certification claim.

## Delivery

Issue #532 owns this additive SDK-only successor under IDR-024. The unpublished
crate remains experimental. Protected responses and transport adapters require
separate decisions.
