# ADR 0058: parse a bounded OID4VCI Credential Error Response core

- **Status:** Accepted for implementation
- **Date:** 2026-09-07
- **Decision authority:** standing product mandate and IDR-023 issue #145
- **Related work:** issues #7, #20, #129, #143, and #145

## Context

The SDK validates an immediate successful Credential Response but has no
reusable Credential Endpoint payload-error boundary. OpenID4VCI 1.0 Final
defines a required error code, an optional developer description, and seven
codes implementations should use. The SHOULD-level registry must remain open
to extensions, while all remote strings and unknown JSON remain hostile input.

## Decision

1. Add a partial `CredentialErrorResponseCore` rather than an HTTP,
   authentication, or recovery state.
2. Preserve the exact bounded code in `CredentialEndpointErrorCode` and
   classify the seven Final values through `CredentialEndpointErrorKind`, with
   every other valid code represented as `Extension`.
3. Validate the optional description under its own bound and expose it only as
   explicitly untrusted remote developer information.
4. Structurally validate and discard unknown fields, including legacy
   `c_nonce` and foreign `error_uri` names; retain no raw response.
5. Zeroize retained strings and keep Debug/errors free of remote content.
6. Preserve the existing dependency, feature, target, and unsafe-code cone.

## Consequences

- Headless consumers can distinguish standardized Credential Endpoint payload
  failures without losing extension interoperability.
- Remote text cannot become trusted UI or recovery policy through the SDK
  surface.
- Parsing cannot claim HTTP validity, authentication semantics, request
  correlation, issuer truth, retryability, blame, or remediation.
- A later transport slice can bind the body core to exact status/media rules
  while keeping RFC 6750 Authorization errors separate.
