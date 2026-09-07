# Design: bounded OID4VCI Credential payload-error HTTP validation

## Context

OpenID4VCI 1.0 Final section 8.3.1.2 defines Credential Request payload
errors separately from RFC 6750 Authorization Error Responses. The payload
branch uses HTTP status 400 and `application/json`, requires its more specific
error parameters instead of generic `invalid_request`, and is never encrypted.
Its `Cache-Control: no-store` header appears only in a non-normative example.

The SDK already has a bounded, duplicate-safe `CredentialErrorResponseCore`
and one private RFC-shaped `application/json` field parser shared by the
Credential Nonce and immediate Credential Response boundaries. This slice
composes those reviewed pieces without adding transport execution, request
state, authorization semantics, or recovery policy.

Read-only Oxid at `5ba38b9bbc9326c294b353daaf2a074eca18c22f`
and Lace ID Portal at
`804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` remain consumer evidence only.
No source or fixture is copied or transformed.

## Goals / Non-Goals

**Goals:**

- require exact status 400 before inspecting remote media or body input;
- bound and validate the effective Content-Type before body parsing;
- reuse the existing body limits, parser, exact extension retention, and
  redaction properties;
- reject exact generic `invalid_request` only at the payload-error envelope;
- expose static bridge errors and preserve native/mobile/browser portability.

**Non-Goals:**

- RFC 6750 Authorization errors or `WWW-Authenticate` challenges;
- HTTP execution, endpoint provenance, TLS/network policy, request correlation,
  issuer truth, retries, blame, remediation, localization, or UI behavior;
- Cache-Control validation, because the Final makes no normative requirement
  for that header on this response;
- deferred polling, response encryption, credential verification/storage,
  chain extensions, FFI, consumer adoption, publication, release, or `main`.

## Decisions

### Compose body and media limits without retaining envelope data

`CredentialErrorHttpResponseLimits` combines the existing
`CredentialErrorResponseLimits` with a positive Content-Type maximum that
defaults to 1,024 bytes. `CredentialErrorResponseCore::parse_http_response`
checks status, then media size and grammar, then delegates to the existing body
parser. Success returns only `CredentialErrorResponseCore`; status and media
are not retained because they add no post-validation authority.

This is a caller-supplied envelope syntax check, not proof that an HTTP
exchange occurred or that a response came from the intended issuer.

### Keep Authorization errors and generic invalid_request outside

Only status 400 enters this payload-error parser. The exact code
`invalid_request` is rejected after bounded body parsing because section
8.3.1.2 requires its specific error parameters instead of the generic RFC
6750 value. The seven known Final codes retain their existing classification;
all other valid codes remain extension-compatible.

This narrow exclusion does not classify other extensions as authorization
errors and does not add retry, blame, or remediation meaning.

### Reuse the strict private application/json grammar

The existing helper accepts case-insensitive `application/json` with
RFC-shaped optional whitespace and parameters. It rejects combined values,
malformed quotes, injection bytes, absent values, and case-insensitive
duplicate parameter names. Reuse prevents drift between protocol endpoints.

Cache-Control is intentionally absent. Treating a header found only in an
example as mandatory would reject conforming implementations.

## Risks / Trade-offs

- **Exact 400 may reject non-conforming issuers** -> the type represents the
  Final payload-error contract; compatibility policy stays with consumers.
- **Extensions can carry unknown semantics** -> exact values survive only as
  `Extension`; the SDK assigns no recovery or display policy.
- **A caller can associate the response with the wrong request** -> the API
  does not claim correlation and carries no request-bound state.
- **Descriptions remain attacker-controlled** -> the body core keeps them in
  zeroizing storage behind an explicitly untrusted accessor and out of Debug.

## Migration Plan

Land as an additive unpublished API. Existing callers and body-only parsing
remain unchanged. Authorization Error Responses, transport orchestration, and
downstream adoption remain separate issue-first changes. Rollback is one
focused revert before publication or adoption.

## Open Questions

None. The normative status/media/generic-code boundary and example-only header
status are explicit in the pinned Final text.

## Provenance

- OpenID4VCI 1.0 Final section 8.3.1.2, retrieved 2026-09-07,
  `https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html`,
  HTML SHA-256
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
- Oxid read-only evidence:
  `MediaNoxLabs/oxid@5ba38b9bbc9326c294b353daaf2a074eca18c22f`,
  Apache-2.0; no source or fixture copied.
- Lace ID Portal read-only evidence:
  `input-output-hk/lace-id-portal@804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`;
  no source or fixture copied and no license grant relied upon.
